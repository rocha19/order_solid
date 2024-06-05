use crate::item::Item;

pub struct Water {
    description: String,
    price: f64,
}

impl Water {
    pub fn new(description: &str, price: f64) -> Self {
        Self {
            description: String::from(description),
            price,
        }
    }
}

impl Item for Water {
    fn get_description(&self) -> &str {
        &self.description
    }

    fn get_price(&self) -> f64 {
        self.price
    }
}
