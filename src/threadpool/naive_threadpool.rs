use crate::{threadpool::ThreadPool,Result};
use std::thread;

pub struct NaiveThreadPool;

impl ThreadPool for NaiveThreadPool {

    fn new(_: usize) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(NaiveThreadPool)
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(job);
    }
}
