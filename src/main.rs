#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }

    fn deposit(&mut self, amount: i32) {
        let current_balance = self.balance;
        self.balance = current_balance + amount;
    }

    fn withdraw(&mut self, amount: i32) {
        let current_balance = self.balance;
        self.balance = current_balance - amount;
    }

    fn summary(&self) -> String {
        format!("{} has a balance of {}", self.holder, self.balance)
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }

    fn print_bank(&self) {
        println!("{:#?}", self);
    }
}

fn main() {
    let mut account1 = Account::new(1, String::from("Bhakul"));
    let mut account2 = Account::new(1, String::from("Bruno"));
    let mut bank = Bank::new();

    account1.deposit(148);
    account2.deposit(239);

    bank.add_account(account1);
    bank.add_account(account2);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
}
