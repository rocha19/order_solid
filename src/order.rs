use crate::item::Item;

use rand::Rng;

#[derive(Default)]
pub struct Order {
    code: u32,
    items: Vec<Item>,
}

impl Order {
    pub fn new() -> Order {
        let mut rng = rand::thread_rng();
        let random_number: u32 = rng.gen_range(0..100_000);

        Order {
            code: random_number,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn get_subtotal(&self) -> f64 {
        self.items
            .iter()
            .fold(0.0, |total, item| total + item.price)
    }

    pub fn get_taxes(&self) -> f64 {
        self.items
            .iter()
            .fold(0.0, |taxes, item| match item.category.as_str() {
                "Cigar" => taxes + (item.price * 0.2),
                "Beer" => taxes + (item.price * 0.1),
                _ => taxes,
            })
    }

    pub fn get_total(&self) -> f64 {
        self.get_subtotal() + self.get_taxes()
    }
}
