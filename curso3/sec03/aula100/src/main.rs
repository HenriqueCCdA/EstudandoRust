mod account;
use account::{BankAccount, Currency, Account};

fn main() {
    let mut account = Account::new(1000.0, Currency::USD);

    account.deposit(500.0);
    println!("New balance {:.2}", account.check_balance());

    let withdraw_success = account.withdraw(1200.0);

    if withdraw_success {
        println!("Withdraw success, new balance: {:.2}", account.check_balance());
    } else {
        println!("Withdraw failed, insufficient funds");
    }

}
