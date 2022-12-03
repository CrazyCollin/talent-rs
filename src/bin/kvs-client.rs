use std::process::exit;
use clap::{arg, ArgMatches, command, SubCommand};
use kvs::{Request};
use kvs::KvsClient;
use kvs::Result;

const DEFAULT_SERVER_ADDRESS:&str="127.0.0.1:4000";

fn main(){
    let matches= command!().name("kvs-client")
        .subcommand(
            // SubCommand::with_name("set")
                // .about("set a value to kv")
                // // .arg(Arg::with_name("key").help("a key for value").required(true))
                // // .arg(Arg::with_name("value").help("a value for key").required(true))
                // // .arg(Arg::with_name("--addr").help("ip:port address").default_value(DEFAULT_SERVER_ADDRESS))
                // .arg(arg!(<KEY>))
                // .arg(arg!(<VALUE>))
                // .arg(arg!(--addr <IPPORT>).required(false).default_value(DEFAULT_SERVER_ADDRESS))
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string. Return an error if the value is not written successfully.")
                .arg(arg!(<KEY>))
                .arg(arg!(<VALUE>))
                .arg(arg!(--addr <IPPORT>).required(false).default_value(DEFAULT_SERVER_ADDRESS)),
        )
        .subcommand(
            SubCommand::with_name("get")
                // .about("get value from kv")
                // .arg(Arg::with_name("key").help("a key for value").required(true))
                // .arg(Arg::with_name("addr").help("ip:port address").default_value(DEFAULT_SERVER_ADDRESS))
                .arg(arg!(<KEY>))
                .arg(arg!(--addr <IPPORT>).required(false).default_value(DEFAULT_SERVER_ADDRESS))
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("remove a kv pair from kv")
                // .arg(Arg::with_name("key").help("a key for value").required(true))
                // .arg(Arg::with_name("--addr").help("ip:port address").default_value(DEFAULT_SERVER_ADDRESS))
                .arg(arg!(<KEY>))
                .arg(arg!(--addr <IPPORT>).required(false).default_value(DEFAULT_SERVER_ADDRESS))
        )
        .get_matches();
    if let Err(err) = send_request(matches) {
        eprintln!("{:?}",err);
        exit(-1);
    }

    // let matches = command!()
    //     .name("kvs-Client")
    //     .subcommand(
    //         SubCommand::with_name("set")
    //             .about("Set the value of a string key to a string. Return an error if the value is not written successfully.")
    //             .arg(arg!(<KEY>))
    //             .arg(arg!(<VALUE>))
    //             .arg(arg!(--addr <IPPORT>).required(false).default_value("127.0.0.1:4000")),
    //     )
    //     .subcommand(
    //         SubCommand::with_name("get")
    //             .about("Get the string value of a string key. If the key does not exist, return None. Return an error if the value is not read successfully.")
    //             .arg(arg!(<KEY>))
    //             .arg(arg!(--addr <IPPORT>).required(false).default_value("127.0.0.1:4000")),
    //     )
    //     .subcommand(
    //         SubCommand::with_name("rm")
    //             .about("Remove a given key. Return an error if the key does not exist or is not removed successfully.c")
    //             .arg(arg!(<KEY>))
    //             .arg(arg!(--addr <IPPORT>).required(false).default_value("127.0.0.1:4000")),
    //     )
    //     .get_matches();
    // if let Err(err) = send_request(matches) {
    //     eprintln!("{:?}", err);
    //     exit(-1);
    // }

    // match matches.subcommand() {
    //     Some(("set",command))=>{
    //         // let addr=command.value_of("--addr").unwrap();
    //         let addr=command.get_one::<String>("addr").cloned().unwrap();
    //         let request=Request::SET(
    //             // command.value_of("key").unwrap().to_owned(),
    //             // command.value_of("value").unwrap().to_owned()
    //             command.get_one::<String>("key").cloned().unwrap(),
    //             command.get_one::<String>("value").cloned().unwrap()
    //         );
    //         let mut  client=KvsClient::new(addr).unwrap();
    //         client.send_request(&request).unwrap();
    //     }
    //     Some(("get",command))=>{
    //         // let addr=command.value_of("--addr").unwrap();
    //         let addr=command.get_one::<String>("addr").cloned().unwrap();
    //         let request=Request::GET(
    //             // command.value_of("key").unwrap().to_owned(),
    //             command.get_one::<String>("key").cloned().unwrap(),
    //         );
    //         let mut client=KvsClient::new(addr).unwrap();
    //         match client.send_request(&request).unwrap() {
    //             Some(val)=>{
    //                 println!("{}",val);
    //             }
    //             None=>{
    //                 println!("Key not found");
    //             }
    //         }
    //     }
    //     Some(("rm",command))=>{
    //         // let addr=command.value_of("--addr").unwrap();
    //         let addr=command.get_one::<String>("addr").cloned().unwrap();
    //         let request=Request::REMOVE(
    //             // command.value_of("key").unwrap().to_owned(),
    //             command.get_one::<String>("key").cloned().unwrap(),
    //         );
    //         let mut client=KvsClient::new(addr).unwrap();
    //         client.send_request(&request).unwrap();
    //     }
    //     _=>exit(-1)
    // }
}

