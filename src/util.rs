use num::Float;

#[inline]
/// Returns the mean of the list
pub fn mean<F>(data: &Vec<F>) -> F where F: Float {
    data.iter().fold(F::zero(), |a,b| a+*b) / F::from(data.len()).unwrap()
}

/// Returns the standard deviation of the list
pub fn std_dev<F>(data: &Vec<F>) -> F where F: Float {
    let mean = mean(data);

    let sum = data
        .iter()
        .map(|e| (*e-F::from(mean).unwrap()).powi(2))
        .fold(F::zero(), |a,b| a+b);

    (sum/F::from(data.len()).unwrap()).sqrt()
}

/// Computes the z-normalisation of the list itself.
pub fn znorm<F>(data: &Vec<F>) -> Vec<F> where F: Float {
    let mean = mean(data);
    let std_dev = std_dev(data);
    data.iter().map(|e| (*e-mean) / std_dev).collect()
}