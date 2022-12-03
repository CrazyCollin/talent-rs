use crate::errors::Result;
use crate::{KvsError, Request, Response};
use serde::Deserialize;
use serde_json::de::IoRead;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub struct KvsClient {
    reader: serde_json::de::Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
    // reader: BufReader<TcpStream>,
    // writer: BufWriter<TcpStream>,
}

impl KvsClient {
    pub fn new(addr: impl ToSocketAddrs) -> Result<Self> {
        let reader = TcpStream::connect(addr)?;
        let writer = reader.try_clone()?;
        Ok(Self {
            reader: serde_json::de::Deserializer::from_reader(BufReader::new(reader)),
            writer: BufWriter::new(writer),
        })
    }

    pub fn send_request(&mut self, request: &Request) -> Result<Option<String>> {
        // serde_json::to_writer(&mut self.writer, request)?;
        // self.writer.flush()?;
        // let resp_reader=&self.reader.into_iter::<Response>();
        // for resp in resp_reader {
        //     let resp=resp?;
        //     // info!("{:?}",&resp);
        //     match resp {
        //         Response::Ok(val) => {
        //             return Ok(val);
        //         }
        //         Response::Err(err) => {
        //             return Err(KvsError::StrErr(err));
        //         }
        //     }
        // }

        serde_json::to_writer(&mut self.writer,request)?;
        self.writer.flush()?;
        // let resp_reader=serde_json::de::Deserializer::from_reader(&mut self.reader).into_iter::<Response>();
        // for resp in resp_reader {
        //     let resp=resp?;
        //     // info!("{:?}",&resp);
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


        match Response::deserialize(&mut self.reader)? {
            Response::Ok(value) => Ok(value),
            Response::Err(err) => Err(KvsError::StrErr(err)),
        }
    }
}


// use serde::Deserializer;
// use serde_json::de::{ IoRead};
//
// pub struct Client {
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
//             Response::Err(err) => Err(KVStoreError::CommonStringError(err)),
//         }
//     }
// }