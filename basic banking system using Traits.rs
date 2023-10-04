// Define the Account trait
pub trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

// Define the BankAccount struct
pub struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

// Implement the Account trait for BankAccount
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
        } else {
            println!("Insufficient balance!");
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Create two BankAccount instances
    let mut account1 = BankAccount { account_number: 123456, holder_name: String::from("Alice"), balance: 1000.0 };
    let mut account2 = BankAccount { account_number: 789012, holder_name: String::from("Bob"), balance: 2000.0 };

    // Call the deposit method on account1
    account1.deposit(500.0);

    // Call the withdraw method on account2
    account2.withdraw(300.0);

    // Call the balance method on both accounts and print the result
    println!("Balance of {}: {}", account1.holder_name, account1.balance());
    println!("Balance of {}: {}", account2.holder_name, account2.balance());
}
