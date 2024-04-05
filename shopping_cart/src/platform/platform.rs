
use crate::platform::{OnlineShopping, Settlement};
use crate::user::user::User;
use super::shopping::{ShoppingCart, Product};




#[derive(Debug)]
pub struct JD {
    name: String,
    account: u32,
    pub shopping_cart: ShoppingCart,
}

impl JD {
    pub fn new (name: String, account: u32) -> Self {
        Self {
            name,
            account,
            shopping_cart: ShoppingCart::new(),
        }
    }
}


impl Settlement for JD {

    fn settle_account(&mut self, user: &mut User) {
        // 模拟结算
        let products = self.shopping_cart.get_all_products();
        let user_account = user.get_money();
        let procut_price = Product::calculate_price(products);
        if user_account < procut_price {
            panic!("账户余额不足")
        } 
        self.account += procut_price;
        user.money -= procut_price;
        print!("结算成功, 交易金额为: {}, 商城 {} 当前余额: {}, 用户 {} 当前余额 {}", 
            procut_price, self.name, self.account, user.name, user.get_money()
        );

    }
}

impl OnlineShopping for JD {
    fn add_to_cart(&mut self, product: super::shopping::Product) {
        self.shopping_cart.add_product(product);
    }
}


