use std::{error::Error, net::TcpStream, io::prelude::*};
use std::process;
use clap::Parser;

use cortex::CortexArgs;
use cortex::CortexCommands;
use cortex::JobState;

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