🧑‍💻 Rust Assignment: Build a Real-World Inventory & Order System

### 🎯 Objective

You're tasked with building a mini inventory and order system for an online store using Rust. This system will track products, allow customers to place orders, update product stock, and handle different order statuses.

---

## 📦 Part 1: Product Inventory

### ✅ Requirements:

1. Define a Product struct with:

   - id: u32
   - name: String
   - price: f64
   - stock: u32

2. Implement methods for Product:

   - is_in_stock(&self, quantity: u32) -> bool
   - reduce_stock(&mut self, quantity: u32) -> Result<(), String>

---

## 📬 Part 2: Order Management

### ✅ Define:

1. An OrderStatus enum with the following variants:

   - Pending
   - Shipped
   - Delivered
   - Cancelled

2. An Order struct with:

   - id: u32
   - customer_name: String
   - product_id: u32
   - quantity: u32
   - status: OrderStatus

3. Implement methods on Order:

   - update_status(&mut self, new_status: OrderStatus)
   - print_status(&self)

---

## 🏪 Part 3: Store System

### ✅ Define a Store struct that holds:

- A list of products (Vec<Product>)
- A list of orders (Vec<Order>)
- A field for tracking the next order_id

### ✅ Implement methods on Store:

1. add_product(&mut self, name: &str, price: f64, stock: u32)
2. place_order(&mut self, customer_name: &str, product_id: u32, quantity: u32)

   - If the product exists and has enough stock, reduce the stock and create a new order.
   - Otherwise, print an error message.

3. update_order_status(&mut self, order_id: u32, new_status: OrderStatus)
4. list_products(&self)
5. list_orders(&self)

---

## 🚀 Part 4: Simulate in main()

Write a main() function to:

1. Add at least 3 products to the store.
2. Place at least 3 orders, including:

   - A successful order
   - An order with insufficient stock
   - An order for a product with zero stock

3. Update the status of at least one order from Pending → Shipped → Delivered.
4. Print all products and orders at the end.

---

## 🧠 Bonus Challenges (Optional)

- Add a Customer struct and associate orders with customers.
- Add an enum PaymentStatus (e.g., Paid, Unpaid, Refunded).
- Let users search for products by name.`. `For ease there is a mock json

JSON

{
"products": [
{
"id": 1,
"name": "MacBook Pro",
"price": 1999.99,
"stock": 8
},
{
"id": 2,
"name": "AirPods",
"price": 199.99,
"stock": 3
},
{
"id": 3,
"name": "iPhone 14",
"price": 899.99,
"stock": 0
}
],
"orders": [
{
"id": 101,
"customer_name": "Alice Johnson",
"product_id": 1,
"quantity": 2,
"status": "Shipped"
},
{
"id": 102,
"customer_name": "Bob Smith",
"product_id": 2,
"quantity": 5,
"status": "Cancelled"
},
{
"id": 103,
"customer_name": "Charlie Daniels",
"product_id": 3,
"quantity": 1,
"status": "Pending"
}
],
"next_order_id": 104}
