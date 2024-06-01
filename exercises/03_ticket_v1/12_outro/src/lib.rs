// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.

use core::panic;
use std::u32;

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

// Validators
pub fn validate_name(name: &String) {
    if name.is_empty() {
        panic!("Product name cannot be empty");
    }
    if name.len() > 300 {
        panic!("Product name cannot be longer than 300 bytes");
    }
}

pub fn validate_quantity(quantity: &u32) {
    if *quantity == 0 {
        panic!("Quantity must be greater than 0");
    }
}

pub fn validate_price(price: &u32) {
    if *price == 0 {
        panic!("Price must be greater than 0");
    }
}

// Methods
impl Order {
    // new
    pub fn new(name: String, quant: u32, price: u32) -> Order {
        validate_price(&price);
        validate_quantity(&quant);
        validate_name(&name);
        Order {
            product_name: name.into(),
            quantity: quant.into(),
            unit_price: price.into(),
        }
    }

    // Getters
    pub fn product_name(&self) -> String {
        self.product_name.clone()
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    // Setters
    pub fn set_product_name(&mut self, name: String) {
        validate_name(&name);
        self.product_name = name;
    }
    pub fn set_quantity(&mut self, quant: u32) {
        validate_quantity(&quant);
        self.quantity = quant;
    }
    pub fn set_unit_price(&mut self, price: u32) {
        validate_price(&price);
        self.unit_price = price;
    }

    // Total
    pub fn total(&self) -> u32 {
        &self.unit_price * &self.quantity
    }
}

// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
