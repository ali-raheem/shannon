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
/// ## Calculating entropy of a Vec
///
/// ```
/// use shannon::entropy;
///
/// let uniform_data = vec![0u8; 100];
/// let e: f64 = entropy(&uniform_data);
/// assert_eq!(e, 0.0);
/// ```
///
/// ## Calculating entropy per character of a String
///
/// ```
/// use shannon::entropy;
///
/// let text = String::from("AABB");
/// let e: f64 = entropy(text.as_bytes());
/// assert_eq!(e, 1.0);
/// ```
///
/// ## Calculating total entropy of a String
///
/// ```
/// use shannon::entropy;
///
/// let text = String::from("AABB");
/// let e = entropy::<f64>(text.as_bytes()) * (text.len() as f64);
/// assert_eq!(e, 4.0);
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn skewed() {
        let a = String::from("ABAB");
        let b = String::from("AAAB");
        let c = String::from("AAAAAB");
        let s_a: f64 = entropy(a.as_bytes());
        let s_b: f64 = entropy(b.as_bytes());
        let s_c: f64 = entropy(c.as_bytes());
        assert!(s_a > s_b);
        assert!(s_b > s_c);
    }
    #[test]
    fn same() {
        let a = String::from("ABAB");
        let b = String::from("AABB");
        let s_a: f32 = entropy(a.as_bytes());
        let s_b: f32 = entropy(b.as_bytes());
        assert_eq!(s_a, s_b);
    }
    #[test]
    fn exact2() {
        let a = String::from("ABAB");
        let s_a: f32 = entropy(a.as_bytes());
        assert_eq!(s_a, 1.0);
    }
    #[test]
    fn perm() {
        let a = String::from("ABAB");
        let b = String::from("CDCD");
        let s_a: f32 = entropy(a.as_bytes());
        let s_b = entropy::<f32>(b.as_bytes());
        assert_eq!(s_a, s_b);
    }
    #[test]
    fn expanding() {
        let a = String::new();
        let b = String::from("A");
        let c = String::from("AB");
        let d = String::from("ABCD");
        let e = String::from("AAAAAAA");
        let s_a: f32 = entropy(a.as_bytes());
        let s_b: f32 = entropy(b.as_bytes());
        let s_c: f32 = entropy(c.as_bytes());
        let s_d: f32 = entropy(d.as_bytes());
        let s_e: f32 = entropy(e.as_bytes());
        assert_eq!(s_a, s_b);
        assert!(s_b < s_c);
        assert!(s_c < s_d);
        assert_eq!(s_a, s_e);
    }
}
