use super::{AsStatusLabel, ScopedObserver, StatusObserver};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

enum ObserverGuardState {
    Waiting,
    Running,
}

struct ObserverGuard<O: ScopedObserver> {
    state: ObserverGuardState,
    observer: O,
}

impl<O: ScopedObserver> ObserverGuard<O> {
    fn new(observer: O) -> Self {
        Self {
            state: ObserverGuardState::Waiting,
            observer,
        }
    }

    fn start_if_first_time(&mut self) {
        if matches!(self.state, ObserverGuardState::Waiting) {
            self.observer.start();
            self.state = ObserverGuardState::Running;
        }
    }
}

impl<O: ScopedObserver> Drop for ObserverGuard<O> {
    fn drop(&mut self) {
        if matches!(self.state, ObserverGuardState::Running) {
            self.observer.stop();
        }
    }
}

pub struct MonitorFuture<O: ScopedObserver, F> {
    guard: ObserverGuard<O>,
    inner: F,
}

impl<O: ScopedObserver, F> MonitorFuture<O, F> {
    fn new(observer: O, inner: F) -> Self {
        Self {
            guard: ObserverGuard::new(observer),
            inner,
        }
    }
}

impl<O: ScopedObserver, F: Future> Future for MonitorFuture<O, F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safety: We're not moving both the inner future and the guard.
        let this = unsafe { self.get_unchecked_mut() };
        let inner = unsafe { Pin::new_unchecked(&mut this.inner) };
        this.guard.start_if_first_time();
        inner.poll(cx)
    }
}

pub struct RecordFuture<O, F> {
    observer: O,
    inner: F,
}

impl<O, F> RecordFuture<O, F> {
    fn new(observer: O, inner: F) -> Self {
        Self { observer, inner }
    }
}

impl<O, F> Future for RecordFuture<O, F>
where
    O: StatusObserver,
    F: Future,
    F::Output: AsStatusLabel,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safety: We're not moving both the inner future and the guard.
        let this = unsafe { self.get_unchecked_mut() };
        let inner = unsafe { Pin::new_unchecked(&mut this.inner) };
        match inner.poll(cx) {
            Poll::Ready(output) => {
                this.observer.record(&output);
                Poll::Ready(output)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

// TODO: remove and use metaprogramming for only .monitor() func
pub trait MetricsFutureExt: Future + Sized {
    fn monitor<O: ScopedObserver>(self, observer: O) -> MonitorFuture<O, Self> {
        MonitorFuture::new(observer, self)
    }

    fn record<O>(self, observer: O) -> RecordFuture<O, Self>
    where
        O: StatusObserver,
        Self::Output: AsStatusLabel,
    {
        RecordFuture::new(observer, self)
    }
}

impl<F: Future + Sized> MetricsFutureExt for F {}
