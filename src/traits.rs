/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
//! reexport the db traits
pub use crate::db::traits::*;
pub use crate::packrat::PackratDbError;
use async_trait::async_trait;
use tokio_postgres::Transaction;
/// Transaction handler provides default implementation of commit trait, along
/// with helper functions.
#[async_trait]
pub trait TransactionHandler {
    type Error: std::convert::From<tokio_postgres::error::Error>;
    // type Error: std::error::Error;
    // /// retrieve an Option<&mut Transaction>. The expectation here is that the
    // /// implementer (a struct) will have a tx: Option<Transaction> field and
    // /// the impl will return self.tx.as_mut()
    async fn tx(&mut self) -> Option<&mut Transaction<'_>>;

    /// take the transaction from the impl. The expectation is that we have a field
    /// self.tx = Option<Transaction<'a>> that can be taken via self.tx.take().
    async fn take_tx(&mut self) -> Transaction<'_>;

    /// Retrieve the number of results of the operation. This should match the
    /// number of updates or creates. The expecation is that the result count
    /// will be stored on the implementing struct.
    fn get_result_cnt(&self) -> u64;

    /// zero out the result count
    fn reset_result_cnt(&mut self);

    /// Given a user and comment, commit the internal transaction, returning the
    /// number of results, if successful, or an error if not.
    /// The default implementation, as provided, should suffice.
    async fn commit(&mut self, author: &str, comment: &str) -> Result<u64, Self::Error> {
        {
            {
                self.tx()
                    .await
                    .unwrap()
                    .execute(
                        "INSERT INTO REVISION (author, comment) VALUES ($1, $2)",
                        &[&author, &comment],
                    )
                    .await?;
            }

            self.take_tx().await.commit().await?;
        }

        let result = self.get_result_cnt();
        self.reset_result_cnt();
        Ok(result)
    }
}
