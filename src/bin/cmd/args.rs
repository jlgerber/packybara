/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
pub mod add;
pub use add::*;
pub mod set;
pub use set::*;
pub mod find;
pub use find::*;
pub mod export;
pub use export::*;

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
    /// Serialize state.
    #[structopt(display_order = 5)]
    Export {
        #[structopt(subcommand)]
        cmd: PbExport,
    },
}
