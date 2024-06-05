use chrono::NaiveDateTime;

pub trait Item {
    fn get_description(&self) -> &str;
    fn get_price(&self) -> f64;
    fn as_tax_item(&self) -> Option<&dyn TaxItem> {
        None
    }
}

pub trait TaxItem: Item {
    fn calculate_taxes(&self, date: NaiveDateTime) -> f64;
    fn set_tax_rate(&mut self, rate: f64);
}
