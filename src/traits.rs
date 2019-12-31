use crate::packrat::{PackratDb, PackratDbError};
use postgres::Transaction;

pub trait TransactionHandler {
    fn take_tx(&mut self) -> Option<Transaction>;
    fn get_result_cnt(&self) -> u64;
    /// Commit's default implementation should suffice
    fn commit(&mut self, user: &str, comment: &str) -> Result<u64, PackratDbError> {
        let tx = self.take_tx().expect("transaction missing");
        PackratDb::commit(tx, user, comment)?;
        Ok(self.get_result_cnt())
    }
}
