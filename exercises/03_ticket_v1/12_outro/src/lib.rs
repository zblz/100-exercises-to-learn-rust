// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    fn validate_product_name(product_name: &String) {
        if product_name.is_empty() {
            panic!("empty")
        } else if product_name.len() > 300 {
            panic!("long")
        }
    }

    fn validate_quantity(quantity: &u32) {
        if *quantity <= 0 {
            panic!("Quantity 1 or more")
        }
    }

    fn validate_unit_price(unit_price: &u32) {
        if *unit_price <= 0 {
            panic!("unit_price 1 or more")
        }
    }

    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        Self::validate_quantity(&quantity);
        Self::validate_product_name(&product_name);
        Self::validate_unit_price(&unit_price);

        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn total(&self) -> u32 {
        self.unit_price * self.quantity
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_quantity(mut self, new_quantity: u32) -> Self {
        Self::validate_quantity(&new_quantity);
        self.quantity = new_quantity;
        self
    }
    pub fn set_unit_price(mut self, new_unit_price: u32) -> Self {
        Self::validate_unit_price(&new_unit_price);
        self.unit_price = new_unit_price;
        self
    }
    pub fn set_product_name(mut self, new_product_name: String) -> Self {
        Self::validate_product_name(&new_product_name);
        self.product_name = new_product_name;
        self
    }
}
