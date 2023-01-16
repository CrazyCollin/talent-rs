use crate::threadpool::ThreadPool;
use crate::Result;

pub struct RayonThreadPool{
    pool:rayon::ThreadPool
}

impl ThreadPool for RayonThreadPool {

    fn new(num: usize) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(RayonThreadPool {
            pool: rayon::ThreadPoolBuilder::new().num_threads(num).build()?,
        })
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.pool.spawn(job);
    }
}