use super::Observer;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

enum ObserverGuardState {
    Waiting,
    Running,
}

struct ObserverGuard<O: Observer> {
    state: ObserverGuardState,
    observer: O,
}

impl<O: Observer> ObserverGuard<O> {
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

    fn record<T>(&mut self, output: &T) {
        if matches!(self.state, ObserverGuardState::Running) {
            self.observer.record(output);
        } else {
            unreachable!("observer must be started before recording");
        }
    }
}

impl<O: Observer> Drop for ObserverGuard<O> {
    fn drop(&mut self) {
        if matches!(self.state, ObserverGuardState::Waiting) {
            self.observer.stop();
        }
    }
}

pub struct MonitorFuture<O: Observer, F> {
    guard: ObserverGuard<O>,
    inner: F,
}

impl<O: Observer, F> MonitorFuture<O, F> {
    fn new(observer: O, inner: F) -> Self {
        Self {
            guard: ObserverGuard::new(observer),
            inner,
        }
    }
}

impl<O: Observer, F: Future> Future for MonitorFuture<O, F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safety: We're not moving both the inner future and the guard.
        let this = unsafe { self.get_unchecked_mut() };
        let inner = unsafe { Pin::new_unchecked(&mut this.inner) };
        this.guard.start_if_first_time();
        match inner.poll(cx) {
            Poll::Ready(output) => {
                // Safety: since record takes `&self`, user is not able to
                // break `Pin` invariants without unsafe on its side.
                this.guard.record(&output);
                Poll::Ready(output)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pub trait MetricsFutureExt: Future + Sized {
    fn monitor<O: Observer>(self, observation: O) -> MonitorFuture<O, Self> {
        MonitorFuture::new(observation, self)
    }
}

impl<F: Future + Sized> MetricsFutureExt for F {}
