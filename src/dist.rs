use num::Float;

/// Calculates the gaussian distance between two lists of floats.
pub fn gaussian<N>(q: &[N], c: &[N]) -> N where N: Float {
    let sum = q
        .iter()
        .zip(c)
        .map(|(qi, ci)| (*qi - *ci).powi(2))
        .fold(N::from(0.0).unwrap(), |acc, x| acc + x);

    sum.sqrt()
}