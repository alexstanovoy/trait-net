use core::{pin::Pin, future::Future, task::{Context, Poll}};
use super::Observation;

pub struct MetricsFuture<F, G> {
    _guard: G,
    inner: F,
}

impl<G, F> MetricsFuture<F, G> {
    pub fn new(guard: G, inner: F) -> Self {
        Self { _guard: guard, inner }
    }
}

impl<G, F: Future> Future for MetricsFuture<F, G> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safety: We're not moving both the inner future and the logger.
        let this = unsafe { self.get_unchecked_mut() };
        let inner = unsafe { Pin::new_unchecked(&mut this.inner) };
        match inner.poll(cx) {
            Poll::Ready(value) => {
                Poll::Ready(value)
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

pub trait MetricsFutureExt: Future + Sized {
    fn with_observation<O: Observation>(self, observation: O) -> MetricsFuture<Self, O> {
        MetricsFuture::new(observation, self)
    }
}

impl<F: Future + Sized> MetricsFutureExt for F {}
