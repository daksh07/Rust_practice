fn main() {
    let mut account = BankAccount {
        owner: "Daksh".to_string(),
        balance: 200.0,
    };

    account.check_balance();
    account.withdraw(50.0);
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount{

    // In the following functions the mutable and immutable borrows have their own scope hence they
    // do not overlap, else in a single scope there can be multiple immutable borrows or single
    // mutable borrow. Also remember that for basic types int, char, bool the value is copied when
    // we use x = y and borrow happens when we use x = &y
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
        println!("Remaining balance {}", self.balance);
    }

    fn check_balance(&self){
        println!("Total balance of {} in account owned by {}", self.balance, self.owner);
    }
}
