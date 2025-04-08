// struct
// NOTE: You can only have one mutable ref to a var at a time, but you can have many
// immutable references. Keeps the data safe

fn main() {
    let mut account = BankAccount {
        owner: "John".to_string(),
        balance: 150.55,
    };

    // Immutable boorow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(50.23);
    account.check_balance();
}

// A struct is a custom data type that we as the user create, a field of var.'s
struct BankAccount {
    owner: String,
    balance: f64,
}

// This defines methods(functions) for the BankAccount struct, what you can do with/to it
impl BankAccount {
    // withdraw uses a mutable reference so we can change the value
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    // check_balance has an immutable ref to itself because we only want to read the data
    fn check_balance(&self) {
        println!("Account owned by {} has {:.2} dollars", self.owner, self.balance);
    }
}