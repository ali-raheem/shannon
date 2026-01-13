//! Shannon entropy calculation library.
//!
//! Provides functions for calculating Shannon entropy of byte sequences,
//! useful for analyzing randomness and information density in data.

use num_traits::{Float, FromPrimitive};

/// Calculates the Shannon entropy of a byte slice.
///
/// Shannon entropy measures the average information content per byte,
/// ranging from 0 (completely uniform) to 8 (maximum randomness).
///
/// # Arguments
///
/// * `data` - A byte slice to analyze
///
/// # Returns
///
/// The entropy value in bits per byte (0.0 to 8.0)
///
/// # Example
///
/// ```
/// use shannon::entropy;
///
/// let uniform_data = vec![0u8; 100];
/// let e: f64 = entropy(&uniform_data);
/// assert_eq!(e, 0.0);
/// ```
pub fn entropy<F: Float + FromPrimitive>(data: &[u8]) -> F {
    let data_len = F::from_usize(data.len()).unwrap();
    let mut counts = [0usize; 256];
    for byte in data {
        counts[*byte as usize] += 1;
    }
    let mut entropy = F::zero();
    for count in counts {
        if count == 0 {
            continue;
        }
        let p_x = F::from_usize(count).unwrap() / data_len;
        entropy = entropy - p_x * p_x.log2();
    }
    entropy
}
