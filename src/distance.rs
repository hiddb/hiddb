//! Define different distance metrics

use num_traits::Float;

/// Returns the squared euclidean distance.
/// Avoids the expensive square root computation.
/// # Panics
///
/// In debug mode the length of the slices at input will be compared.
///
/// ```rust,should_panic
/// use hiddb::distance::squared_euclidean;
/// let _ = squared_euclidean(&[0.0, 0.0], &[1.0, 0.0, 0.0]);
/// ```
#[inline]
pub fn squared_euclidean<T: Float>(a: &[T], b: &[T]) -> T {
    debug_assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| ((*x) - (*y)) * ((*x) - (*y)))
        .fold(T::zero(), ::std::ops::Add::add)
}

/// Returns the euclidean distance.
#[inline]
pub fn euclidean<T: Float>(a: &[T], b: &[T]) -> T {
    Float::sqrt(squared_euclidean(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_squared_euclidean() {
        let vector_a: [f32; 3] = [1.0, 2.0, 3.0];
        let vector_b: [f32; 3] = [1.0, 2.0, 1.0];
        let squared_distance: f32 = squared_euclidean(&vector_a, &vector_b);
        assert!(approx_eq!(f32, 4.0, squared_distance, ulps = 2));

        let vector_a: [f64; 3] = [1.0, 2.0, 3.0];
        let vector_b: [f64; 3] = [1.0, 2.0, 1.0];
        let squared_distance: f64 = squared_euclidean(&vector_a, &vector_b);
        assert!(approx_eq!(f64, 4.0, squared_distance, ulps = 2));
    }

    #[test]
    fn test_euclidean() {
        let vector_a: [f32; 3] = [1.0, 2.0, 3.0];
        let vector_b: [f32; 3] = [1.0, 2.0, 1.0];
        let distance: f32 = euclidean(&vector_a, &vector_b);
        assert!(approx_eq!(f32, 2.0, distance, ulps = 2));

        let vector_a: [f64; 3] = [1.0, 2.0, 3.0];
        let vector_b: [f64; 3] = [1.0, 2.0, 1.0];
        let distance: f64 = euclidean(&vector_a, &vector_b);
        assert!(approx_eq!(f64, 2.0, distance, ulps = 2));
    }
}
