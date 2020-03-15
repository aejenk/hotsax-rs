use num::Float;
use lazy_static::lazy_static;

lazy_static!(
    static ref BREAKPOINTS: [Vec<f64>; 5] = [
        vec![-0.43, 0.43], // 3
        vec![-0.67, 0.0, 0.67], // 4
        vec![-0.84, -0.25, 0.25, 0.84], // 5
        vec![-0.97, -0.43, 0.0, 0.43, 0.97], // 6
        vec![-1.07, -0.57, -0.18, 0.18, 0.57, 1.07] // 7
    ];
);

/// Returns a piecewise approximation of the original list of values.
/// The size of the output array will be the same as `dim`.
pub fn paa<N>(data: &Vec<N>, dim: usize) -> Vec<N> where N: Float {
    let len = data.len();

    if len <= dim {
        panic!("len <= dim: Output size is larger than the vector itself.");
    }

    let mut newvec : Vec<N> = Vec::new();

    for i in 0..dim {
        let j = ((len as f64/dim as f64) * (i as f64) + 1.0) as usize;
        let end = ((len as f64/dim as f64) * (i as f64+1.0)) as usize;

        let sum= data[j..end]
            .iter()
            .fold(N::zero(), |a, b| a+*b);

        newvec.push(N::from(dim as f64 / len as f64).unwrap() * sum);
    }

    newvec
}

fn to_sax_letter<N>(elem: &N, alpha: usize) -> char where N: Float {
    let breakpoints = &BREAKPOINTS[alpha-3];

    let num = elem.to_f64().unwrap();

    for (i, b) in breakpoints.iter().enumerate() {
        if b > &num {
            return ('a' as u8 + i as u8) as char;
        }
    }

    ('a' as u8 + (alpha-1) as u8) as char
}

/// Returns a sax word representation of the original list.
///
/// `word_size` determines the length of the word, and `alpha` represents the alphabet size.
///
/// # Panics
/// - if `alpha` is not between 3 and 7. Higher numbers can only be supported if the static
/// variable `BREAKPOINTS` is updated.
pub fn sax<N>(data: &Vec<N>, word_size: usize, alpha: usize) -> String where N: Float {
    let norm = super::util::znorm(data);
    let paa = paa(&norm, word_size);

    let string: String = paa
        .iter()
        .map(|e| to_sax_letter(e, alpha))
        .collect();

    return string;
}