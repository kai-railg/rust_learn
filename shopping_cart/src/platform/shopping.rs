use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::cmp::Eq;

#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub price: u32,
}

impl Product {
    pub fn new(name: String, price: u32) -> Self {
        Self {
            name,
            price,
        }
    }
    // 计算价格
    pub fn calculate_price(products: &HashMap<Product, u32>) -> u32 {
        let mut price = 0;
        for (product, count) in products.iter() {
            price += product.price * count ;
        }
        price
        
    }
}

// #[derive(PartialEq)]
#[derive(Debug)]
pub struct ShoppingCart {
    pub product_count: HashMap<Product, u32>,
}

impl ShoppingCart {
    pub fn new() -> Self {
        Self {
            product_count: HashMap::new(),
        }
    }
    pub fn add_product(&mut self, product: Product) {
        print!("add product: {:?}", product);

        let entry =  self.product_count.entry(product);
        match entry {
            Entry::Occupied(mut occupied) => {
                let count = occupied.get_mut();
                *count += 1;
                // self.product_count.insert(product, *count);
                // print!("add product: {}, count: {}", &product.name, *count)
            }
            Entry::Vacant(vacant) => {
                vacant.insert(1);
            }
        }
    }

    pub fn get_all_products(&self) -> &HashMap<Product, u32> {
        &self.product_count
    }

    
}