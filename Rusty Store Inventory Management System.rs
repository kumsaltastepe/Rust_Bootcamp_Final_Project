cargo new rusty_store
cd rusty_store 

// Define the Product struct
pub struct Product {
    name: String,
    description: String,
    price: f32,
    quantity: i32,
}

impl Product {
    pub fn new(name: String, description: String, price: f32, quantity: i32) -> Self {
        Self {
            name,
            description,
            price,
            quantity,
        }
    }
}

// Define the Transaction struct
pub struct Transaction {
    product: Product,
    quantity: i32,
}

impl Transaction {
    pub fn new(product: Product, quantity: i32) -> Self {
        Self { product, quantity }
    }

    pub fn total_price(&self) -> f32 {
        self.product.price * self.quantity as f32
    }
}

// Define the Inventory struct
pub struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { products: Vec::new() }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    // Add methods for editing and deleting products
}

// Define the Sales and Purchases structs
pub struct Sales {
    transactions: Vec<Transaction>,
}

pub struct Purchases {
    transactions: Vec<Transaction>,
}

impl Sales {
    pub fn new() -> Self {
        Self { transactions: Vec::new() }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    // Add methods for calculating total sales and profit
}

impl Purchases {
    pub fn new() -> Self {
        Self { transactions: Vec::new() }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    // Add methods for calculating total cost
}

impl Inventory {
    // ...

    pub fn edit_product(&mut self, product_name: &str, new_product: Product) {
        for product in &mut self.products {
            if product.name == product_name {
                *product = new_product;
                break;
            }
        }
    }

    pub fn delete_product(&mut self, product_name: &str) {
        self.products.retain(|product| product.name != product_name);
    }
}

impl Sales {
    // ...

    pub fn total_sales(&self) -> f32 {
        self.transactions.iter().map(|transaction| transaction.total_price()).sum()
    }

    pub fn total_profit(&self) -> f32 {
        self.transactions.iter().map(|transaction| transaction.total_price() - transaction.product.price * transaction.quantity as f32).sum()
    }
}

impl Purchases {
    // ...

    pub fn total_cost(&self) -> f32 {
        self.transactions.iter().map(|transaction| transaction.total_price()).sum()
    }
}

use std::io::{self, Write};

// Define the User struct
pub struct User {
    username: String,
    password: String,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> bool {
        self.username == username && self.password == password
    }
}

// Define the Store struct
pub struct Store {
    inventory: Inventory,
    sales: Sales,
    purchases: Purchases,
    user: User,
}

impl Store {
    pub fn new(user: User) -> Self {
        Self {
            inventory: Inventory::new(),
            sales: Sales::new(),
            purchases: Purchases::new(),
            user,
        }
    }

    pub fn run(&mut self) {
        loop {
            println!("1. Manage Inventory");
            println!("2. Record Sales");
            println!("3. Record Purchases");
            println!("4. Generate Reports");
            println!("5. Exit");

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim().parse().unwrap();

            match choice {
                1 => self.manage_inventory(),
                2 => self.record_sales(),
                3 => self.record_purchases(),
                4 => self.generate_reports(),
                5 => break,
                _ => println!("Invalid choice"),
            }
        }
    }

    // Add methods for managing inventory, recording sales and purchases, generating reports
}

impl Store {
    // ...

    fn manage_inventory(&mut self) {
        // Add code here to manage inventory
        // This could include adding, editing, and deleting products
    }

    fn record_sales(&mut self) {
        // Add code here to record sales
        // This could include creating a new transaction and adding it to the sales
    }

    fn record_purchases(&mut self) {
        // Add code here to record purchases
        // This could include creating a new transaction and adding it to the purchases
    }

    fn generate_reports(&self) {
        // Add code here to generate reports
        // This could include printing out the inventory, sales, and purchases in a user-friendly format
    }
}
