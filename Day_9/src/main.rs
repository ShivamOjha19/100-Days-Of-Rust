use std::collections::HashMap;
struct Bank {
    accounts: HashMap<String, u32>,
}
impl Bank {
    fn new() -> Bank {
        Bank {
            accounts: HashMap::new(),
        }
    }

    fn create_account(&mut self, name: String, initial_balance: u32) {
        self.accounts.insert(name, initial_balance);
    }

    fn transfer(&mut self, from: &str, to: &str, amount: u32) {
        let from_balance = match self.accounts.get(from) {
            Some(balance) => *balance,
            None => return,
        };

        if from_balance >= amount {
            if let Some(sender_balance) = self.accounts.get_mut(from) {
                *sender_balance -= amount;
            }
            if let Some(receiver_balance) = self.accounts.get_mut(to) {
                *receiver_balance += amount;
            }
        }
    }

    fn get_balance(&self, name: &str) -> Option<u32> {
        self.accounts.get(name).copied()
    }
}
fn main() {
    let mut bank = Bank::new();

    bank.create_account("Alice".to_string(), 100);
    bank.create_account("Bob".to_string(), 50);

    println!("Before transfer:");
    println!("Alice: ${}", bank.get_balance("Alice").unwrap());
    println!("Bob: ${}", bank.get_balance("Bob").unwrap());

    bank.transfer("Alice", "Bob", 30);

    println!("After transfer:");
    println!("Alice: ${}", bank.get_balance("Alice").unwrap());
    println!("Bob: ${}", bank.get_balance("Bob").unwrap());
}