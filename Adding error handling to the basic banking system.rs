pub trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

pub struct BankAccount {
    holder_name: String,
    account_number: u32,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount < 0.0 {
            Err("Cannot deposit negative amount".to_string())
        } else {
            self.balance += amount;
            Ok(())
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            Err("Not enough balance".to_string())
        } else {
            self.balance -= amount;
            Ok(())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount { holder_name: "Alice".to_string(), account_number: 123456, balance: 1000.0 };
    let mut account2 = BankAccount { holder_name: "Bob".to_string(), account_number: 654321, balance: 2000.0 };

    match account1.deposit(500.0) {
        Ok(_) => println!("Deposit successful"),
        Err(err) => println!("Error: {}", err),
    }

    match account2.withdraw(2500.0) {
        Ok(_) => println!("Withdrawal successful"),
        Err(err) => println!("Error: {}", err),
    }

    println!("Balance of account1: {}", account1.balance());
    println!("Balance of account2: {}", account2.balance());
}
