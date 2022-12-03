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
            // Arg::with_name("--addr")
            //     .help("ip:port address")
            //     .required(false)
            //     .default_value(DEFAULT_SERVER_ADDRESS)
                arg!(--addr <IPPORT>)
                .required(false)
                .default_value(DEFAULT_SERVER_ADDRESS)
        )
        .arg(
            // Arg::with_name("--engine")
            //     .help("chose kv engine")
            //     .required(false)
            //     .default_value(EngineType::default_type().to_string().as_str())
            arg!(--engine <ENGINENAME>)
                .required(false)
                .value_parser(["kvs", "sled"]),
        )
        .get_matches();
    // let address=matches.value_of("addr").unwrap().to_string();
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

//
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

    // match engine {
    //     //
    //     Some(v)=>{
    //         // let engine_type=e.to_string();
    //         // let s1=String::from("kvs");
    //         // match e {
    //         //     s1=>{
    //                 if dir_path.join("sled").exists() {
    //                     return Err(KvsError::EngineErr);
    //                 }
    //                 Ok(EngineType::default_type())
    //         //     },
    //         //     "sled"=>{
    //         //         if dir_path.join("kvs").exists() {
    //         //             return Err(KvsError::EngineErr);
    //         //         }
    //         //         Ok(EngineType::Sled)
    //         //     },
    //         //     _ => Ok(EngineType::default_type())
    //         //
    //         // }
    //         if v==String::from("sled"){
    //             if dir_path.join("sled").exists() {
    //                 return Err(KvsError::EngineErr);
    //             }
    //             Ok(EngineType::default_type())
    //         }
    //
    //     }
    //     None=>{
    //         if dir_path.join(EngineType::to_string(&EngineType::Sled)).exists() {
    //             return Ok(EngineType::Sled);
    //         }
    //         Ok(EngineType::Default)
    //     }
    //
    // }
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
