#[cfg(test)]
mod tests {
    use order_solid::{item::Item, order::Order};

    #[test]
    fn calculate_order_subtotal() {
        let mut order = Order::new();

        order.add_item(Item::new("Fruit", "Apple", 5.0));
        order.add_item(Item::new("Fruit", "Banana", 2.0));
        order.add_item(Item::new("Fruit", "Orange", 3.0));

        let subtotal = order.get_subtotal();
        assert_eq!(subtotal, 10.0);
    }

    #[test]
    fn calculate_order_taxes() {
        let mut order = Order::new();

        order.add_item(Item::new("Cigar", "Derby", 10.0));
        order.add_item(Item::new("Beer", "Sckol", 5.0));
        order.add_item(Item::new("Water", "Santa Joana 500ml", 2.0));

        let taxes = order.get_taxes();
        assert_eq!(taxes, 2.5);
    }

    #[test]
    fn calculate_order_total() {
        let mut order = Order::new();

        order.add_item(Item::new("Cigar", "Derby", 10.0));
        order.add_item(Item::new("Beer", "Sckol", 5.0));
        order.add_item(Item::new("Water", "Santa Joana 500ml", 2.0));

        let total = order.get_total();
        assert_eq!(total, 19.5);
    }
}
