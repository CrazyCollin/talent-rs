extern crate core;

use core::panicking::panic;
use std::env;
use std::path::PathBuf;
use std::process::exit;
use clap::{App, Arg, SubCommand};
use log::LevelFilter;
use kvs::{EngineType, KvStore, Sled, Result, SledKvsEngine, KvsEngine, KvsServer, KvsError};


const DEFAULT_SERVER_ADDRESS:&str="localhost::4000";

fn main()->Result<()>{
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let matches=App::new(format!("kvs-server"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(format!("kv server use"))
        .arg(
            Arg::with_name("--addr")
                .help("ip:port address")
                .default_value(DEFAULT_SERVER_ADDRESS)
        )
        .arg(
            Arg::with_name("--engine")
                .help("chose kv engine")
                .default_value(EngineType::default_type().to_string().as_str())
        )
        .get_matches();
    let address=matches.value_of("--addr").unwrap().to_string();
    let engine_type;
    match convert(matches.value_of("--engine")) {
        Ok(engine)=>engine_type=engine,
        Err(err)=>{
            eprintln!("{:?}",err);
            exit(-1);
        }
    }
    start(env::current_dir()?,address,engine_type)?;
    Ok(())
}

//
fn convert(engine:Option<&str>)->Result<EngineType>{
    let dir_path=env::current_dir()?;
    match engine {
        //
        Some(e)=>{
            let engine_type=e.to_string();
            match engine_type {
                EngineType::to_string(&EngineType::Default)=>{
                    if dir_path.join(EngineType::to_string(&EngineType::Sled)).exists() {
                        return Err(KvsError::EngineErr);
                    }
                    Ok(EngineType::default_type())
                },
                EngineType::to_string(&EngineType::Sled)=>{
                    if dir_path.join(EngineType::to_string(&EngineType::Default)).exists() {
                        return Err(KvsError::EngineErr);
                    }
                    Ok(EngineType::Sled)
                },
            }

        }
        None=>{
            if dir_path.join(EngineType::to_string(&EngineType::Sled)).exists() {
                return Ok(EngineType::Sled);
            }
            Ok(EngineType::Default)
        }

    }
}

// start kv engine for kvs server use
fn start<E:KvsEngine>(dir_path:PathBuf,addr:String,engine_type:EngineType)->Result<()>{

    // first we should determine whether there is previous data here
    match engine_type {
        EngineType::Default=>{
            start_server(KvStore::open(dir_path.join(engine_type.to_string()))?, addr)?;
        }
        EngineType::Sled=>{
            start_server(SledKvsEngine::open(dir_path.join(engine_type.to_string()))?, addr)?;
        }
    }
    Ok(())
}

fn start_server<E:KvsEngine>(engine:E,addr:String)->Result<()>{
    let mut server=KvsServer::new(engine);
    server.run(addr)?;
    Ok(())
}
