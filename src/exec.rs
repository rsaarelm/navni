use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

/// Future that will wait for the completion of the current engine rendering
/// frame.
///
/// Should be awaited on at the bottom of the game loop.
#[derive(Default)]
pub struct FrameFuture {
    done: bool,
}

impl Future for FrameFuture {
    type Output = ();

    fn poll(
        mut self: Pin<&mut Self>,
        _context: &mut Context,
    ) -> Poll<Self::Output> {
        if self.done {
            // We were told to step, meaning this future gets destroyed and we run
            // the main future until we call next_frame again and end up in this poll
            // function again.
            Poll::Ready(())
        } else {
            self.done = true;
            Poll::Pending
        }
    }
}

// Called from the conditionally compiled backends, marked as dead_code
// here so backend-less compiles don't give a warning.
#[allow(dead_code)]
pub fn poll<T>(f: &mut Pin<Box<dyn Future<Output = T>>>) -> Option<T> {
    let waker = waker();
    let mut ctx = std::task::Context::from_waker(&waker);
    match Pin::new(f).poll(&mut ctx) {
        Poll::Pending => None,
        Poll::Ready(val) => Some(val),
    }
}

fn waker() -> Waker {
    use std::task::{RawWaker, RawWakerVTable};
    unsafe fn clone(data: *const ()) -> RawWaker {
        RawWaker::new(data, &VTABLE)
    }
    unsafe fn wake(_data: *const ()) {
        panic!("not supported")
    }
    unsafe fn wake_by_ref(data: *const ()) {
        unsafe { wake(data) }
    }
    unsafe fn drop(_data: *const ()) {
        // Nothing to do
    }
    const VTABLE: RawWakerVTable =
        RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    let raw_waker = RawWaker::new(std::ptr::null(), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}
