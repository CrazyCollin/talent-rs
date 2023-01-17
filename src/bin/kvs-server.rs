extern crate core;

use std::env;
use std::path::PathBuf;
use std::process::exit;
use clap::{App, arg};
use log::{info, LevelFilter};
use kvs::{EngineType, KvStore, Result, SledKvsEngine, KvsEngine, KvsServer, KvsError};


const DEFAULT_SERVER_ADDRESS:&str="127.0.0.1:4000";

fn main()->Result<()>{
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let matches=App::new(format!("kvs-server"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            arg!(--addr <IPPORT>)
                .required(false)
                .default_value(DEFAULT_SERVER_ADDRESS)
        )
        .arg(
            arg!(--engine <ENGINENAME>)
                .required(false)
                .value_parser(["kvs", "sled"]),
        )
        .get_matches();
    let address=matches.get_one::<String>("addr").cloned().unwrap();
    let engine_type;
    match convert(matches.get_one::<String>("engine").cloned()) {
        Ok(engine)=>engine_type=engine,
        Err(err)=>{
            eprintln!("{:?}",err);
            exit(-1);
        }
    }
    start(env::current_dir()?,address,engine_type)?;
    Ok(())
}

// convert string into spec type
fn convert(engine:Option<String>)->Result<EngineType>{
    let dir_path=env::current_dir()?;
    match engine {
        Some(v)=>{
            if v==String::from("kvs") {
                if dir_path.join("sled").exists() {
                    return Err(KvsError::EngineErr);
                }
                Ok(EngineType::default_type())
            }else {
                if dir_path.join("kvs").exists() {
                    return Err(KvsError::EngineErr);
                }
                Ok(EngineType::Sled)
            }
        }
        None=>{
            if dir_path.join("sled").exists() {
                return Err(KvsError::EngineErr);
            }
            Ok(EngineType::default_type())
        }
    }

}

// start kv engine for kvs server use
fn start(dir_path:PathBuf,addr:String,engine_type:EngineType)->Result<()>{

    // first we should determine whether there is previous data here

    info!("kvs server {}",env!("CARGO_PKG_VERSION"));
    info!("start listen address: {}",&addr);
    match engine_type {
        EngineType::Default=>{
            info!("engine: {}",engine_type.to_string());
            start_server(KvStore::open(dir_path.join(engine_type.to_string()))?, addr)?;
        }
        EngineType::Sled=>{
            info!("engine: {}",engine_type.to_string());
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
