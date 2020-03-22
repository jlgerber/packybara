/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use crate::db::traits::{PBAdd, PBExport, PBFind, PBUpdate};
use crate::db::{add, find, find_all, update};
use crate::io::packages_xml::xml::write_xml;
use crate::types::IdType;
pub use deadpool_postgres::{Client, ClientWrapper, Transaction};
use snafu::{ResultExt, Snafu};
pub use tokio_postgres::NoTls;
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

pub struct PackratDb {}

impl PackratDb {
    /// New up a PackratDb instance given a Client
    pub fn new() -> Self {
        PackratDb {}
    }

    /// Commit a transaction to the database, setting the author and comment along with
    /// a transaction id
    ///
    /// # Arguments
    ///
    /// * `tx` - The postgres Transaction instance which provides a handle to a set of database transactions
    ///          which may be undone atomically
    /// * `author` - The author of the changes grouped by the transaction
    /// * `comment` - The author's explanation of the changes grouped by the transaction
    /// * `commits` - The number of commits
    ///
    /// # Returns
    ///
    /// * A future wrapping a Result, wrapping the number of upates if successful; otherwise, a PackratDbError
    pub async fn commit<'a>(
        tx: Transaction<'a>,
        author: &str,
        comment: &str,
        commits: u64,
    ) -> Result<u64, PackratDbError> {
        {
            tx.execute(
                "INSERT INTO REVISION (author, comment) VALUES ($1, $2)",
                &[&author, &comment],
            )
            .await
            .context(TokioPostgresError {
                msg: "failed to insert Revisions",
            })?;
        }

        tx.commit().await.context(TokioPostgresError {
            msg: "failed to commit transaction",
        })?;
        Ok(commits)
    }

    /// Generate a transaction for updates and adds
    pub async fn transaction<'a>(&'a self, client: &'a mut ClientWrapper) -> Transaction<'a> {
        client.transaction().await.unwrap()
    }
}

impl PBFind for PackratDb {
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
    fn find_versionpin<'b>(&'b mut self, package: &'b str) -> find::versionpin::FindVersionPin {
        find::versionpin::FindVersionPin::new(package)
    }

    fn find_versionpins<'b>(&'b mut self, package: &'b str) -> find::versionpins::FindVersionPins {
        find::versionpins::FindVersionPins::new(package)
    }

    fn find_all_versionpins<'b>(&'b mut self) -> find_all::versionpins::FindAllVersionPins {
        find_all::versionpins::FindAllVersionPins::new()
    }

    fn find_all_roles<'b>(&'b mut self) -> find_all::roles::FindAllRoles {
        find_all::roles::FindAllRoles::new()
    }

    fn find_all_revisions<'b>(&'b mut self) -> find_all::revisions::FindAllRevisions {
        find_all::revisions::FindAllRevisions::new()
    }

    fn find_all_changes<'b>(&'b mut self) -> find_all::changes::FindAllChanges {
        find_all::changes::FindAllChanges::new()
    }

    fn find_all_platforms<'b>(&'b mut self) -> find_all::platforms::FindAllPlatforms {
        find_all::platforms::FindAllPlatforms::new()
    }

    fn find_all_sites<'b>(&'b mut self) -> find_all::sites::FindAllSites {
        find_all::sites::FindAllSites::new()
    }

    fn find_all_levels<'b>(&'b mut self) -> find_all::levels::FindAllLevels {
        find_all::levels::FindAllLevels::new()
    }

    fn find_all_packages<'b>(&'b mut self) -> find_all::packages::FindAllPackages {
        find_all::packages::FindAllPackages::new()
    }
    /// find withs for a particular versionpin
    fn find_all_versionpin_withs<'b>(
        &'b mut self,
        versionpin_id: IdType,
    ) -> find_all::versionpin_withs::FindAllWiths {
        find_all::versionpin_withs::FindAllWiths::new(versionpin_id)
    }

    fn find_all_distributions<'b>(&'b mut self) -> find_all::distributions::FindAllDistributions {
        find_all::distributions::FindAllDistributions::new()
    }
    /// Find pins that meet a specific criteria
    fn find_pins<'b>(&'b mut self) -> find::pins::FindPins {
        find::pins::FindPins::new()
    }
    /// Find pkgcoords that meet a specific criteria
    fn find_pkgcoords<'b>(&'b mut self) -> find_all::pkgcoords::FindAllPkgCoords {
        find_all::pkgcoords::FindAllPkgCoords::new()
    }
    /// find withs of a
    fn find_withs<'b>(&'b mut self, package: &'b str) -> find::withs::FindWiths {
        find::withs::FindWiths::new(package)
    }
}

impl PBAdd for PackratDb {
    /// add packages
    fn add_packages() -> add::packages::AddPackages {
        add::packages::AddPackages::new()
    }

    /// add levels
    fn add_levels() -> add::levels::AddLevels {
        add::levels::AddLevels::new()
    }

    /// add roles
    fn add_roles() -> add::roles::AddRoles {
        add::roles::AddRoles::new()
    }

    /// add platforms
    fn add_platforms() -> add::platforms::AddPlatforms {
        add::platforms::AddPlatforms::new()
    }

    /// Add withs to the transaction
    fn add_withs() -> add::withs::AddWiths {
        add::withs::AddWiths::new()
    }

    fn add_versionpins<I>(package: I, version: I) -> add::versionpins::AddVersionPins
    where
        I: Into<String>,
    {
        add::versionpins::AddVersionPins::new(package.into(), version.into())
    }
}

impl PBUpdate for PackratDb {
    /// update packages
    ///
    /// # Arguments
    /// * `comment` - A comment describing the update
    /// * `user` - The name of the user making the update
    fn update_versionpins() -> update::versionpins::UpdateVersionPins {
        update::versionpins::UpdateVersionPins::new()
    }
}

use async_trait::async_trait;

#[async_trait]
impl<'a> PBExport<'a> for PackratDb {
    type Error = crate::io::packages_xml::xml::PackagesXmlError;

    async fn export_packages(
        &'a mut self,
        client: &ClientWrapper,
        show: &'a str,
        path: &'a str,
    ) -> Result<(), Self::Error> {
        write_xml(self, client, show.to_string(), path.to_string()).await
    }
}
