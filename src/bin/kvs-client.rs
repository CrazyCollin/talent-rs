use std::process::exit;
use clap::{arg, ArgMatches, command, SubCommand};
use kvs::{Request};
use kvs::KvsClient;
use kvs::Result;

const DEFAULT_SERVER_ADDRESS:&str="127.0.0.1:4000";

fn main(){
    let matches= command!().name("kvs-client")
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string. Return an error if the value is not written successfully.")
                .arg(arg!(<KEY>))
                .arg(arg!(<VALUE>))
                .arg(arg!(--addr <IPPORT>).required(false).default_value(DEFAULT_SERVER_ADDRESS)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .arg(arg!(<KEY>))
                .arg(arg!(--addr <IPPORT>).required(false).default_value(DEFAULT_SERVER_ADDRESS))
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("remove a kv pair from kv")
                .arg(arg!(<KEY>))
                .arg(arg!(--addr <IPPORT>).required(false).default_value(DEFAULT_SERVER_ADDRESS))
        )
        .get_matches();
    if let Err(err) = send_request(matches) {
        eprintln!("{:?}",err);
        exit(-1);
    }
}

fn send_request(matches:ArgMatches)->Result<()>{
    match matches.subcommand() {
        Some(("set",command))=>{
            let addr=command.get_one::<String>("addr").unwrap();
            let key=command.get_one::<String>("KEY").unwrap();
            let value=command.get_one::<String>("VALUE").unwrap();
            let request=Request::SET(
                key.to_owned(),value.to_owned()
            );
            let mut client=KvsClient::new(addr.to_owned())?;
            client.send_request(&request)?;
        }
        Some(("get",command))=>{
            let addr=command.get_one::<String>("addr").unwrap();
            let key=command.get_one::<String>("KEY").unwrap();
            let request=Request::GET(
                key.to_owned()
            );
            let mut client=KvsClient::new(addr.to_owned())?;
            match client.send_request(&request)? {
                Some(val)=>{
                    println!("{}",val);
                }
                None=>{
                    println!("Key not found");
                }
            }
        }
        Some(("rm",command))=>{
            let addr=command.get_one::<String>("addr").unwrap();
            let key=command.get_one::<String>("KEY").unwrap();
            let request=Request::REMOVE(
                key.to_owned()
            );
            let mut client=KvsClient::new(addr.to_owned())?;
            client.send_request(&request)?;
        }
        _=>exit(-1)
    }
    Ok(())
}
