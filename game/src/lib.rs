mod utils;

use rand::rngs::StdRng;
use utils::shuffle::shuffle;

#[derive(Clone)]
pub struct Item {
    pub belongs_to: usize,
}

impl Item {
    pub fn new(belongs_to: usize) -> Item {
        Item { belongs_to }
    }
    pub fn is_belongs_to(&self, id: usize) -> bool {
        self.belongs_to == id
    }
}

#[derive(Clone)]
pub struct Shelf {
    pub items: Vec<Item>, // index of the item is the id of the box
}

impl Shelf {
    pub fn shuffle(self, rng: &mut StdRng) -> Shelf {
        let mut shelf = self.clone();
        shuffle(&mut shelf.items, rng);
        shelf
    }
    pub fn exchange_items(&mut self, idx1: usize, idx2: usize) {
        self.items.swap(idx1, idx2)
    }
    pub fn remove_item(&mut self, idx: usize) -> Item {
        self.items.remove(idx)
    }
}