
Final design decisions and more specifics will go in the individul docs

# Vision
Cortex is a peer-to-peer system. The end goal is to create a general purpose P2P system, but that is really phase 2. Phase 1 is just to create a useful peer-to-peer system to incentivize contributors into opting into the network. Once enough contributors have opted in to make the system actually useful, then we can transition to phase 2. In phase 2, Cortex is really infrastructure and not software. At this point we expose an API to our network of collected nodes so that developers can easily build their own distributed applications on top of our infrastructure. The idea here being that to build a P2P system like Bitcoin or Bittorent, a key obstacle to success is will anyone actually opt in? Cortex solves this problem for future developers by solving it first for ourselves. We build a useful P2P system to begin with, then let any developer use the network we have created. Our product is basically our solution to this problem and perhaps software packages to streamline the process.

Phase 2 though is far away and the initial hurdle is to design a useful P2P system.

# Design
The initial design could go in one of two directions (or both): the serverless architecture, or clusters. The serverless architecture meaning a function execution service where users define events and triggers and don't have to worry about anything else and we take care of executing the functions on a given trigger. Clusters meaning we try to make Cortex compatible with cluster managers like YARN so users can deploy apps like Apache Spark on our infrastructure. The advantage of serverless is that fault-tolerance is an inherently easier problem to solve in that setting because each node is stateless. For clusters on the other hand, all that really needs to be done is expose an API compatible with common cluster managers and have a service for allocating nodes to each cluster.

One idea could be have to static (or maybe call them safe) nodes and volatile nodes. Static nodes being ones that are guaranteed to have a certain amount of uptime (maybe even just measure their uptime instead of asking outright) and volatile nodes being ones with no guarantees on their uptime.

On to more concrete design issues

## P2P Architecture
Chord is probably the best choice here so to start with we'll just go with that. This does mean though that a stabilization algorithm will be necessary periodically.

## Failure Detection
There are a few options here. The easiest one is probably basic push-gossip heartbeating. The advantage here being simplicity and ease of message piggybacking. The other option is SWIM. SWIM could still work with piggybacking on the heartbeats but it will take more rounds of heartbeating to detect a failure and is more complex to implement.

Nodes should periodically log non-security-sensitive information to disk such as neighbors so that on recovering from a failure, they can re-join without having to contact an introducer if the timestamp on the log is fresh enough.

## Virtualization
This is a pretty critical piece to get right. Contributors need to be protected from malicious users trying to execute code on their machines. Users need to be protected from malicious contributors who might want to spy on their activity (this is the tricky one). Users need to be able to rely on a standard execution environment and not have to worry about the variety of OS's and architectures contributors have.

Some options to consider here are QEMU, Firecracker, Docker, or doing it yourself.
More options: libcontainer, look at bocker (so you can basically clone docker yourself), webassembly, LXC

## Replication
For function execution this shouldn't be too complicated, just replicate the execution on multiple nodes. An interesting idea to consider though is whether or not you take the first response that comes back from the replicas or wait for all the responses within a reasonable window of time and make some decision after seeing them all. The advantage of this second one is that it may help with security.

## Payment
There are two options here: pay for execution, and pay for uptime
Paying for execution rewards contributors somewhat arbitrarily for actual usage of their hardware. It would require logging information about each function executed on a blockchain in a tamper proof way.
Paying for uptime is simpler because it does not require logging executions and it means everyone gets paid fairly.

Decision made: we'll pay for uptime. This more directly incentivizes contributors to have Cortex running. Each contributor's payment will simply be their nodes uptime as a fraction of the total uptime of all nodes for the pay period.

This still requires tamper-proofing.
If we pay per heartbeat, contributors could try to spoof extra heartbeats. One way to fix this could be on the receiving end to set a cap at how many heartbeats can be received from each neighbor (i.e set a wait period between receiving beats).

Either way, a blockchain will be needed track payments. I think a simple design for this could be to just have nodes add blocks in order around the chord as blocks become full

SEE SECURITY 3 FOR A BETTER IDEA

also theres probably going to have to actually be separate payment servers unfortunately to convert uptime into real money.

## Event Triggers
Users will obviously want to define themselves when events will be triggered so it will be necessary to create Python, NPM, etc modules to make this possible. (Also it should be possible to trigger events both synchronously or asynchronously)

Right now there are two ways I can imagine outside devices contacting the network to trigger an event. One is for every trigger, an introducer node is contacted via DNS, the trigger is sent to the introducer and then routed to the target node. Another is that upon instantiating the Cortex object in whatever module is doing this, an introducer is contacted, or maybe several known introducers are saved to disk. The benefit of this is that it removes the need for an entire DNS resolution every time an event is triggered. However, it introduces the possibility that the known introducers will be down when an event needs to be triggered and the DNS resolution will be required anyway. The second way is probably better because it is essentially the first way but with an added cache of known introducers that we will attempt to fill upon instantiating the Cortex object.

To actually trigger the event there are basically two options: HTTP and a custom network protocol. Its probably better to just use the custom protocol that will be used by the network itself.

Events will need to be given names. Users can then trigger an event by name obviously instead of having to know the location of the node in the chord that is responible for it. The name should be hashed using our consistent hashing function to decide which node will execute it. That way the node responsible for running it can be resolved by the introducer.

