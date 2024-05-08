use rand::rngs::StdRng;

use crate::utils::shuffle::shuffle;

use super::item::Item;

#[derive(Clone)]
pub struct Shelf {
    pub items: Vec<Item>, // index of the item is the id of the box
}

impl Shelf {
    /// Create a new shelf by shuffling the items
    pub fn shuffle(self, rng: &mut StdRng) -> Shelf {
        let mut shelf = self.clone();
        shuffle(&mut shelf.items, rng);
        shelf
    }

    /// Exchange the items at the given indices
    pub fn exchange_items(&mut self, idx1: usize, idx2: usize) {
        self.items.swap(idx1, idx2)
    }

    /// Remove the item at the given index
    pub fn remove_item(&mut self, idx: usize) -> Item {
        self.items.remove(idx)
    }
}
