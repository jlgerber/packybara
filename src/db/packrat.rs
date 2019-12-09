/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use crate::db::{add, find, find_all};
//use postgres::Client;
pub use postgres::{Client, NoTls};

pub struct PackratDb {
    client: Client,
}

impl PackratDb {
    pub fn new(client: Client) -> Self {
        PackratDb { client }
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

    pub fn find_all_distributions<'b>(
        &'b mut self,
    ) -> find_all::distributions::FindAllDistributions {
        find_all::distributions::FindAllDistributions::new(&mut self.client)
    }
    /// Find pins that meet a specific criteria
    pub fn find_pins<'b>(&'b mut self) -> find::pins::FindPins {
        find::pins::FindPins::new(&mut self.client)
    }

    pub fn find_withs<'b>(&'b mut self, package: &'b str) -> find::withs::FindWiths {
        find::withs::FindWiths::new(&mut self.client, package)
    }

    /// Find pins that meet a specific criteria
    pub fn add_packages<'b>(&'b mut self) -> add::packages::AddPackages {
        add::packages::AddPackages::new(&mut self.client)
    }

    /// add
    pub fn add_levels<'b>(&'b mut self) -> add::levels::AddLevels {
        add::levels::AddLevels::new(&mut self.client)
    }

    /// add
    pub fn add_roles<'b>(&'b mut self) -> add::roles::AddRoles {
        add::roles::AddRoles::new(&mut self.client)
    }

    /// add
    pub fn add_platforms<'b>(&'b mut self) -> add::platforms::AddPlatforms {
        add::platforms::AddPlatforms::new(&mut self.client)
    }
}
