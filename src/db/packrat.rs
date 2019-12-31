/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use crate::db::{add, find, find_all, update};
//use postgres::Client;
use crate::types::IdType;
pub use postgres::{Client, NoTls, Transaction};
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum PackratDbError {
    /// When constructing a query, postgres has thrown an error
    #[snafu(display("Postgres Error: {} {:#?}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
    #[snafu(display("No update data supplied"))]
    NoUpdatesError,
}

pub struct PackratDb {
    client: Client,
}

impl PackratDb {
    pub fn new(client: Client) -> Self {
        PackratDb { client }
    }
    /// Generate a transaction for updates and adds
    pub fn transaction(&mut self) -> Transaction {
        self.client.transaction().unwrap()
    }
    /// commit the transaction
    pub fn commit(mut tx: Transaction, author: &str, comment: &str) -> Result<(), PackratDbError> {
        tx.execute(
            "INSERT INTO REVISION (author, comment) VALUES ($1, $2)",
            &[&author, &comment],
        )
        .context(TokioPostgresError {
            msg: "failed to update revision entity",
        })?;
        tx.commit().context(TokioPostgresError {
            msg: "failed to commit",
        })?;
        Ok(())
    }
    /// Find the most appropriate versionpin for a request. `find_versionpin`
    /// returns an instance of `FindVersionPinBuilder`, which provides
    /// setter methods providing a fluent api.
    ///
    /// # Arguments
    ///
    /// *`package` - The name of the package that we are interested in finding
    ///              a versionpin for
    ///
    /// # Returns
    ///
    /// - `FindVersionPinBuilder` - construct and execute the query to find
    /// the versionpin. (see documentation for `FindVersionPinBuilder`)
    ///
    /// # Example
    /// ```rust
    /// use packybara::packrat::{Client, NoTls, PackratDb};
    /// let mut client = Client::connect(
    ///    "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
    ///    NoTls,
    ///    ).unwrap();
    /// let mut db = PackratDb::new(client);
    /// let dist = db.find_versionpin("maya")
    ///                     .level("dev01")
    ///                     .role("model")
    ///                     .platform("cent7_64")
    ///                     .site("portland")
    ///                     .query().unwrap();
    /// ```
    pub fn find_versionpin<'b>(&'b mut self, package: &'b str) -> find::versionpin::FindVersionPin {
        find::versionpin::FindVersionPin::new(&mut self.client, package)
    }

    pub fn find_versionpins<'b>(
        &'b mut self,
        package: &'b str,
    ) -> find::versionpins::FindVersionPins {
        find::versionpins::FindVersionPins::new(&mut self.client, package)
    }

    pub fn find_all_versionpins<'b>(&'b mut self) -> find_all::versionpins::FindAllVersionPins {
        find_all::versionpins::FindAllVersionPins::new(&mut self.client)
    }

    pub fn find_all_roles<'b>(&'b mut self) -> find_all::roles::FindAllRoles {
        find_all::roles::FindAllRoles::new(&mut self.client)
    }

    pub fn find_all_revisions<'b>(&'b mut self) -> find_all::revisions::FindAllRevisions {
        find_all::revisions::FindAllRevisions::new(&mut self.client)
    }

    pub fn find_all_changes<'b>(&'b mut self) -> find_all::changes::FindAllChanges {
        find_all::changes::FindAllChanges::new(&mut self.client)
    }

    pub fn find_all_platforms<'b>(&'b mut self) -> find_all::platforms::FindAllPlatforms {
        find_all::platforms::FindAllPlatforms::new(&mut self.client)
    }

    pub fn find_all_sites<'b>(&'b mut self) -> find_all::sites::FindAllSites {
        find_all::sites::FindAllSites::new(&mut self.client)
    }

    pub fn find_all_levels<'b>(&'b mut self) -> find_all::levels::FindAllLevels {
        find_all::levels::FindAllLevels::new(&mut self.client)
    }

    pub fn find_all_packages<'b>(&'b mut self) -> find_all::packages::FindAllPackages {
        find_all::packages::FindAllPackages::new(&mut self.client)
    }
    /// find withs for a particular versionpin
    pub fn find_all_versionpin_withs<'b>(
        &'b mut self,
        versionpin_id: IdType,
    ) -> find_all::versionpin_withs::FindAllWiths {
        find_all::versionpin_withs::FindAllWiths::new(&mut self.client, versionpin_id)
    }
    pub fn find_all_distributions<'b>(
        &'b mut self,
    ) -> find_all::distributions::FindAllDistributions {
        find_all::distributions::FindAllDistributions::new(&mut self.client)
    }
    /// Find pins that meet a specific criteria
    pub fn find_pins<'b>(&'b mut self) -> find::pins::FindPins {
        find::pins::FindPins::new(&mut self.client)
    }
    /// Find pkgcoords that meet a specific criteria
    pub fn find_pkgcoords<'b>(&'b mut self) -> find_all::pkgcoords::FindAllPkgCoords {
        find_all::pkgcoords::FindAllPkgCoords::new(Some(&mut self.client))
    }
    /// find withs of a
    pub fn find_withs<'b>(&'b mut self, package: &'b str) -> find::withs::FindWiths {
        find::withs::FindWiths::new(&mut self.client, package)
    }

    /// add packages
    pub fn add_packages<'b>(&'b mut self) -> add::packages::AddPackages {
        add::packages::AddPackages::new(&mut self.client)
    }

    /// add levels
    pub fn add_levels() -> add::levels::AddLevels {
        add::levels::AddLevels::new()
    }

    /// add roles
    pub fn add_roles<'b>(&'b mut self) -> add::roles::AddRoles {
        add::roles::AddRoles::new(&mut self.client)
    }

    /// add platforms
    pub fn add_platforms<'b>(&'b mut self) -> add::platforms::AddPlatforms {
        add::platforms::AddPlatforms::new(&mut self.client)
    }

    /// add with
    pub fn add_withs<'b>(&'b mut self) -> add::withs::AddWiths {
        add::withs::AddWiths::new(&mut self.client)
    }

    /// update packages
    ///
    /// # Arguments
    /// * `comment` - A comment describing the update
    /// * `user` - The name of the user making the update
    pub fn update_versionpins(&self) -> update::versionpins::UpdateVersionPins {
        update::versionpins::UpdateVersionPins::new()
    }
}
