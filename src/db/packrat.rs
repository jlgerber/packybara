/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use crate::db::find;
//use postgres::Client;
pub use postgres::{Client, NoTls};

pub struct PackratDb {
    client: Client,
}

impl PackratDb {
    pub fn new(client: Client) -> Self {
        PackratDb { client }
    }

    /// Find the most appropriate distribution for a request. `find_distribution`
    /// returns an instance of `FindDistributionBuilder`, which provides
    /// setter methods providing a fluent api.
    ///
    /// # Arguments
    ///
    /// *`package` - The name of the package that we are interested in finding
    ///              a distribution for
    ///
    /// # Returns
    ///
    /// - `FindDistributionBuilder` - construct and execute the query to find
    /// the distribution. (see documentation for `FindDistributionBuilder`)
    ///
    /// # Example
    /// ```rust
    /// use packybara::packrat::{Client, NoTls, PackratDb};
    /// let mut client = Client::connect(
    ///    "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
    ///    NoTls,
    ///    ).unwrap();
    /// let mut db = PackratDb::new(client);
    /// let dist = db.find_distribution("maya")
    ///                     .level("dev01")
    ///                     .role("model")
    ///                     .platform("cent7_64")
    ///                     .site("portland")
    ///                     .query().unwrap();
    /// ```
    pub fn find_distribution<'b>(
        &'b mut self,
        package: &'b str,
    ) -> find::distribution::FindDistribution {
        find::distribution::FindDistribution::new(&mut self.client, package)
    }

    pub fn find_distributions<'b>(
        &'b mut self,
        package: &'b str,
    ) -> find::distributions::FindDistributions {
        find::distributions::FindDistributions::new(&mut self.client, package)
    }

    pub fn find_distribution_withs<'b>(
        &'b mut self,
        package: &'b str,
    ) -> find::distribution_withs::FindDistributionWiths {
        find::distribution_withs::FindDistributionWiths::new(&mut self.client, package)
    }
}
