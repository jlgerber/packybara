use packybara::types::IdType;
use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
#[structopt(about = "PackybaraDb Add")]
pub enum PbAdd {
    /// Add one or more packages to the database.
    #[structopt(display_order = 1, name = "package")]
    Packages {
        #[structopt(name = "PACKAGE")]
        names: Vec<String>,
    },
    /// Add one or more levels to the database.
    #[structopt(display_order = 2, name = "level")]
    Levels {
        #[structopt(name = "LEVEL")]
        names: Vec<String>,
    },
    /// Add one or more roles to the database.
    #[structopt(display_order = 3, name = "role")]
    Roles {
        #[structopt(name = "ROLE")]
        names: Vec<String>,
    },
    /// Add one or more roles to the database.
    #[structopt(display_order = 4, name = "platform")]
    Platforms {
        #[structopt(name = "PLATFORM")]
        names: Vec<String>,
    },
    /// Add one or more roles to the database.
    #[structopt(display_order = 5, name = "withs")]
    Withs {
        #[structopt(short, long = "versionpin-id", display_order = 1)]
        vpin_id: IdType,
        #[structopt(short, long, display_order = 2)]
        comment: String,
        #[structopt(name = "WITHS")]
        withs: Vec<String>,
    },
}
