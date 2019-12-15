pub mod add;
pub use add::*;
pub mod set;
pub use set::*;
pub mod find;
pub use find::*;

use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
pub struct Pb {
    /// Set the log level. This may target one or more
    /// specific modules or be general.
    /// (levels: trace, debug, info, warn, error)
    #[structopt(long)]
    pub loglevel: Option<String>,
    /// Subcommand
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub crud: PbCrud,
}

#[derive(StructOpt, Debug, PartialEq)]
#[structopt(about = "PackybaraDb CRUD")]
pub enum PbCrud {
    /// Find things in the database.
    #[structopt(display_order = 1)]
    Find {
        /// Read subcommands
        #[structopt(subcommand)]
        cmd: PbFind,
    },
    /// Update things in the database.
    #[structopt(display_order = 2)]
    Set {
        /// Read subcommands
        #[structopt(subcommand)]
        cmd: PbSet,
    },
    /// Create new things in the database.
    #[structopt(display_order = 3)]
    Add {
        /// Read subcommands
        #[structopt(subcommand)]
        cmd: PbAdd,
    },
    /// Remove things from the database.
    #[structopt(display_order = 4)]
    Delete {},
}
