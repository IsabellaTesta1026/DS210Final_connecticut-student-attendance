pub fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len());

    let mut sum = 0.0;
    for i in 0..a.len() {
        let diff = a[i] - b[i];
        sum += diff * diff;
    }

    let dist = sum.powf(0.5); // same as taking square root
    dist
}

pub fn manhattan_distance(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len());

    let mut sum = 0.0;
    for i in 0..a.len() {
        sum += (a[i] - b[i]).abs();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        assert_eq!(euclidean_distance(&a, &b), 0.0);
    }

    #[test]
    fn test_manhattan_distance() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![2.0, 3.0, 4.0];
        assert_eq!(manhattan_distance(&a, &b), 3.0);
    }
}
