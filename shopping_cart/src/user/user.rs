use crate::platform::shopping::{ShoppingCart, Product};


#[derive(Debug)]
pub enum Gender {
    Male,
    Female
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub gender:  Gender,
    pub money: u32,
}

impl User {
    pub fn new (name: String, age: u32, gender: Gender, money: u32) -> Self {
        Self {
            name,
            age,
            gender,
            money,
        }
    }
    pub fn get_money(&self) -> u32 {
        self.money
    }
    pub fn select_product(&mut self, product: Product, shopping_cart: &mut ShoppingCart, count: u32) {
        for _ in 0..count {
            shopping_cart.add_product(product.clone());
        }
    }
}

