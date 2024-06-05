use rand::Rng;

pub trait Item {
    fn get_description(&self) -> &str;
    fn get_price(&self) -> f64;
    fn as_tax_item(&self) -> Option<&dyn TaxItem> {
        None
    }
}

pub trait TaxItem: Item {
    fn calculate_taxes(&self) -> f64;
    fn set_tax_rate(&mut self, rate: f64);
}

pub struct Beer {
    description: String,
    price: f64,
    tax_rate: f64,
}

impl Beer {
    pub fn new(description: &str, price: f64) -> Self {
        Self {
            description: String::from(description),
            price,
            tax_rate: 0.1,
        }
    }
}

impl Item for Beer {
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

impl TaxItem for Beer {
    fn calculate_taxes(&self) -> f64 {
        self.price * self.tax_rate
    }

    fn set_tax_rate(&mut self, rate: f64) {
        self.tax_rate = rate;
    }
}

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
    fn calculate_taxes(&self) -> f64 {
        self.price * self.tax_rate
    }

    fn set_tax_rate(&mut self, rate: f64) {
        self.tax_rate = rate;
    }
}

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

    pub fn get_taxes(&self) -> f64 {
        self.items
            .iter()
            .filter_map(|item| item.as_tax_item())
            .map(|taxable_item| taxable_item.calculate_taxes())
            .sum::<f64>()
    }

    pub fn get_total(&self) -> f64 {
        self.get_subtotal() + self.get_taxes()
    }
}

fn main() {
    let mut order = Order::new();

    order.add_item(Box::new(Beer::new("Beer", 5.0)));
    order.add_item(Box::new(Cigar::new("Cigar", 10.0)));
    order.add_item(Box::new(Water::new("Water", 2.0)));

    println!("Subtotal: {}", order.get_subtotal());
    println!("Taxes: {}", order.get_taxes());
    println!("Total: {}", order.get_total());
}
