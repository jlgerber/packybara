use crate::db::{add, find, find_all, update};
use crate::types::IdType;
use async_trait::async_trait;
pub use tokio_postgres::Transaction;

pub trait PBFind {
    fn find_versionpin<'b>(&'b mut self, package: &'b str) -> find::versionpin::FindVersionPin;

    fn find_all_versionpins<'b>(&'b mut self) -> find_all::versionpins::FindAllVersionPins;

    fn find_all_roles<'b>(&'b mut self) -> find_all::roles::FindAllRoles;

    fn find_all_revisions<'b>(&'b mut self) -> find_all::revisions::FindAllRevisions;

    fn find_all_changes<'b>(&'b mut self) -> find_all::changes::FindAllChanges;

    fn find_all_platforms<'b>(&'b mut self) -> find_all::platforms::FindAllPlatforms;

    fn find_all_sites<'b>(&'b mut self) -> find_all::sites::FindAllSites;

    fn find_all_levels<'b>(&'b mut self) -> find_all::levels::FindAllLevels;

    fn find_all_packages<'b>(&'b mut self) -> find_all::packages::FindAllPackages;

    fn find_all_versionpin_withs<'b>(
        &'b mut self,
        versionpin_id: IdType,
    ) -> find_all::versionpin_withs::FindAllWiths;

    fn find_versionpins<'b>(&'b mut self, package: &'b str) -> find::versionpins::FindVersionPins;

    fn find_all_distributions<'b>(&'b mut self) -> find_all::distributions::FindAllDistributions;

    fn find_pins<'b>(&'b mut self) -> find::pins::FindPins;

    fn find_pkgcoords<'b>(&'b mut self) -> find_all::pkgcoords::FindAllPkgCoords;

    fn find_withs<'b>(&'b mut self, package: &'b str) -> find::withs::FindWiths;
}
/*
pub trait PBAdd<'b> {
    type TransactionType;

    fn add_packages(tx: Self::TransactionType) -> add::packages::AddPackages<'b>;

    fn add_levels(tx: Self::TransactionType) -> add::levels::AddLevels<'b>;

    fn add_roles(tx: Self::TransactionType) -> add::roles::AddRoles<'b>;

    fn add_platforms(tx: Self::TransactionType) -> add::platforms::AddPlatforms<'b>;

    fn add_withs(tx: Self::TransactionType) -> add::withs::AddWiths<'b>;

    fn add_versionpins<I>(
        tx: Self::TransactionType,
        package: I,
        version: I,
    ) -> add::versionpins::AddVersionPins<'b>
    where
        I: Into<String>;
}
*/
pub trait PBUpdate<'a> {
    type TransactionType;
    fn update_versionpins(tx: Self::TransactionType) -> update::versionpins::UpdateVersionPins<'a>;
}

#[async_trait]
pub trait PBExport<'a> {
    type Error;

    async fn export_packages(&'a mut self, show: &'a str, path: &'a str)
        -> Result<(), Self::Error>;
}
