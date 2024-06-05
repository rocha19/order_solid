#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use order_solid::{beer::Beer, cigar::Cigar, fruit::Fruit, order::Order, water::Water};

    #[test]
    fn calculate_order_subtotal() {
        let mut order = Order::new();

        order.add_item(Box::new(Fruit::new("Apple", 5.0)));
        order.add_item(Box::new(Fruit::new("Banana", 2.0)));
        order.add_item(Box::new(Fruit::new("Orange", 3.0)));

        let subtotal = order.get_subtotal();
        assert_eq!(subtotal, 10.0); // Now expects 10.0 (fruit prices)
    }

    #[test]
    fn calculate_order_taxes() {
        let mut order = Order::new();

        order.add_item(Box::new(Cigar::new("Derby", 10.0)));
        order.add_item(Box::new(Beer::new("Sckol", 5.0)));
        order.add_item(Box::new(Water::new("Santa Joana 500ml", 2.0)));

        let today =
            NaiveDateTime::parse_from_str("2024-06-05T08:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let taxes = order.get_taxes(today);
        assert_eq!(taxes, 2.5);
    }

    #[test]
    fn calculate_order_holiday_taxes() {
        let mut order = Order::new();

        order.add_item(Box::new(Cigar::new("Derby", 10.0)));
        order.add_item(Box::new(Beer::new("Sckol", 5.0)));
        order.add_item(Box::new(Water::new("Santa Joana 500ml", 2.0)));

        let today =
            NaiveDateTime::parse_from_str("2024-01-01T08:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let taxes = order.get_taxes(today);
        assert_eq!(taxes, 3.5);
    }

    #[test]
    fn calculate_order_total() {
        let mut order = Order::new();

        order.add_item(Box::new(Cigar::new("Derby", 10.0)));
        order.add_item(Box::new(Beer::new("Sckol", 5.0)));
        order.add_item(Box::new(Water::new("Santa Joana 500ml", 2.0)));

        let today =
            NaiveDateTime::parse_from_str("2024-06-05T08:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let total = order.get_total(today);
        assert_eq!(total, 19.5);
    }
}
