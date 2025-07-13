// store.rs - Store struct and its methods
use crate::product::Product;
use crate::order::{Order, OrderStatus};

#[derive(Debug)]
pub struct Store {
    pub products: Vec<Product>,
    pub orders: Vec<Order>,
    pub next_order_id: u32,
}

impl Store {
    pub fn new() -> Self {
        Store {
            products: Vec::new(),
            orders: Vec::new(),
            next_order_id: 1,
        }
    }
    
    pub fn add_product(&mut self, name: &str, price: f64, stock: u32) {
        let id = self.products.len() as u32 + 1;
        let product = Product::new(id, name.to_string(), price, stock);
        self.products.push(product);
    }
    
    pub fn place_order(&mut self, customer_name: &str, product_id: u32, quantity: u32) {
        match self.products.iter_mut().find(|p| p.id == product_id) {
            Some(product) => {
                match product.reduce_stock(quantity) {
                    Ok(()) => {
                        let order = Order::new(
                            self.next_order_id,
                            customer_name.to_string(),
                            product_id,
                            quantity,
                        );
                        
                        self.orders.push(order);
                        self.next_order_id += 1;
                        
                        println!("Order placed successfully!");
                    }
                    Err(msg) => {
                        println!("Error: {}", msg);
                    }
                }
            }
            None => {
                println!("Error: Product with ID {} not found", product_id);
            }
        }
    }
    
    pub fn update_order_status(&mut self, order_id: u32, new_status: OrderStatus) {
        match self.orders.iter_mut().find(|o| o.id == order_id) {
            Some(order) => {
                order.update_status(new_status);
                println!("Order {} status updated!", order_id);
            }
            None => {
                println!("Error: Order with ID {} not found", order_id);
            }
        }
    }
    
    pub fn list_products(&self) {
        println!("=== Products ===");
        for product in &self.products {
            println!("ID: {}, Name: {}, Price: ${}, Stock: {}", 
                     product.id, product.name, product.price, product.stock);
        }
    }
    
    pub fn list_orders(&self) {
        println!("=== Orders ===");
        for order in &self.orders {
            println!("ID: {}, Customer: {}, Product ID: {}, Quantity: {}, Status: {:?}", 
                     order.id, order.customer_name, order.product_id, order.quantity, order.status);
        }
    }
}