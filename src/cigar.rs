use chrono::NaiveDateTime;

use crate::item::{Item, TaxItem};

pub struct Cigar {
    description: String,
    price: f64,
    tax_rate: f64,
}

impl Cigar {
    pub fn new(description: &str, price: f64) -> Self {
        Self {
            description: String::from(description),
            price,
            tax_rate: 0.2,
        }
    }
}

impl Item for Cigar {
    fn get_description(&self) -> &str {
        &self.description
    }

    fn get_price(&self) -> f64 {
        self.price
    }

    fn as_tax_item(&self) -> Option<&dyn TaxItem> {
        Some(self)
    }
}

impl TaxItem for Cigar {
    fn calculate_taxes(&self, _: NaiveDateTime) -> f64 {
        self.price * self.tax_rate
    }

    fn set_tax_rate(&mut self, rate: f64) {
        self.tax_rate = rate;
    }
}
