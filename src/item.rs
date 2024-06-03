pub struct Item {
    pub category: String,
    pub description: String,
    pub price: f64,
}

impl Item {
    pub fn new(category: &str, description: &str, price: f64) -> Item {
        Item {
            category: String::from(category),
            description: String::from(description),
            price,
        }
    }
}
