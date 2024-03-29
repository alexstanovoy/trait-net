use crate::metrics::Observer;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

enum State {
    NotStarted,
    Polling,
}

pub struct MeteredFuture<Obs, Fut, Out>
where
    Obs: Observer<Out>,
    Fut: Future<Output = Out>,
{
    state: State,
    observer: Obs,
    inner: Fut,
}

impl<Obs, Fut, Out> MeteredFuture<Obs, Fut, Out>
where
    Obs: Observer<Out>,
    Fut: Future<Output = Out>,
{
    fn new(observer: Obs, inner: Fut) -> Self {
        Self {
            state: State::NotStarted,
            observer,
            inner,
        }
    }
}

impl<Obs, Fut, Out> Future for MeteredFuture<Obs, Fut, Out>
where
    Obs: Observer<Out>,
    Fut: Future<Output = Out>,
{
    type Output = Out;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safety: We're not moving both the inner future and the guard.
        let this = unsafe { self.get_unchecked_mut() };
        let inner = unsafe { Pin::new_unchecked(&mut this.inner) };
        if matches!(this.state, State::NotStarted) {
            this.observer.on_first_poll();
            this.state = State::Polling;
        }
        match inner.poll(cx) {
            Poll::Ready(output) => {
                this.observer.on_poll_ready(&output);
                Poll::Ready(output)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<Obs, Fut, Out> Drop for MeteredFuture<Obs, Fut, Out>
where
    Obs: Observer<Out>,
    Fut: Future<Output = Out>,
{
    fn drop(&mut self) {
        self.observer.on_drop();
    }
}

pub trait MetricsFutureExt: Future + Sized {
    fn observe<Obs: Observer<Self::Output>>(
        self,
        observer: Obs,
    ) -> MeteredFuture<Obs, Self, Self::Output> {
        MeteredFuture::new(observer, self)
    }
}

impl<F: Future + Sized> MetricsFutureExt for F {}
