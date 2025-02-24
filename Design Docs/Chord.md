# Chord

### Chord details:
 - nodes will maintain 4 successors and 4 predecessors to start although this number is subject to change. The system MUST be programmed so that this number can be easily changed.
 - SHA-256 for consistent hashing. The paper recommends SHA-1 but that is not collision resistant. It's not a huge deal but why not just be safe.

### Key assignment
For key k, k is assigned to either:
- The node with id equal to H(k)
- the node with next highest id if no node has that exact id

### Node joins
When a new node joins it will obtain a successor and predecessor (actually several of each but that is not important here). Its newly given successor will hand over some of its keys to the new node.

### Node leaves
When a node leaves the network all of its keys are given to its successor.

### Finger Tables
finger_table[i] = first node with id >= 2^i + k mod 2^m, where ki is that particulars node's id


### Stabilization

(There's pseudocode for all this in the paper)

Stabilization is to be run periodically at each node
Two nodes a, and a's succesor b
1. node a asks b for its predecessor p
2. node a now has the opportunity to change its successor to p if it needs to
3. node b now has the opportunity to change its predecessor to a if it should

Every node should refresh its finger table periodically

Every node should check its predecessor periodically. If its failed, it waits until it is contacted by another in notify()

