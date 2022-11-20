use std::collections::HashMap;
use std::{fs, io};
use std::fs::{File, OpenOptions, remove_file};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use crate::errors::{KvsError,Result};


use crate::command::Command;

const COMPACT_THRESHOLD:u64=1024;

pub struct KvStore {
    // directory path for storage data log
    path:PathBuf,
    // to record command position
    index:HashMap<String,CommandPos>,
    // file id -> reader
    readers:HashMap<u64,BufReaderWithPos<File>>,
    // current file writer
    writer: BufWriterWithPos<File>,
    // current write file id
    current_file_id:u64,
    // kv store current data size, it should compact when
    // it's bigger than compact threshold value
    uncompacted:u64,
}

impl KvStore {
    pub fn open(path:impl Into<PathBuf>)->Result<KvStore>{
        let dir_path=path.into();
        // create dir if not exist
        fs::create_dir_all(&dir_path)?;

        let mut readers=HashMap::new();
        let mut index=HashMap::new();

        // get write file id and uncompacted size
        let (current_file_id,uncompacted)=Self::restore(&dir_path,&mut index,&mut readers)?;
        let current_filename=dir_path.join(format!("{}.log",current_file_id));

        let writer=BufWriterWithPos::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&current_filename)?
        )?;

        // return None, create index 0 reader
        if current_file_id==0 {
            readers.insert(
                current_file_id,
                BufReaderWithPos::new(File::open(&current_filename)?)?,
            );
        }


        let mut store=KvStore{
            path:dir_path,
            index,
            readers,
            writer,
            current_file_id,
            uncompacted
        };

        if store.uncompacted>COMPACT_THRESHOLD {
            store.compact()?;
        }

        Ok(store)
    }

    pub fn set(&mut self,key:String,value:String)->Result<()>{
        let command=Command::SET(key,value);
        let ser_command=serde_json::to_vec(&command)?;

        let offset=self.writer.position;

        // write command into writer buffer
        self.writer.write(&ser_command)?;
        self.writer.flush()?;

        let command_length=self.writer.position-offset;

        if let Command::SET(key, _) = command {
            // closure use
            self.uncompacted+=self.index.insert(key,CommandPos{
                file_id: self.current_file_id,
                offset,
                length: command_length,
            }).map(|command_pos|command_pos.length).unwrap_or(0);
        }

        // if current uncompacted data size is bigger than COMPACT_THRESHOLD,
        // then we should start compact operation
        if self.uncompacted>COMPACT_THRESHOLD {
            self.compact()?
        }
        Ok(())
    }

    pub fn get(&mut self,key:String)->Result<Option<String>>{
        if let Some(command_pos) = self.index.get(&key) {
            // get mut reader
            let reader=self.readers.get_mut(&command_pos.file_id)
                .expect("get reader error");
            reader.seek(SeekFrom::Start(command_pos.offset))?;
            let data_reader=reader.take(command_pos.length as u64);
            if let Command::SET(_,value) = serde_json::from_reader(data_reader)? {
                Ok(Some(value))
            }else {
                Err(KvsError::UnexpectedCommandType)
            }
        }else {
            Ok(None)
        }
    }

    pub fn remove(&mut self, key:String)->Result<()>{
        if self.index.contains_key(&key) {
            // it should be removed when memory map has 'SET' command,
            // so we should also remove this command when compact
            self.uncompacted+=self.index.remove(&key).map(|command_pos|command_pos.length).unwrap_or(0);
            let command=Command::REMOVE(key);
            let ser_command=serde_json::to_vec(&command)?;

            let offset=self.writer.position;

            self.writer.write(&ser_command)?;
            self.writer.flush()?;

            let command_length=self.writer.position-offset;
            self.uncompacted+=command_length;

            if self.uncompacted>COMPACT_THRESHOLD {
                self.compact()?;
            }

            Ok(())
        }else {
            Err(KvsError::KeyNotFound)
        }
    }

    // restore kv store from file
    // 1.Sort all log files with specified directory path
    // 2.Restore from sorted files, reload memory index map and readers map
    // return current write file index and uncompacted size
    fn restore(
        path:&PathBuf,
        index:&mut HashMap<String,CommandPos>,
        readers:&mut HashMap<u64,BufReaderWithPos<File>>
    )->Result<(u64,u64)>{
        let file_list=Self::sort_files(path)?;
        Self::reload_from_files(path,&file_list,index,readers)
    }

    // compact file if current memory file
    // size exceed threshold
    fn compact(&mut self)->Result<()>{
        // create a new file to load data from all memory data
        self.new_file()?;
        let mut offset=0;
        // traverse memory data map, load data from disk
        for command_pos in self.index.values_mut() {
            let reader=self.readers.get_mut(&command_pos.file_id).unwrap();
            reader.seek(SeekFrom::Start(command_pos.offset))?;
            let mut data_reader=reader.take(command_pos.length);
            // copy memory data into disk writer
            io::copy(&mut data_reader,&mut self.writer)?;
            let writer_pos=self.writer.position;
            // modify specific command position metadata
            *command_pos=CommandPos{
                file_id: self.current_file_id,
                offset,
                length: writer_pos-offset
            };
            offset=writer_pos;
        }
        // flush compact file
        self.writer.flush()?;
        // collect old file id, in order to delete log file
        let del_file_num_list:Vec<u64>=self.readers.iter()
            .map(|(file_id,_)|*file_id)
            .filter(|file_id|*file_id<self.current_file_id)
            .collect();
        for file_num in del_file_num_list.iter() {
            self.readers.remove(file_num);
            fs::remove_file(self.path.join(format!("{}.log",file_num)))?;
        }
        self.uncompacted=0;
        Ok(())
    }


    // create new log file
    fn new_file(&mut self)->Result<()>{
        // self increase file id
        self.current_file_id+=1;
        // join file path
        let new_file_dir=self.path.join(format!("{}.log",self.current_file_id));
        self.writer=BufWriterWithPos::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&new_file_dir)?,
        )?;
        // insert buffer reader into memory map
        self.readers.insert(self.current_file_id,BufReaderWithPos::new(File::open(&new_file_dir)?)?);
        Ok(())
    }

    fn sort_files(path: &PathBuf) ->Result<Vec<u64>>{
        let file_list=fs::read_dir(path)?;
        let mut list:Vec<u64>=file_list.flat_map(|res|res.map(|entry|entry.path()))
            .filter(|path|path.is_file()&&path.extension()==Some("log".as_ref()))
            .flat_map(|path|{
                path.file_name()
                    .and_then(|filename|filename.to_str())
                    .map(|filename|filename.trim_end_matches(".log"))
                    .map(str::parse::<u64>)
            })
            .flatten()
            .collect();
        list.sort();
        Ok(list)
    }

    fn reload_from_files(path:&PathBuf,
                         file_list:&Vec<u64>,
                         index:&mut HashMap<String,CommandPos>,
                         readers:&mut HashMap<u64,BufReaderWithPos<File>>
    )->Result<(u64,u64)>{
        let mut uncompacted:u64=0;
        for file_id in file_list {
            let file_path=path.join(format!("{}.log",file_id));
            // create current file reader
            let reader=BufReaderWithPos::new(
                File::open(&file_path)?
            )?;

            let mut stream=serde_json::Deserializer::from_reader(reader).into_iter::<Command>();
            let mut reader_pos=stream.byte_offset() as u64;

            while let Some(command) = stream.next() {
                let stream_pos=stream.byte_offset() as u64;
                match command? {
                    Command::SET(key,_)=>{
                        uncompacted+=index.insert(key,CommandPos{
                            file_id:*file_id,
                            offset: reader_pos,
                            length: stream_pos-reader_pos })
                            .map(|command_pos|command_pos.length)
                            .unwrap_or(0);
                    }
                    Command::REMOVE(key)=>{
                        uncompacted+=index.remove(&key)
                            .map(|command_pos|command_pos.length)
                            .unwrap_or(0);
                    }
                }
                reader_pos=stream_pos;
            }
            readers.insert(*file_id,BufReaderWithPos::new(File::open(&file_path)?)?);
        }
        Ok((*file_list.last().unwrap_or(&0), uncompacted))
    }
}

