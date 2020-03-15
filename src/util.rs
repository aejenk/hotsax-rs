use num::Float;

#[inline]
/// Returns the mean of the list
pub fn mean<R, F>(data: &R) -> F where R: AsRef<[F]>, F: Float {
    data.as_ref().iter().fold(F::zero(), |a,b| a+*b) / F::from(data.as_ref().len()).unwrap()
}

/// Returns the standard deviation of the list
pub fn std_dev<R, F>(data: &R) -> F where R: AsRef<[F]>,  F: Float {
    let mean = mean(data);

    let sum = data.as_ref()
        .iter()
        .map(|e| (*e-F::from(mean).unwrap()).powi(2))
        .fold(F::zero(), |a,b| a+b);

    (sum/F::from(data.as_ref().len()).unwrap()).sqrt()
}

/// Computes the z-normalisation of the list itself.
pub fn znorm<R, F>(data: &R) -> Vec<F> where R: AsRef<[F]>, F: Float {
    let mean = mean(data);
    let std_dev = std_dev(data);
    data.as_ref().iter().map(|e| (*e-mean) / std_dev).collect()
}