use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use crate::Delay;

enum MainFuture {
    State0,
    State1(Delay),
    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        use MainFuture::*;

        loop {
            match *self {
                State0 => {
                    let when = Instant::now() + Duration::from_millis(512);
                    let future = Delay::new(when);
                    *self = State1(future);
                }
                State1(ref mut fut) => match Pin::new(fut).poll(cx) {
                    Poll::Ready(out) => {
                        assert_eq!(out, "done");
                        *self = Terminated;
                        return Poll::Ready(());
                    }
                    Poll::Pending => {
                        return Poll::Pending;
                    }
                },
                Terminated => {
                    panic!("MainFuture polled after completion")
                }
            }
        }
    }
}
