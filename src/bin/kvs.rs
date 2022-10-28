use std::process::exit;
use clap::{App, Arg, SubCommand};

fn main(){
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
    match matches.subcommand() {
        ("set",Some(_matches))=>{
            eprintln!("unimplemented");
            exit(1);
        },
        ("get",Some(_matches))=>{
            eprintln!("unimplemented");
            exit(1);
        },
        ("rm",Some(_matches))=>{
            eprintln!("unimplemented");
            exit(1);
        },
        _=>unreachable!(),
    }
}