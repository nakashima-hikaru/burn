use super::state::FormatOptions;
use super::state::NumericMetricState;
use super::MetricEntry;
use crate::metric::{Metric, Numeric};
use burn_core::tensor::backend::Backend;
use burn_core::tensor::ElementConversion;
use burn_core::tensor::Tensor;

/// The loss metric.
#[derive(Default)]
pub struct LossMetric<B: Backend> {
    state: NumericMetricState,
    _b: B,
}

/// The [loss metric](LossMetric) input type.
#[derive(new)]
pub struct LossInput<B: Backend> {
    tensor: Tensor<B, 1>,
}

impl<B: Backend> LossMetric<B> {
    /// Create the metric.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<B: Backend> Metric for LossMetric<B> {
    type Input = LossInput<B>;

    fn update(&mut self, loss: &Self::Input) -> MetricEntry {
        let loss = f64::from_elem(loss.tensor.clone().mean().into_data().value[0]);

        self.state
            .update(loss, 1, FormatOptions::new("Loss").precision(4))
    }

    fn clear(&mut self) {
        self.state.reset()
    }
}

impl<B: Backend> Numeric for LossMetric<B> {
    fn value(&self) -> f64 {
        self.state.value()
    }
}
