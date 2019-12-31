use crate::packrat::PackratDb;
pub use crate::packrat::PackratDbError;
use postgres::Transaction;
/// Transaction handler provides default implementation of commit trait, along
/// with helper functions.
pub trait TransactionHandler<'a> {
    type Error: std::convert::From<tokio_postgres::error::Error>;
    // /// retrieve an Option<Transaction>. The expectation here is that the
    // /// implementer (a struct) will have a tx: Option<Transaction> field and
    // /// the impl will return self.tx.take()
    // fn take_tx(&mut self) -> Option<Transaction>;
    fn tx(&mut self) -> Option<&mut Transaction<'a>>;

    fn take_tx(&mut self) -> Transaction<'a>;
    /// Retrieve the number of results of the operation. This should match the
    /// number of updates or creates. The expecation is that the result count
    /// will be stored on the implementing struct.
    fn get_result_cnt(&self) -> u64;

    /// zero out the result count
    fn reset_result_cnt(&mut self);
    /// Given a user adn comment, commit the internal transaction, returning the
    /// number of results, if successful, or an error if not.
    /// The default implementation, as provided, should suffice.
    fn commit(&mut self, author: &str, comment: &str) -> Result<u64, Self::Error> {
        {
            //let tx = self.tx();
            {
                self.tx().unwrap().execute(
                    "INSERT INTO REVISION (author, comment) VALUES ($1, $2)",
                    &[&author, &comment],
                )?;
            }

            self.take_tx().commit()?;
        }

        let result = self.get_result_cnt();
        self.reset_result_cnt();
        Ok(result)
    }
}
