use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};
use log::info;
use serde_json::de::IoRead;
use crate::errors::Result;
use crate::{KvsError, proto, Request, Response};

pub struct KvsClient{
    reader:serde_json::de::Deserializer<IoRead<BufReader<TcpStream>>>,
    writer:BufWriter<TcpStream>
}

impl KvsClient {

    pub fn new(addr:impl ToSocketAddrs)->Result<Self>{
        let reader=TcpStream::connect(addr)?;
        let writer=reader.try_clone()?;
        Ok(Self{
            reader:serde_json::de::Deserializer::from_reader(BufReader::new(reader)),
            writer:BufWriter::new(writer)
        })
    }

    pub fn send_request(&mut self,request:&Request)->Result<Option<String>>{
        serde_json::to_writer(&mut self.writer,request)?;
        self.writer.flush()?;
        // let resp_reader=&self.reader.into_iter::<Response>();
        // for resp in resp_reader {
        //     let resp=resp?;
        //     info!("{:?}",&resp);
        //     match resp {
        //         Response::Ok(val) => {
        //             return Ok(val);
        //         }
        //         Response::Err(err) => {
        //             return Err(KvsError::StrErr(err));
        //         }
        //     }
        // }
        // return Ok(None);
        return match proto::Response::deserialize(&mut self.reader)? {
            Response::Ok(val) => {
                Ok(val)
            }
            Response::Err(err) => {
                Err(KvsError::StrErr(err))
            }
        }
    }

}