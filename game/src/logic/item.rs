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