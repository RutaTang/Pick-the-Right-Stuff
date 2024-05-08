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
