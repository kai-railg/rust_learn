
mod user;
use user::user::{User, Gender};

mod platform;
use platform::shopping::Product;
use platform::platform::JD;
use platform::Platform;

use crate::platform::Settlement;

// 简单的购物车
fn main() {
    let mut user = User::new("XiaoMing".to_string(), 18, Gender::Male, 200);
    let product_apple = Product::new("apple".to_string(), 10);
    let product_banana = Product::new("banana".to_string(), 5);
    let product_beef = Product::new("beef".to_string(), 50);
    let mut plat_t = Platform::new(
        JD::new("JD".to_string(), 1000));
    {
        let platform = &mut plat_t.platform;
        let shopping_cart = &mut platform.shopping_cart;
        for product in vec![product_apple, product_banana, product_beef] {
            user.select_product(product, shopping_cart, 2);
        };
    };
    {
        let shop = &mut plat_t.platform;
        shop.settle_account(&mut user);
    };
}   
