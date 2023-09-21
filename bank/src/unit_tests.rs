pub fn print_and_return_value(value: i32) -> i32 {
    println!("I got the value: {}", value);
    value
}

/// A savings account
pub struct SavingAccounts {
    balance: i32,
}

impl SavingAccounts {
    /// Creates a 'SavingsAccount' with a balance of 0
    ///
    /// # Examples
    ///
    /// ```
    /// use documentation::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get_balance(), 0);
    /// ```
    pub fn new() -> SavingAccounts {
        SavingAccounts { balance: 0 }
    }

    pub fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Cannot deposit negative amount");
        } // no good practice, only for demostrate panic
        self.balance += amount;
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn transfer(&mut self, acc_number: i32, amount: i32) -> Result<String, String> {
        Ok(format!("Transfered ${amount} to ${acc_number}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_a_starting_balance_of_0() {
        let account = SavingAccounts::new();
        print_and_return_value(account.get_balance());
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = SavingAccounts::new();
        account.deposit(100);
        print_and_return_value(account.get_balance());
        assert_eq!(account.get_balance(), 100, "Balance should be 100");
        // assert_ne!(account.get_balance(), 0);
        // assert!(account.get_balance() == 100);
    }

    #[test]
    #[should_panic]
    fn should_be_panic_if_deposit_negative_amount() {
        let mut account = SavingAccounts::new();
        account.deposit(-1);
    }

    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let mut account = SavingAccounts::new();
        account.deposit(100);
        account.transfer(10, 50)?;
        print_and_return_value(account.get_balance());
        Ok(())
    }
}
