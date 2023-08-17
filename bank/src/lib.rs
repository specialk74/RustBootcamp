pub struct SavingAccount {
    balance: i32,
}

impl  SavingAccount {
    pub fn new() -> SavingAccount {
        SavingAccount {
            balance: 0,
        }
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_have_a_starting_balance_of_0() {
        let account = SavingAccount::new();
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = SavingAccount::new();
        account.deposit(100);
        assert_eq!(account.get_balance(), 100);
    }
}
