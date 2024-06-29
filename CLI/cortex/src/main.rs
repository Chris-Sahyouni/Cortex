// use std::{process};
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct CortexArgs {

    #[command(subcommand)]
    cmd: CortexCommands,

    /// The number of hosts for the job to run on
    #[arg(short, long)]
    redundancy: i32,

    /// The path to the dockerfile
    #[arg(short, long, default_value = ".")]
    container: String,

    /// The path to the volume to be mounted into the container. If none is provided, one will be created
    #[arg(short, long)]
    volume: Option<String>,

    /// The number of GPUS on the host. A range is also accepted
    #[arg(short, long, default_value = "1")]
    gpus: String,

    /// GPU make, i.e Nvidia.
    #[arg(short, long, default_value = "any")]
    make: String,

    /// GPU model. The "make" argument is not necessary if "model" is specified
    #[arg(long, default_value = "any")]
    model: String,

    /// The path to where you would like the results of the job to be placed
    #[arg(short, long, default_value = ".")]
    out: String
}



#[derive(Subcommand, Debug, Clone)]
enum CortexCommands {
    Run,
    Test
}




fn main() {

    let args: CortexArgs = CortexArgs::parse();

    println!("Done");

    // if let Err(e) = run(config) {
    //     println!("an error during execution here");
    //     process::exit(1);
    // }

}
