use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub mod simple_executor;

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    /// Takes an arbitrary future with an output type of `()` and pins it in memory
    /// through the `Box::pin` function. Then it wraps the boxed future in the `Task`
    /// struct and returns it.
    ///
    /// Static lifetime required because the returned `Task`
    /// can live for an arbitrary amount of time, so future must be valid for that
    /// amount of time as well
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
