use std::env;
use std::env::current_dir;
use std::process::exit;
use clap::{App, Arg, SubCommand};
use kvs::KvStore;
use kvs::KvsError;
use kvs::Result;

fn main()->Result<()>{
    let matches=App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("set")
                .about("set a value to kv")
                .arg(Arg::with_name("key").help("a key for value").required(true))
                .arg(Arg::with_name("value").help("a value for key").required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("get value from kv")
                .arg(Arg::with_name("key").help("a key for value").required(true))
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("remove a kv pair from kv")
                .arg(Arg::with_name("key").help("a key for value").required(true))
        )
        .get_matches();
    let mut store=KvStore::open(current_dir()?)?;
    match matches.subcommand() {
        ("set",Some(sub))=>{
            let key=sub.value_of("key").unwrap();
            let val=sub.value_of("value").unwrap();
            if let Err(err)=store.set(key.to_owned(),val.to_owned())  {
                println!("{:?}",err);
                exit(-1)
            }
        },
        ("get",Some(sub))=>{
            let key=sub.value_of("key").unwrap();
            match store.get(key.to_owned())?  {
                Some(value)=>{
                    println!("{}",value)
                }
                None=>{
                    println!("Key not found")
                }
            }
        },
        ("rm",Some(sub))=>{
            let key=sub.value_of("key").unwrap();
            if let Err(KvsError::KeyNotFound)=store.remove(key.to_owned())  {
                println!("Key not found");
                exit(-1)
            }
        },
        _=>unreachable!(),
    }
    Ok(())
}