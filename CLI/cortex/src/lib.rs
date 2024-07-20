use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use nanoid::{alphabet, nanoid};


fn rand_id() {
    let alphabet: [char; 2] = ['0', '1'];
    nanoid!(8, &alphabet);
}

#[derive(Debug, Parser, Serialize, Deserialize)]
#[command(author, version, about)]
pub struct CortexArgs {

    #[command(subcommand)]
    pub cmd: CortexCommands,


    /// The name for or id of the job. Defaults to a random value
    #[arg(default_value = rand_id())]
    pub id: String,

    /// The number of hosts for the job to run on
    #[arg(short, long)]
    pub redundancy: i32,

    /// The path to the dockerfile
    #[arg(short, long, default_value = ".")]
    pub container: String,

    /// The path to the volume to be mounted into the container. If none is provided, one will be created
    #[arg(short, long)]
    pub volume: Option<String>,

    /// The number of GPUS on the host. A range is also accepted
    #[arg(short, long, default_value = "1")]
    pub gpus: String,

    /// GPU make, i.e Nvidia.
    #[arg(short, long, default_value = "any")]
    pub make: String,

    /// GPU model. The "make" argument is not necessary if "model" is specified
    #[arg(long, default_value = "any")]
    pub model: String,

    /// The path to where you would like the results of the job to be placed
    #[arg(short, long, default_value = ".")]
    pub out: String,

    /// An upper bound on how long this job should run for
    #[arg(short, long, default_value = "none")]
    pub max_runtime: String
}


#[derive(Subcommand, Debug, Clone, Serialize, Deserialize)]
pub enum CortexCommands {
    Run,
    /// Will check to see if the job can be run without actually running it
    Test,
    /// List currently running cortex jobs
    Ps
}

pub enum JobState {
    INIT,
    PREPARING, // verifying docker image security, searching for hosts
    WAITING, // verified, waiting for a host to become available
    RUNNING, // job is currently running on hosts
    FAIL,
    DONE
}
