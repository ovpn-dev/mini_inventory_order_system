// main.rs - Entry point of our application

use mini_inventory_order_system::{Store, OrderStatus};

fn main() {
    let mut store = Store::new();
    
    // Add products
    println!("=== Adding Products ===");
    store.add_product("MacBook Pro", 1999.99, 8);
    store.add_product("AirPods", 199.99, 3);
    store.add_product("iPhone 14", 899.99, 0);
    
    // Place orders
    println!("\n=== Placing Orders ===");

    store.place_order("Alice Johnson", 1, 2);
    
    store.place_order("Bob Smith", 2, 5);
    
    store.place_order("Charlie Daniels", 3, 1);
    
    // Update order status
    println!("\n=== Updating Order Status ===");
    store.update_order_status(1, OrderStatus::Shipped);
    store.update_order_status(1, OrderStatus::Delivered);
    
    // Display everything
    println!("\n=== Final State ===");
    store.list_products();
    println!();
    store.list_orders();
}