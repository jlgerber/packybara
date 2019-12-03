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
    pub cmd: PbSub,
}

#[derive(StructOpt, Debug, PartialEq)]
#[structopt(about = "PackybaraDb CRUD")]
pub enum PbSub {
    /// Find the specific versionpin whose coords are closest to
    /// the supplied coords (level,role,platform,site)
    #[structopt(display_order = 1)]
    VersionPin {
        /// The name of the package to search for
        #[structopt()]
        package: String,
        /// Levelspec format show[.seq[.shot]]. Defaults to 'facility'.
        #[structopt(short = "L", long, display_order = 1)]
        level: Option<String>,
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short = "R", long, display_order = 2)]
        role: Option<String>,
        /// OS - (eg cent7_64)
        #[structopt(short = "P", long, display_order = 3)]
        platform: Option<String>,
        /// The site - defaults to 'any'
        #[structopt(short = "S", long, display_order = 4)]
        site: Option<String>,
        /// Search mode - ancestor (or down), exact, descendant (or up)
        #[structopt(long = "search", display_order = 5)]
        search_mode: Option<String>,
        /// limit the number of returned items
        #[structopt(short, long, display_order = 6)]
        limit: Option<i32>,
        /// provide one or more comma separated items to order the return by.
        #[structopt(short, long = "order-by", display_order = 7)]
        order_by: Option<String>,
        /// do not truncate the withs if true. defaults to false
        #[structopt(short = "w", long = "withs", display_order = 8)]
        full_withs: bool,
    },
    #[structopt(display_order = 2)]
    /// Find all distributions that meet criteria.
    VersionPins {
        /// The name of the package
        #[structopt(short, long, display_order = 1)]
        package: Option<String>,
        /// Levelspec format show[.seq[.shot]]. Defaults to 'facility'.
        #[structopt(short = "L", long, display_order = 2)]
        level: Option<String>,
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short = "R", long, display_order = 3)]
        role: Option<String>,
        /// OS - (eg cent7_64)
        #[structopt(short = "P", long, display_order = 4)]
        platform: Option<String>,
        /// The site - defaults to 'any'
        #[structopt(short = "S", long, display_order = 5)]
        site: Option<String>,
        /// Search mode - ancestor (or down), exact, descendant (or up)
        #[structopt(short, long = "search", display_order = 6)]
        search_mode: Option<String>,
        /// limit the number of returned items
        #[structopt(short, long, display_order = 7)]
        limit: Option<i32>,
        /// provide one or more comma separated items to order the return by.
        #[structopt(short, long = "order-by", display_order = 8)]
        order_by: Option<String>,
        /// do not truncate the withs if true. defaults to false
        #[structopt(short = "w", long = "withs", display_order = 9)]
        full_withs: bool,
    },
    #[structopt(display_order = 3)]
    /// Find the specific distribution's specific withs,
    /// whose coords are closest to the supplied coords
    /// (level,role,platform,site)
    Withs {
        /// The name of the package to search for
        #[structopt()]
        package: String,
        /// Levelspec format show[.seq[.shot]]. Defaults to 'facility'.
        #[structopt(short = "L", long, display_order = 1)]
        level: Option<String>,
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short = "R", long, display_order = 2)]
        role: Option<String>,
        /// OS - (eg cent7_64)
        #[structopt(short = "P", long, display_order = 3)]
        platform: Option<String>,
        /// The site - defaults to 'any'
        #[structopt(short = "S", long, display_order = 4)]
        site: Option<String>,
        /// Search mode - ancestor (or down), exact, descendant (or up)
        #[structopt(short, long = "search", display_order = 5)]
        search_mode: Option<String>,
        /// limit the number of returned items
        #[structopt(short, long, display_order = 6)]
        limit: Option<i32>,
        /// provide one or more comma separated items to order the return by.
        #[structopt(short, long = "order-by", display_order = 7)]
        order_by: Option<String>,
    },
    #[structopt(display_order = 4)]
    /// Search for roles. Discover what roles are being used
    /// in relation to other coordinates. Or just get a list.
    Roles {
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short = "R", long, display_order = 1)]
        role: Option<String>,
        /// Levelspec format show[.seq[.shot]]. Defaults to 'facility'.
        #[structopt(short = "L", long, display_order = 2)]
        level: Option<String>,
        /// OS - (eg cent7_64)
        #[structopt(short = "P", long, display_order = 3)]
        platform: Option<String>,
        /// The site - defaults to 'any'
        #[structopt(short = "S", long, display_order = 4)]
        site: Option<String>,
        /// Search mode - ancestor (or down), exact, descendant (or up)
        #[structopt(short, long = "search", display_order = 5)]
        search_mode: Option<String>,
        /// limit the number of returned items
        #[structopt(short, long, display_order = 6)]
        limit: Option<i32>,
        /// provide one or more comma separated items to order the return by.
        #[structopt(short, long = "order-by", display_order = 7)]
        order_by: Option<String>,
    },
    #[structopt(display_order = 5)]
    /// Get a simple list of all roles
    AllRoles {
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short = "R", long, display_order = 1)]
        role: Option<String>,
        ///  One of: role, subrole, any
        #[structopt(short = "C", long, display_order = 2)]
        category: Option<String>,
        /// provide one or more comma separated items to order the return by.
        #[structopt(short, long = "order-by", display_order = 3)]
        order_by: Option<String>,
    },
    #[structopt(display_order = 6)]
    /// Get a simple list of all platforms
    AllPlatforms {
        /// The platform (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short = "P", long, display_order = 1)]
        platform: Option<String>,
        ///  platform or nutin'
        #[structopt(short, long = "order-by", display_order = 2)]
        order_by: Option<String>,
    },
    #[structopt(display_order = 7)]
    /// Get a simple list of all sites
    AllSites {
        /// The site (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short = "S", long)]
        site: Option<String>,
    },
    #[structopt(display_order = 8)]
    /// Get a simple list of all levels
    AllLevels {
        /// The level. Defaults to 'any'.
        #[structopt(short = "L", long, display_order = 1)]
        level: Option<String>,
        ///  name of the show
        #[structopt(short = "S", long, display_order = 2)]
        show: Option<String>,
        /// provide one or more comma separated items to order the return by.
        #[structopt(short, long = "order-by", display_order = 3)]
        order_by: Option<String>,
    },
}
