use std::{error::Error, net::TcpStream, io::prelude::*, path::{Path, PathBuf}};
use std::process;
use clap::Parser;

use cortex::{CortexArgs, CortexCommands, JobState};

const SERVER_IP_ADDR: &str = "127.0.0.1:4444";


fn main() {

    let args: CortexArgs = CortexArgs::parse();

    match args.cmd {
        CortexCommands::Run => {
            if let Err(e) = run(args) {
                println!("Error from run: {}", e);
                process::exit(1);
            }
        }
        CortexCommands::Test => {
            if let Err(e) = test(args) {
                println!("Error from test: {}", e);
                process::exit(1);
            }
        }
        CortexCommands::Ps => {
            ps();
        }
    }

}


fn run(args: CortexArgs) -> Result<(), Box<dyn Error>> {

    updateJobState(&args.id, JobState::INIT);

    let mut stream = TcpStream::connect(SERVER_IP_ADDR)?;
    let serialized_args = serde_json::to_string(&args).unwrap();
    // the dockerfile will need to be sent over too, not just the args
    stream.write_all(serialized_args.as_bytes()).unwrap();

    updateJobState(&args.id, JobState::PREPARING);

    Ok(())
}



fn test(args: CortexArgs) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn ps() {
    ()
}


fn updateJobState(job_id: &String, new_state: JobState) {
    ()
}



/* ---------------------------------- Tests --------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_ARGS: CortexArgs = CortexArgs {
            cmd: CortexCommands::Run,
            id: String::from("test_job"),
            redundancy: 4,
            container: PathBuf::from(String::from(".")),
            volume: Some(PathBuf::from(String::from("../../test_resources/test_volume"))),
            gpus: String::from("1"),
            make: String::from("any"),
            model: String::from("any"),
            out: PathBuf::from("../../test_resources/test_outputs"),
            max_runtime: String::from("1 minute")
        };
    }


    #[test]
    fn commands_take_correct_path() {

    }
}

