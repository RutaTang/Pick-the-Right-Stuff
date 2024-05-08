use rand::rngs::StdRng;
use rand::seq::IteratorRandom;

/// Shuffle the given data in place while ensuring elements are not in their original place
pub fn shuffle<T>(data: &mut [T], rng: &mut StdRng) {
    for i in (1..data.len()).rev() {
        let range = 0..i;
        let j = range.choose(rng).unwrap();
        data.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_shuffle() {
        // Test with empty data
        let mut empty_data: [i32; 0] = [];
        let mut rng = StdRng::seed_from_u64(1);
        shuffle(&mut empty_data, &mut rng);
        assert_eq!(empty_data, []);

        // Test with single element
        let mut single_data = [42];
        let mut rng = StdRng::seed_from_u64(1);
        shuffle(&mut single_data, &mut rng);
        assert_eq!(single_data, [42]);

        // Test with multiple elements
        let mut data = [1, 2, 3, 4, 5];
        let mut rng = StdRng::seed_from_u64(1);
        shuffle(&mut data, &mut rng);
        assert_ne!(data, [1, 2, 3, 4, 5]);
    }

}
