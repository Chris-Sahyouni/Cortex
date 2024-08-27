use std::{error::Error, net::TcpStream, io::{prelude::*, BufReader}, path::{Path, PathBuf}};
use std::process;
use clap::Parser;
use cortex::{AllArgs, CortexArgs, CortexCommands, JobState, PathArgs};

const SERVER_IP_ADDR: &str = "127.0.0.1:32503";


fn main() {

    let AllArgs {cortex_args, path_args} = AllArgs::parse();


    match cortex_args.cmd {
        CortexCommands::Run => {
            if let Err(e) = run(cortex_args, path_args) {
                println!("Error from run: {}", e);
                process::exit(1);
            }
        }
        CortexCommands::Check => {
            if let Err(e) = check(cortex_args, path_args) {
                println!("Error from check: {}", e);
                process::exit(1);
            }
        }
        CortexCommands::Ps => {
            ps();
        }
    }

}


fn run(args: CortexArgs, paths: PathArgs) -> Result<(), Box<dyn Error>> {

    update_job_state(&args.id, JobState::INIT);

    let mut stream = TcpStream::connect(SERVER_IP_ADDR)?;
    stream.write_all(serialize(&args).as_bytes()).unwrap();

    update_job_state(&args.id, JobState::PREPARING);

    let mut response = String::new();
    let mut buf_reader = BufReader::new(&mut stream);
    if let Ok(_bytes_read) = buf_reader.read_line(&mut response) {
        let response_parts: Vec<&str> = response.split(":").collect();
        let job_id = response_parts[1];
        assert_eq!(job_id, args.id);
        let hosts_requested = args.redundancy;
        match response_parts[0] {
            "Couldn't find requested number of hosts" => {
                let num_hosts: u32 = response_parts[2].parse().expect("Number of hosts field in response could not be parsed to u32");
                println!("Cortex: only {num_hosts} / {hosts_requested} hosts found");
                println!("Would you like to:");
                // users should be able to select from the following options
                println!("Run the job on {num_hosts} hosts?");
                println!("Queue the job until {hosts_requested} are available");
                println!("Cancel the job?");
            },
            "Hosts-Found" => {
                assert_eq!(args.redundancy, hosts_requested);
                println!("Cortex: {hosts_requested} / {hosts_requested} hosts found, sending dockerfile and volumes");
            },
            _ => ()
        }
    }


    Ok(())
}


fn serialize(args: &CortexArgs) -> String {
    let mut serialized_args = serde_json::to_string(&args).unwrap();
    serialized_args.push('\n');
    serialized_args
}



fn check(args: CortexArgs, paths: PathArgs) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn ps() {
    ()
}


fn update_job_state(job_id: &String, new_state: JobState) {
    ()
}



/* ---------------------------------- Tests --------------------------------- */

#[cfg(test)]
mod tests {
    // use super::*;
    // use lazy_static::lazy_static;

    // lazy_static! {
    //     #[derive(Debug)]
    //     static ref TEST_ARGS: AllArgs = AllArgs {
    //         cortex_args {cmd: CortexCommands::Run,
    //         id: String::from("test_job"),
    //         redundancy: 4,
    //         max_runtime: String::from("1 minute")
    //         gpus: String::from("1"),
    //         make: String::from("any"),
    //         model: String::from("any"),
    //             ]
    //         out: PathBuf::from("../../test_resources/test_outputs"),
    //         container: PathBuf::from(String::from(".")),
    //         volume: Some(PathBuf::from(String::from("../../test_resources/test_volume"))),
    //     };
    // }


    // #[test]
    // fn data_preserved_after_serialization() {
    //     let copy_of_args = TEST_ARGS.clone();
    //     let serialized_args = serde_json::to_string(&copy_of_args).unwrap();
    //     println!("{}", serialized_args);
    //     let deserialized_args: CortexArgs = serde_json::from_str(&serialized_args).unwrap();
    //     assert_eq!(deserialized_args, TEST_ARGS.clone());
    // }
}

