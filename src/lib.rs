//! Shannon entropy calculation library.
//!
//! Provides functions for calculating Shannon entropy of byte sequences,
//! useful for analyzing randomness and information density in data.

use num_traits::{Float, FromPrimitive};

/// Represents the type of entropy edge detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    Rising,
    Falling,
}

/// Represents a detected entropy edge in a sequence of entropy values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntropyEdge<F> {
    /// Block index where the edge was detected
    pub block_index: usize,
    /// Type of edge (rising or falling)
    pub edge_type: EdgeType,
    /// Normalized entropy value (0.0 to 1.0) at this edge
    pub entropy: F,
}

/// Detects rising and falling edges in a sequence of entropy values.
///
/// Uses hysteresis to avoid spurious edge detection: a rising edge is only
/// detected when entropy crosses above `high_threshold`, and a falling edge
/// when it crosses below `low_threshold`.
///
/// # Arguments
///
/// * `entropy_values` - Slice of (block_index, entropy) tuples where entropy is in bits (0-8)
/// * `high_threshold` - Normalized threshold (0.0-1.0) for detecting rising edges
/// * `low_threshold` - Normalized threshold (0.0-1.0) for detecting falling edges
///
/// # Returns
///
/// A vector of detected entropy edges
///
/// # Example
///
/// ```
/// use shannon::{detect_edges, EdgeType};
///
/// // Entropy values in bits (0-8): starts high, drops low
/// let values = vec![(0, 7.8_f64), (1, 7.9), (2, 2.0), (3, 1.0)];
/// let edges = detect_edges(&values, 0.95, 0.85);
/// assert_eq!(edges.len(), 2);
/// assert_eq!(edges[0].edge_type, EdgeType::Rising);
/// assert_eq!(edges[0].block_index, 0);
/// assert_eq!(edges[1].edge_type, EdgeType::Falling);
/// assert_eq!(edges[1].block_index, 2);
/// ```
pub fn detect_edges<F: Float + FromPrimitive>(
    entropy_values: &[(usize, F)],
    high_threshold: F,
    low_threshold: F,
) -> Vec<EntropyEdge<F>> {
    let eight = F::from_f64(8.0).unwrap();
    let mut edges = Vec::new();
    let mut last_edge: Option<bool> = None;
    let mut trigger_reset = true;

    for &(block_index, entropy) in entropy_values {
        let normalized = entropy / eight;

        if (matches!(last_edge, None | Some(false)) && normalized > low_threshold)
            || (matches!(last_edge, Some(true)) && normalized < high_threshold)
        {
            trigger_reset = true;
        }

        if trigger_reset && normalized >= high_threshold {
            edges.push(EntropyEdge {
                block_index,
                edge_type: EdgeType::Rising,
                entropy: normalized,
            });
            last_edge = Some(true);
            trigger_reset = false;
        } else if trigger_reset && normalized <= low_threshold {
            edges.push(EntropyEdge {
                block_index,
                edge_type: EdgeType::Falling,
                entropy: normalized,
            });
            last_edge = Some(false);
            trigger_reset = false;
        }
    }

    edges
}

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
/// Calculates the total Shannon entropy of a byte slice.
///
/// Shannon entropy measures the average information content per byte,
/// multiplied by the length this returns the total entropy of the byte slice.
///
/// # Arguments
///
/// * `data` - A byte slice to analyze
///
/// # Returns
///
/// The total entropy of data
///
/// # Example
///
/// ## Calculating total entropy of a String
///
/// ```
/// use shannon::total_entropy;
///
/// let text = String::from("AABB");
/// let e = total_entropy::<f64>(text.as_bytes());
/// assert_eq!(e, 4.0);
/// ```
///
pub fn total_entropy<F: Float + FromPrimitive>(data: &[u8]) -> F {
    entropy::<F>(data) * (F::from_usize(data.len()).unwrap())
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
