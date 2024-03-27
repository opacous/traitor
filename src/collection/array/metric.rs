use {
    crate::{
        analysis::{Metric, Real, RealExponential},
        collection::{Array, ArraySub, RealArray},
        ops::*,
    },
    num_traits::Zero,
};

// Haven't figured out yet how to use InnerProductMetric to impl this...
pub struct EuclideanMetric;

impl<'a, X: ArraySub + RealArray> Metric<&'a X, X::Element> for EuclideanMetric {
    fn distance(&self, x1: &'a X, x2: &'a X) -> X::Element {
        x1.zip_fold(x2, X::Element::repr(0.0), |a, (x, y)| {
            a.add(&(x.sub(y)).pow(X::Element::repr(2.0)))
        })
        .sqrt()
    }
}

pub struct WeightedEuclideanMetric<W: RealArray> {
    pub weights: W,
}

impl<W: RealArray> WeightedEuclideanMetric<W> {
    pub fn new(weights: W) -> Self {
        WeightedEuclideanMetric { weights }
    }
}

impl<'a, X: RealArray, W: RealArray> Metric<&'a X, X::Element> for WeightedEuclideanMetric<W> {
    fn distance(&self, x1: &'a X, x2: &'a X) -> X::Element {
        let mut acc = X::Element::zero();

        for (ind, w) in self.weights.iter().enumerate() {
            acc += w
                .transfer::<X::Element>()
                .mul(&(x1.nth(ind).unwrap().sub(x2.nth(ind).unwrap())).pow(2.0.transfer()))
        }

        acc.sqrt()
    }
}
