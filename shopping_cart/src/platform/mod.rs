pub mod platform;
pub mod shopping;
use crate::user::user::User;

pub trait Settlement {
    // 结算
    fn settle_account(&mut self, user: &mut User);
}

pub trait OnlineShopping {
    // 添加商品到购物车
    fn add_to_cart(&mut self, product: shopping::Product);
}


// 平台类泛型
pub struct Platform<T: Settlement + OnlineShopping> {
    pub platform: T,
}

// 平台类需要实现的 traits
impl<T: Settlement + OnlineShopping > Platform<T> {
    pub fn new(platform: T) -> Self {
        Platform { platform }
    }

    pub fn settle_account(&mut self, user: &mut User) {
        self.platform.settle_account(user);
    }

    pub fn add_to_cart(&mut self, product: shopping::Product) {
        self.platform.add_to_cart(product);
    }


}