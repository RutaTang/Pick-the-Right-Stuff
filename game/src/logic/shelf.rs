use super::item::Item;

#[derive(Clone)]
pub struct Shelf {
    pub items: Vec<Item>, // index of the item is the id of the box
}

impl Shelf {
    pub fn new(items_n: usize) -> Shelf {
        Shelf {
            items: (0..items_n).map(|id| Item::new(id)).collect(),
        }
    }
    /// Exchange the items at the given indices
    pub fn exchange_items(&mut self, idx1: usize, idx2: usize) {
        self.items.swap(idx1, idx2)
    }

    /// Remove the item at the given index
    pub fn remove_item(&mut self, idx: usize) -> Item {
        self.items.remove(idx)
    }

    /// Get Item Idx by item belongs
    pub fn get_item_idx_by_belongs(&self, belongs: usize) -> usize {
        self.items.iter().position(|item| item.belongs_to == belongs).unwrap()
    }
}
