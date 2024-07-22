use clap::{Args, Parser, Subcommand};
use serde::{Serialize, Deserialize};
use nanoid::nanoid;
use std::path::PathBuf;


fn rand_id() -> String {
    let alphabet: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
    nanoid!(8, &alphabet)
}

fn default_path(path: &str) -> PathBuf {
    PathBuf::from(String::from(path))
}

#[derive(Debug, Parser)]
#[command(author, version, about, name = "cortex")]
pub struct AllArgs {

    #[command(flatten)]
    pub cortex_args: CortexArgs,

    #[command(flatten)]
    pub path_args: PathArgs
}

#[derive(Debug, Parser, Clone, Serialize, Deserialize)]
pub struct CortexArgs {

    #[command(subcommand)]
    pub cmd: CortexCommands,

    /// The name for or id of the job. Defaults to a random value
    #[arg(long, default_value = rand_id(), hide_default_value = true)]
    pub id: String,

    /// The number of hosts for the job to run on
    #[arg(short, long)]
    pub redundancy: i32,

    /// The number of GPUS on the host. A range is also accepted
    #[arg(short, long, default_value = "1")]
    pub gpus: String,

    /// GPU make, i.e Nvidia.
    #[arg(short, long, default_value = "any", group = "gpu_arg")]
    pub make: String,

    /// GPU model. The "make" argument is not necessary if "model" is specified
    #[arg(long, default_value = "any", group = "gpu_arg")]
    pub model: String,

    /// An upper bound on how long this job should run for. Can be specified as "x minutes" or "x hours"
    #[arg(long, default_value = "none")]
    pub max_runtime: String
}

#[derive(Parser, Debug)]
pub struct PathArgs {
    /// The path to the dockerfile
    #[arg(short, long, default_value = default_path(".").into_os_string())]
    pub container: PathBuf,

    /// The path to the volume to be mounted into the container. If none is provided, one will be created
    #[arg(short, long)]
    pub volume: Option<PathBuf>,

    /// The path to where you would like the results of the job to be placed
    #[arg(short, long, default_value = default_path(".").into_os_string())]
    pub out: PathBuf,
}


#[derive(Subcommand, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CortexCommands {
    Run,
    /// Will check to see if the job can be run without actually running it
    Check,
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
