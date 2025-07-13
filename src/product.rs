// product.rs - Product struct and its methods
#[derive(Debug)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub stock: u32,
}

impl Product {
    pub fn new(id: u32, name: String, price: f64, stock: u32) -> Self {
        Product {
            id,
            name,
            price,
            stock,
        }
    }
    
    pub fn is_in_stock(&self, quantity: u32) -> bool {
        self.stock >= quantity
    }
    
    pub fn reduce_stock(&mut self, quantity: u32) -> Result<(), String> {
        if self.is_in_stock(quantity) {
            self.stock -= quantity;
            Ok(())
        } else {
            Err("Not enough stock".to_string())
        }
    }
}