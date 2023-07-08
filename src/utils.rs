use crate::location::Continent;
use crate::transaction::Transaction;

pub fn printTransactionsByContinent(transactions: &Vec<Transaction>, continent: &Continent) {
    let filteredTransactions: Vec<&Transaction> = transactions
        .iter()
        .filter(|s| s.continent == *continent)
        .collect();

    for transaction in &filteredTransactions {
        println!("{:?}", transaction)
    }
}
