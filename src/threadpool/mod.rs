
use crate::Result;

mod naive_threadpool;
mod rayon_threadpool;
mod shared_queue_threadpool;

pub use naive_threadpool::NaiveThreadPool;
pub use rayon_threadpool::RayonThreadPool;
pub use shared_queue_threadpool::SharedQueueThreadPool;

pub trait ThreadPool{

    fn new(threads: usize) -> Result<Self>
    where
        Self: Sized;

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;

}