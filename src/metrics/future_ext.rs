use crate::metrics::Observer;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

enum State {
    Initialized,
    Started,
    Ended,
}

pub struct ObserverFuture<Obs, Fut, Out>
where
    Obs: Observer<Out>,
    Fut: Future<Output = Out>,
{
    state: State,
    observer: Obs,
    inner: Fut,
}

impl<Obs, Fut, Out> ObserverFuture<Obs, Fut, Out>
where
    Obs: Observer<Out>,
    Fut: Future<Output = Out>,
{
    fn new(observer: Obs, inner: Fut) -> Self {
        Self {
            state: State::Initialized,
            observer,
            inner,
        }
    }
}

impl<Obs, Fut, Out> Future for ObserverFuture<Obs, Fut, Out>
where
    Obs: Observer<Out>,
    Fut: Future<Output = Out>,
{
    type Output = Out;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safety: We're not moving both the inner future and the guard.
        let this = unsafe { self.get_unchecked_mut() };
        let inner = unsafe { Pin::new_unchecked(&mut this.inner) };
        if matches!(this.state, State::Initialized) {
            this.observer.start();
            this.state = State::Started;
        }
        match inner.poll(cx) {
            Poll::Ready(output) => {
                this.observer.record(&output);
                this.observer.stop();
                this.state = State::Ended;
                Poll::Ready(output)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<Obs, Fut, Out> Drop for ObserverFuture<Obs, Fut, Out>
where
    Obs: Observer<Out>,
    Fut: Future<Output = Out>,
{
    fn drop(&mut self) {
        if matches!(self.state, State::Started) {
            self.observer.stop();
            self.state = State::Ended;
        }
    }
}

pub trait MetricsFutureExt: Future + Sized {
    fn observe<Obs: Observer<Self::Output>>(
        self,
        observer: Obs,
    ) -> ObserverFuture<Obs, Self, Self::Output> {
        ObserverFuture::new(observer, self)
    }
}

impl<F: Future + Sized> MetricsFutureExt for F {}
