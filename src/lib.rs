//! A currently heavily unstable crate containing algorithms relating to data analysis
//! for anomaly detection. The API will experience numerous breaking changes for the time being until
//! the API is finalized (v1.0)

/// Implements the brute force and HOT SAX algorithms as specified by Keogh's paper, found
/// [here](http://www.cse.cuhk.edu.hk/~adafu/Pub/icdm05time.pdf).
pub mod keogh;

/// Distance algorithms between two lists of floats.
///
/// Currently only contains support for `gaussian`.
pub mod dist;

/// Dimensionality reduction techniques.
///
/// Currently only includes piecewise approximation (paa) and
/// symbolic aggregate approximation (sax)
pub mod dim_reduction;

/// Miscellaneous utility functions.
pub mod util;

pub(crate) mod trie;

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use crate::keogh::{brute_force, hot_sax};
        let mut data: Vec<f64> = (1..1000).into_iter().map(|num| num as f64).collect();

        data[180] = 500.0;
        let windowsize= 20;

        dbg!(brute_force(&data, windowsize));
        dbg!(hot_sax(&data, windowsize, 10));
    }
}