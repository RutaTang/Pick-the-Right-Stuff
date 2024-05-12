/// Locker is a struct that holds a list of items.
#[derive(Clone)]
pub struct Locker {
    pub items: Vec<Option<Item>>, // index of the item is the id of the box
}

impl Locker {
    pub fn new(items_n: usize) -> Locker {
        Locker {
            items: (0..items_n).map(|_| Some(Item::new(0))).collect(),
        }
    }
    /// Exchange the items at the given indices
    pub fn exchange_items(&mut self, idx1: usize, idx2: usize) {
        self.items.swap(idx1, idx2)
    }

    /// Remove the item at the given index
    pub fn remove_item(&mut self, idx: usize) -> Option<Item> {
        self.items[idx].take()
    }

    /// Get Item Idx by item belongs id
    pub fn get_item_idx_by_belongs(&self, belongs: usize) -> usize {
        self.items
            .iter()
            .position(|item| item.is_some() && item.as_ref().unwrap().belongs_to == belongs)
            .unwrap()
    }
}

/// Item is a struct that holds the id of the user it belongs to.
#[derive(Clone)]
pub struct Item {
    /// id of the user it belongs to
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
