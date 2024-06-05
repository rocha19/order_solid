use chrono::{Datelike, NaiveDateTime};
use rand::Rng;

use crate::item::Item;

#[allow(dead_code)]
#[derive(Default)]
pub struct Order {
    code: u32,
    items: Vec<Box<dyn Item>>,
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

    pub fn add_item(&mut self, item: Box<dyn Item>) {
        self.items.push(item);
    }

    pub fn get_subtotal(&self) -> f64 {
        self.items.iter().map(|item| item.get_price()).sum::<f64>()
    }

    pub fn get_taxes(&self, date: chrono::NaiveDateTime) -> f64 {
        self.items
            .iter()
            .filter_map(|item| item.as_tax_item())
            .map(|taxable_item| taxable_item.calculate_taxes(date))
            .sum::<f64>()
    }

    pub fn get_total(&self, date: NaiveDateTime) -> f64 {
        self.get_subtotal() + self.get_taxes(date)
    }
}
