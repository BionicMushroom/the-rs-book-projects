pub trait FnOnceSend: FnOnce() + Send + 'static {}
impl<T> FnOnceSend for T where T: FnOnce() + Send + 'static {}

pub type Job = Box<dyn FnOnceSend>;