There's also the issue of responding to the request. The device triggering it will not be directly communicating with the node executing the event. The fastest option is to have the executing node send the result straight back to the caller. The problem though is this would require opening up a second network connection between the caller and the executor node. This isn't necessarily a huge problem but poses a security risk since an attacker could try and spoof being the executor node. The slower but safer option is to use the introducer as an intermediary. The introducer could then sign the result given to it by the executor node.

There will also need to be a queueing mechanism at each node for events that have been triggered. Alternatively, if a node is busy when another trigger arrives at it, just send it to a replica since the replica should be nearby in the chord. This does mean that nodes should have knowledge of which keys are at their neighbors.

## Event Execution
Some events will require runtimes (probably most), others will not. So the execution environment must include any necessary runtimes. This shouldn't be too hard though since this is standard for any virtualization really.

(It will be easier to flesh out this section once we have decided on a virtualization solution)

## Piggybacking
Right now I'm thinking that EVERY message in the entire system should be piggybacked. Basically there will be no communication that doesn't happen piggybacked on top of hearbeats. This should just simplify everything in terms of programming and really reduce the overhead on the network since it means every node can always rely on having a fixed number of TCP connections.

Messages should be formatted as basically JSON, YAML, or some other equivalent format. There will be different sections for different things like failure detection, churn, routing triggers, etc. A dispatcher on each node can then take each part of the message and route them locally to threads responsible for handling different parts.

Even though I said right above this that everything would be piggybacked, I am wondering if this isn't the best idea. It would definitely make everything easier to reason about, but it could seriously slow down the entire network because then everything is bounded by the pace of heartbeats. One way to get around this could be to just have really fast heartbeating. Another way could be to piggyback everything except event triggering since that is the critical piece.

Probably the best way to go about this is to piggyback everything except for triggers.
Actually new idea, separate things into two categories basically, latency-sensitive and non latency-sensitive then piggyback all non-latency-sensitive messages

## Security
For this we'll just go through all the different scenarios I can think of and try to address them one by one.

(brief not that in general all communication should be done over TLS, also it would not be a bad idea for every node to have a certificate)

### 1. Malicious user tries to attack contributor
If our virtualization doesn't solve this on its own, we need different virtualization. This one should be pretty easy to handle.

### 2. Malicious node tries to spy on/interfere with user
The first way to mitigate this is to just again try to have as robust of a virtualization solution as possible so that its difficult for the attacker to interfere with the VM/container. There are some other things to consider though"

You could try something along the lines of across replicas, have a coordinator pick a random number of clock cycles and a random address. (This is assuming we use a VM which would therefore have a virtual system clock and not a container in which case I'm not sure it would have a system clock), then require each replica to hash x number of bytes at that address of memory after the chosen number of clock cycles. Assuming none of them are malicious, they should all reply with the same hash. If any of them are different that could indicate interference. There would also have to be no randomness in the event code. Each event would have to be fully deterministic.

A watchdog process could also work, but it would have to exist on another node which could be challenging.

Some general ideas to think about are splitting things up across multiple nodes so no one node can see the entire event, encrypt whatever you can

other ideas: encrypt data-at-rest, use a runtime secrets manager, secure enclaves (requires hardware support), encrypt the VM/container image until runtime (like maybe send the key with the trigger)

no matter what, whether its a container or VM, do not mount files into it, do not share ANY host resources with the isolated environment.

look into TPMs

### 3. Node tries to artificially increase its uptime
The key here is that nodes should not record their own uptime, their neighbors should. Even so though, just because a node cannot lie about its own uptime does not mean that it cannot lie about other nodes' uptimes (even though this is certainly less likely). This also brings up the issue that neighbors will disagree on shared neighbors' uptimes.

A better idea is to have nodes record actual time intervals instead of just amounts of time. So instead of recording how long a node was up for, record when it came online and when it went offline. Because nodes will always be heartbeating to some neighbor if they are alive, we can be sure that every interval will be recorded by some node at some time. Then when payment time comes, the intervals can be aggregated for each node and the total time alive can be summed up. Its worth noting that it is totally possible for individual nodes to disagree about another's uptime, but across the whole network, all the intervals of time where a node was alive will be captured.

This does not necessarily solve the problem of nodes lying about each other's uptimes. We'll come back to this one later though because it is unlikely.

### 4. Node tries to spoof the return value of an event
digital signatures should help?

also you could have the introducer node that will give the return value back to the caller initiate the TCP connection with the executor node. That way the executor node *responds* to the introducer rather than contacting it. That way we rely on the built in security of TCP (TLS really) meaning an attacker would have to guess random starting sequence numbers. This should pretty much fix this issue.

### 5. Nodes try to intercept trigger parameters or event return values
encrypt them somehow?

The simple answer here is don't include the parameters when routing to find the executor node. Rather, first have the introducer find the executor node, then contact it directly and send it the parameters. Then for the return values they will also be sent directly to the introducer responsible and forwarded to the caller.

This still poses the issue of what if the introducer is the malicious one. One option could be to do a key exchange between the caller and the executor, but that doesn't fix anything unless you sign one of the exchanged keys. The other option could be to have the executor node and the caller communicate directly. This would involve having the caller contact the executor node. This is probably the way to go.

