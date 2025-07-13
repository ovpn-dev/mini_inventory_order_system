// order.rs - Order struct and OrderStatus enum
#[derive(Debug, Clone)]
pub enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug)]
pub struct Order {
    pub id: u32,
    pub customer_name: String,
    pub product_id: u32,
    pub quantity: u32,
    pub status: OrderStatus,
}

impl Order {
    pub fn new(id: u32, customer_name: String, product_id: u32, quantity: u32) -> Self {
        Order {
            id,
            customer_name,
            product_id,
            quantity,
            status: OrderStatus::Pending,
        }
    }
    
    pub fn update_status(&mut self, new_status: OrderStatus) {
        self.status = new_status;
    }
    
    pub fn print_status(&self) {
        println!("Order {} status: {:?}", self.id, self.status);
    }
}