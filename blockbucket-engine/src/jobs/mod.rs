use crate::worker::BlockbucketWorker;

pub trait JobProgress {
    fn name<P: AsRef<str>>(&self) -> P;
    fn set_max(&self, max: usize);
    fn set_progress(&self, progress: usize);
    fn set_current_task(&self, name: &str);
}

pub trait Job<W: BlockbucketWorker, R, E> {
    fn execute(self, worker: &W, progress: impl JobProgress) -> impl Future<Output = Result<R, E>>;
}