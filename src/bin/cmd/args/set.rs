use packybara::types::IdType;
use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
#[structopt(about = "Set entities in db")]
pub enum PbSet {
    /// Update one or more existing VersionPins
    #[structopt(display_order = 1)]
    VersionPins {
        /// provide one or more ids of the versionpins(s) we are updating
        #[structopt(short, long = "vpin-id", display_order = 1)]
        vpin_ids: Vec<IdType>,
        /// provide on or more distribution ids to update to. Each id provided
        /// by the --vpin-id must have a corresponding id set in --dist-id. They
        /// form pairs.
        #[structopt(short, long = "dist-id", display_order = 2)]
        dist_ids: Vec<IdType>,
    },
}
