use num_traits::Float;

const NUMERICAL_ACCURACY: f64 = 1e-8;

pub fn float_vector_comp<T: Float>(arr_a: &[T], arr_b: &[T]) -> bool {
    for (&a, &b) in arr_a.iter().zip(arr_b.iter()) {
        if (a - b).abs().to_f64().unwrap() > NUMERICAL_ACCURACY {
            return false;
        }
    }
    true
}

mod tests {
    #[test]
    fn test_float_vector_comp() {
        use super::float_vector_comp;

        assert!(float_vector_comp(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0]));
        assert!(!float_vector_comp(&[1.0, 2.0, 3.0], &[1.0, 2.0, 2.9999]));
    }
}
