use crate::item::Item;

pub struct Fruit {
    description: String,
    price: f64,
}

impl Fruit {
    pub fn new(description: &str, price: f64) -> Self {
        Self {
            description: String::from(description),
            price,
        }
    }
}

impl Item for Fruit {
    fn get_description(&self) -> &str {
        &self.description
    }

    fn get_price(&self) -> f64 {
        self.price
    }
}
