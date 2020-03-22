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
//use tokio_postgres::Transaction;
use deadpool_postgres::Transaction;
/// Transaction handler provides default implementation of commit trait, along
/// with helper functions.
#[async_trait]
pub trait TransactionHandler {
    //type Error: std::convert::From<tokio_postgres::error::Error>;
    type Error: std::error::Error;

    /// Retrieve the number of results of the operation. This should match the
    /// number of updates or creates. The expecation is that the result count
    /// will be stored on the implementing struct.
    fn get_result_cnt(&self) -> u64;

    /// zero out the result count
    fn reset_result_cnt(&mut self);

    /// Given a user and comment, commit the internal transaction, returning the
    /// number of results, if successful, or an error if not.
    /// The default implementation, as provided, should suffice.
    async fn commit(
        &mut self,
        author: &str,
        comment: &str,
        tx: Transaction<'_>,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let result = self.get_result_cnt();
        self.reset_result_cnt();
        {
            tx.execute(
                "INSERT INTO REVISION (author, comment) VALUES ($1, $2)",
                &[&author, &comment],
            )
            .await?;
            tx.commit().await?;
        }

        Ok(result)
    }
}
