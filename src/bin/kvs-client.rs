use clap::{App, Arg, SubCommand};
use predicates::ord::le;
use kvs::{Request, Result};
use kvs::KvsClient;

const DEFAULT_SERVER_ADDRESS:&str="localhost::4000";

fn main()->Result<()>{
    let matches=App::new(format!("kvs-client"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("set")
                .about("set a value to kv")
                .arg(Arg::with_name("key").help("a key for value").required(true))
                .arg(Arg::with_name("value").help("a value for key").required(true))
                .arg(Arg::with_name("--addr").help("ip:port address").default_value(DEFAULT_SERVER_ADDRESS))
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("get value from kv")
                .arg(Arg::with_name("key").help("a key for value").required(true))
                .arg(Arg::with_name("addr").help("ip:port address").default_value(DEFAULT_SERVER_ADDRESS))
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("remove a kv pair from kv")
                .arg(Arg::with_name("key").help("a key for value").required(true))
                .arg(Arg::with_name("--addr").help("ip:port address").default_value(DEFAULT_SERVER_ADDRESS))
        )
        .get_matches();
    let mut  client;
    match matches.subcommand() {
        ("set",Some(command))=>{
            let addr=command.value_of("--addr").unwrap();
            let request=Request::SET(
                command.value_of("key").unwrap().to_owned(),
                command.value_of("value").unwrap().to_owned());
            client=KvsClient::new(addr)?;
            client.send_request(&request)?;
        }
        ("get",Some(command))=>{
            let addr=command.value_of("--addr").unwrap();
            let request=Request::GET(
                command.value_of("key").unwrap().to_owned(),);
            client=KvsClient::new(addr)?;
            match client.send_request(&request)? {
                Some(val)=>{
                    println!("{}",val);
                }
                None=>{
                    println!("Key not found");
                }
            }
        }
        ("rm",Some(command))=>{
            let addr=command.value_of("--addr").unwrap();
            let request=Request::REMOVE(
                command.value_of("key").unwrap().to_owned(),);
            client=KvsClient::new(addr)?;
            client.send_request(&request)?;
        }
        _=>unreachable!()
    }
    Ok(())
}
