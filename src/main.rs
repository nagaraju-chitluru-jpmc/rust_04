mod location;
use std::collections::HashMap;

use std::fs::File;
use std::io::{BufRead, BufReader};

mod transaction;
use transaction::Transaction;

mod utils;

use crate::location::Continent;

fn main() {
    let file = File::open("./transactions.csv").unwrap();
    let reader = BufReader::new(file);

    let mut transactions: Vec<Transaction> = Vec::new();
    let mut skipped_lines: Vec<_> = Vec::new();
    let mut amountInvestedByContinent: HashMap<String, f64> = HashMap::new();

    for (idx, line) in reader.lines().enumerate() {
        if idx == 0 {
            continue;
        }

        let line_str = line.unwrap();
        let parsed_transaction = Transaction::from_csv_line(&line_str);

        match parsed_transaction {
            Ok(transaction) => transactions.push(transaction),
            Err(error_string) => skipped_lines.push((idx, error_string, line_str)),
        }
    }

    for transaction in transactions.iter() {
        println!("valid transaction: {:?}", transaction);
    }

    for skipped_line in skipped_lines.iter() {
        println!("invalid transaction: {:?}", skipped_line)
    }

    // find total amount invested by continent
    for transaction in transactions.iter() {
        let continent = transaction.continent.to_string();
        let amount = transaction.amount;
        match amountInvestedByContinent.get(&continent) {
            Some(totalAmount) => {
                amountInvestedByContinent.insert(continent, totalAmount + amount);
            }
            None => {
                amountInvestedByContinent.insert(continent, amount);
            }
        }
    }

    for (continent, amountInvested) in amountInvestedByContinent {
        println!("total investment {} in {} continent", amountInvested, continent)
    }

    utils::printTransactionsByContinent(&transactions, &Continent::Asia);
}
