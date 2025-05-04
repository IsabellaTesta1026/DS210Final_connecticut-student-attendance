//calculations for the Euclidean distances between equal len vectors
pub fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    //check equal len
    assert_eq!(a.len(), b.len());
    //sum of squares of the difference between each pair
    let mut sum = 0.0;
    for i in 0..a.len() {
        let diff = a[i] - b[i];
        sum += diff * diff;
    }
    //square root of the sum
    let dist = sum.powf(0.5);
    dist
}
//calculations for the manhattan distance between equal len vectors
pub fn manhattan_distance(a: &[f64], b: &[f64]) -> f64 {
    //check equal len
    assert_eq!(a.len(), b.len());
    //sum of the abs difference between pairs
    let mut sum = 0.0;
    for i in 0..a.len() {
        sum += (a[i] - b[i]).abs();
    }

    sum
}
//test
#[cfg(test)]
mod tests {
    use super::*; //bring outer mods

    #[test]
    //Euclidean distance between identical vectors should be 0
    fn test_euclidean_distance() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        assert_eq!(euclidean_distance(&a, &b), 0.0);
    }

    #[test]
    //Manhattan distance between [1,2,3] and [2,3,4] is |1-2|+|2-3|+|3-4| = 3.0
    fn test_manhattan_distance() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![2.0, 3.0, 4.0];
        assert_eq!(manhattan_distance(&a, &b), 3.0);
    }
}