struct BufReaderWithPos<R:Read+Seek>{
    position:u64,
    reader:BufReader<R>
}

struct BufWriterWithPos<W:Write+Seek>{
    position:u64,
    writer:BufWriter<W>,
}

impl<R:Read+Seek> BufReaderWithPos<R> {
    fn new(mut inner:R)->Result<BufReaderWithPos<R>>{
        let position=inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos{
            position,
            reader: BufReader::new(inner),
        })
    }
}

impl<W:Write+Seek> BufWriterWithPos<W> {
    fn new(mut inner:W)->Result<BufWriterWithPos<W>>{
        let position=inner.seek(SeekFrom::End(0))?;
        Ok(BufWriterWithPos{
            position,
            writer:BufWriter::new(inner),
        })
    }
}

impl<R> Read for BufReaderWithPos<R>
where
    R:Read+Seek,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len=self.reader.read(buf)?;
        self.position+=len as u64;
        Ok(len)
    }
}

impl<R> Seek for BufReaderWithPos<R>
where
    R:Read+Seek,
{
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.reader.seek(pos)
    }
}

impl<W> Write for BufWriterWithPos<W>
where
    W:Write+Seek,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len=self.writer.write(buf)?;
        self.position+=len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W> Seek for BufWriterWithPos<W>
where
    W:Write+Seek,
{
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.writer.seek(pos)
    }
}

// record a command position in storage file
struct CommandPos{
    // file index number
    file_id:u64,
    // command in file's position
    offset:u64,
    // current command size
    length:u64,
}

#[cfg(test)]
mod tests{

    #[test]
    fn test(){

    }
}