use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use log::info;
use crate::{KvsEngine, Request, Response};
use crate::errors::Result;
use serde::Deserialize;
use serde_json::Deserializer;

pub struct KvsServer<E:KvsEngine>{
    engine:E
}

impl<E:KvsEngine> KvsServer<E> {

    pub fn new(engine:E)->Self{
        Self{
            engine
        }
    }

    pub fn run(&mut self,addr:impl ToSocketAddrs)->Result<()>{
        let listener=TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream)=>{
                    self.handle_connection(stream)?;
                }
                Err(err)=>{
                    eprintln!("{}",err);
                }
            }
        }
        Ok(())
    }

    fn handle_connection(&mut self,mut stream:TcpStream)->Result<()>{
        let request=Request::deserialize(&mut Deserializer::from_reader(BufReader::new(&mut stream)))?;
        info!("{:?}",&request);
        let response;
        match request {
            Request::SET(key, value) => {
                match self.engine.set(key, value) {
                    Ok(_) => response = Response::Ok(None),
                    Err(err) => response = Response::Err(format!("{}", err)),
                };
            }
            Request::REMOVE(key) => {
                match self.engine.remove(key) {
                    Ok(_) => response = Response::Ok(None),
                    Err(err) => response = Response::Err(format!("{}", err)),
                };
            }
            Request::GET(key) => {
                match self.engine.get(key) {
                    Ok(value) => response = Response::Ok(value),
                    Err(err) => response = Response::Err(format!("{}", err)),
                };
            }
        }
        info!("Response: {:?}", &response);
        serde_json::to_writer(stream, &response)?;
        Ok(())

        // let mut writer=BufWriter::new(&stream);
        // let reader=BufReader::new(&stream);
        // let request_reader=serde_json::de::Deserializer::from_reader(reader).into_iter::<Request>();
        //
        // for request in request_reader {
        //     let request=request?;
        //     let response;
        //     info!("{:?}",&request);
        //     match request {
        //         Request::SET(key,val)=>{
        //             match self.engine.set(key,val) {
        //                 Ok(_)=>{
        //                     response=Response::Ok(None);
        //                 }
        //                 Err(err)=>{
        //                     response=Response::Err(format!("{}",err));
        //                 }
        //             }
        //         }
        //         Request::GET(key)=>{
        //             match self.engine.get(key) {
        //                 Ok(val)=>{
        //                     response=Response::Ok(val);
        //                 }
        //                 Err(err)=>{
        //                     response=Response::Err(format!("{}",err));
        //                 }
        //             }
        //         }
        //         Request::REMOVE(key)=>{
        //             match self.engine.remove(key) {
        //                 Ok(_)=>{
        //                     response=Response::Ok(None);
        //                 }
        //                 Err(err)=>{
        //                     response=Response::Err(format!("{}",err));
        //                 }
        //             }
        //         }
        //     }
        //     info!("{:?}",&response);
        //     serde_json::to_writer(&mut writer,&response)?;
        // }
        // Ok(())
    }

}