fn send_request(matches:ArgMatches)->Result<()>{
    match matches.subcommand() {
        Some(("set",command))=>{
            // let addr=command.value_of("--addr").unwrap();
            let addr=command.get_one::<String>("addr").unwrap();
            let key=command.get_one::<String>("KEY").unwrap();
            let value=command.get_one::<String>("VALUE").unwrap();
            let request=Request::SET(
                // command.value_of("key").unwrap().to_owned(),
                // command.value_of("value").unwrap().to_owned()
                key.to_owned(),value.to_owned()
            );
            let mut client=KvsClient::new(addr.to_owned())?;
            client.send_request(&request)?;
        }
        Some(("get",command))=>{
            // let addr=command.value_of("--addr").unwrap();
            let addr=command.get_one::<String>("addr").unwrap();
            let key=command.get_one::<String>("KEY").unwrap();
            let request=Request::GET(
                // command.value_of("key").unwrap().to_owned(),
                // command.get_one::<String>("key").cloned().unwrap(),
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
            // let addr=command.value_of("--addr").unwrap();
            let addr=command.get_one::<String>("addr").unwrap();
            let key=command.get_one::<String>("KEY").unwrap();
            let request=Request::REMOVE(
                // command.value_of("key").unwrap().to_owned(),
                // command.get_one::<String>("key").cloned().unwrap(),
                key.to_owned()
            );
            let mut client=KvsClient::new(addr.to_owned())?;
            client.send_request(&request)?;
        }
        _=>exit(-1)
    }
    Ok(())
}

// use clap::{arg, command, ArgMatches, SubCommand};
// use kvs::{KvsError, Result};
// use kvs::{Request, Response};
// use serde::Deserialize;
// use serde_json::de::IoRead;
// use serde_json::Deserializer;
// use std::io::{BufReader, BufWriter, Write};
// use std::net::TcpStream;
// use std::string::String;
// use std::{env, process};
//
// fn main() {
//     let matches = command!()
//         .name("kvs-Client")
//         .subcommand(
//             SubCommand::with_name("set")
//                 .about("Set the value of a string key to a string. Return an error if the value is not written successfully.")
//                 .arg(arg!(<KEY>))
//                 .arg(arg!(<VALUE>))
//                 .arg(arg!(--addr <IPPORT>).required(false).default_value("127.0.0.1:4000")),
//         )
//         .subcommand(
//             SubCommand::with_name("get")
//                 .about("Get the string value of a string key. If the key does not exist, return None. Return an error if the value is not read successfully.")
//                 .arg(arg!(<KEY>))
//                 .arg(arg!(--addr <IPPORT>).required(false).default_value("127.0.0.1:4000")),
//         )
//         .subcommand(
//             SubCommand::with_name("rm")
//                 .about("Remove a given key. Return an error if the key does not exist or is not removed successfully.c")
//                 .arg(arg!(<KEY>))
//                 .arg(arg!(--addr <IPPORT>).required(false).default_value("127.0.0.1:4000")),
//         )
//         .get_matches();
//     if let Err(err) = send_request(matches) {
//         eprintln!("{:?}", err);
//         process::exit(-1);
//     }
// }
//
// fn send_request(matches: ArgMatches) -> Result<()> {
//     match matches.subcommand() {
//         Some(("set", sub_matches)) => {
//             let addr = sub_matches.get_one::<String>("addr").unwrap();
//             let key = sub_matches.get_one::<String>("KEY").unwrap();
//             let value = sub_matches.get_one::<String>("VALUE").unwrap();
//             let mut client = Client::new(addr)?;
//             client.request(&Request::SET(key.to_owned(), value.to_owned()))?;
//         }
//         Some(("get", sub_matches)) => {
//             let addr = sub_matches.get_one::<String>("addr").unwrap();
//             let key = sub_matches.get_one::<String>("KEY").unwrap();
//             let mut client = Client::new(addr)?;
//             match client.request(&Request::GET(key.to_owned()))? {
//                 None => println!("Key not found"),
//                 Some(value) => println!("{}", value),
//             };
//         }
//         Some(("rm", sub_matches)) => {
//             let addr = sub_matches.get_one::<String>("addr").unwrap();
//             let key = sub_matches.get_one::<String>("KEY").unwrap();
//             let mut client = Client::new(addr)?;
//             client.request(&Request::REMOVE(key.to_owned()))?;
//         }
//         _ => process::exit(-1),
//     }
//     Ok(())
// }
//
// struct Client {
//     reader: Deserializer<IoRead<BufReader<TcpStream>>>,
//     writer: BufWriter<TcpStream>,
// }
//
// impl Client {
//     fn new(addr: &str) -> Result<Client> {
//         let stream = TcpStream::connect(addr)?;
//         Ok(Client {
//             reader: Deserializer::from_reader(BufReader::new(stream.try_clone()?)),
//             writer: BufWriter::new(stream),
//         })
//     }
//
//     fn request(&mut self, request: &Request) -> Result<Option<String>> {
//         serde_json::to_writer(&mut self.writer, request)?;
//         self.writer.flush()?;
//         match Response::deserialize(&mut self.reader)? {
//             Response::Ok(value) => Ok(value),
//             Response::Err(err) => Err(KvsError::StrErr(err)),
//         }
//     }
// }
