use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
#[structopt(about = "Find entities in db")]
pub enum PbFind {
    /// Find the versionpin whose pin coords are closest to
    /// the supplied pin coords (level, role, platform, site).
    #[structopt(display_order = 1)]
    VersionPin {
        /// The name of the package to search for.
        #[structopt(name = "PACKAGE")]
        package: String,
        /// The level, which may be 'facility' or a Levelspec (ie show[.seq[.shot]]). Defaults to 'facility'.
        #[structopt(short = "L", long, display_order = 1)]
        level: Option<String>,
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short = "R", long, display_order = 2)]
        role: Option<String>,
        /// The operating system name (eg cent7_64). Defaults to 'any'.
        #[structopt(short = "P", long, display_order = 3)]
        platform: Option<String>,
        /// The location name (eg portland). Defaults to 'any'.
        #[structopt(short = "S", long, display_order = 4)]
        site: Option<String>,
        /// Search mode - ancestor (or down), exact, descendant (or up). Defauts to 'ancestor'.
        #[structopt(long = "search", display_order = 5)]
        search_mode: Option<String>,
        /// Limit the number of returned items.
        #[structopt(short, long, display_order = 6)]
        limit: Option<i32>,
        /// Provide one or more comma separated items to order the return by.
        #[structopt(short, long = "order-by", display_order = 7)]
        order_by: Option<String>,
        /// Do not truncate the withs if true. Defaults to false.
        #[structopt(short = "w", long = "withs", display_order = 8)]
        full_withs: bool,
    },
    #[structopt(display_order = 2)]
    /// Find all versionpins that meet supplied name and pin coordinate criteria.
    VersionPins {
        /// The name of the package.
        //#[structopt(short, long, display_order = 1)]
        #[structopt(name = "PACKAGE")]
        package: Option<String>,
        /// Levelspec format show[.seq[.shot]]. Defaults to 'facility'.
        #[structopt(short = "L", long, display_order = 1)]
        level: Option<String>,
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short = "R", long, display_order = 2)]
        role: Option<String>,
        /// The operating system name (eg cent7_64).
        #[structopt(short = "P", long, display_order = 3)]
        platform: Option<String>,
        /// The location name (eg portland). Defaults to 'any'.
        #[structopt(short = "S", long, display_order = 4)]
        site: Option<String>,
        /// The search mode - ancestor (or down), exact, descendant (or up).
        #[structopt(short, long = "search", display_order = 5)]
        search_mode: Option<String>,
        /// Limit the number of returned items.
        #[structopt(short, long, display_order = 6)]
        limit: Option<i32>,
        /// Provide one or more comma separated items to order the return by.
        #[structopt(short, long = "order-by", display_order = 7)]
        order_by: Option<String>,
        /// Do not truncate the withs if true. Defaults to false.
        #[structopt(short = "w", long = "withs", display_order = 8)]
        full_withs: bool,
    },
    #[structopt(display_order = 3)]
    /// Find a distribution's withs' distributions,
    /// based on a supplied package name and pin coords
    /// (level, role, platform, site).
    Withs {
        /// The name of the package to search for.
        #[structopt(name = "PACKAGE")]
        package: String,
        /// The level, which may be 'facility' or a Levelspec (ie show[.seq[.shot]]). Defaults to 'facility'.
        #[structopt(short = "L", long, display_order = 1)]
        level: Option<String>,
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short = "R", long, display_order = 2)]
        role: Option<String>,
        /// The operating system name - (eg cent7_64). Defaults to 'any'.
        #[structopt(short = "P", long, display_order = 3)]
        platform: Option<String>,
        /// The site - defaults to 'any'.
        #[structopt(short = "S", long, display_order = 4)]
        site: Option<String>,
        /// Search mode - ancestor (or down), exact, descendant (or up).
        #[structopt(short, long = "search", display_order = 5)]
        search_mode: Option<String>,
        /// Limit the number of returned items.
        #[structopt(short, long, display_order = 6)]
        limit: Option<i32>,
        /// Provide one or more comma separated items to order the return by.
        #[structopt(short, long = "order-by", display_order = 7)]
        order_by: Option<String>,
    },
    #[structopt(display_order = 4)]
    /// Search for pins. Discover what pin coordinates are being used.
    Pins {
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short = "R", long, display_order = 2)]
        role: Option<String>,
        /// Levelspec format show[.seq[.shot]]. Defaults to 'facility'.
        #[structopt(short = "L", long, display_order = 1)]
        level: Option<String>,
        /// The operating system name - (eg cent7_64). Defaults to 'any'.
        #[structopt(short = "P", long, display_order = 3)]
        platform: Option<String>,
        /// The location name (eg portland) - defaults to 'any'.
        #[structopt(short = "S", long, display_order = 4)]
        site: Option<String>,
        /// Search mode - ancestor (or down), exact, descendant (or up). Defaults to 'ancestor'.
        #[structopt(short, long = "search", display_order = 5)]
        search_mode: Option<String>,
        /// Limit the number of returned items.
        #[structopt(short, long, display_order = 6)]
        limit: Option<i32>,
        /// Provide one or more comma separated items to order the return by.
        #[structopt(short, long = "order-by", display_order = 7)]
        order_by: Option<String>,
    },
    #[structopt(display_order = 5)]
    /// Get a simple list of all roles.
    Roles {
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short = "R", long, display_order = 1)]
        role: Option<String>,
        ///  The role category. One of: role, subrole, any. Defaults to 'any'.
        #[structopt(short = "C", long, display_order = 2)]
        category: Option<String>,
        /// Provide one or more comma separated items to order the return by.
        #[structopt(short, long = "order-by", display_order = 3)]
        order_by: Option<String>,
    },
    #[structopt(display_order = 6)]
    /// Get a simple list of all platforms.
    Platforms {
        /// The platform (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short = "P", long, display_order = 1)]
        platform: Option<String>,
        ///  Order by ... Platform or nutin'.
        #[structopt(short, long = "order-by", display_order = 2)]
        order_by: Option<String>,
    },
    #[structopt(display_order = 7)]
    /// Get a simple list of all sites.
    Sites {
        /// The location name (eg portland). Defaults to 'any'.
        #[structopt(short = "S", long)]
        site: Option<String>,
    },
    #[structopt(display_order = 8)]
    /// Get a simple list of all levels.
    Levels {
        /// The jobsystem level (facility or show[.seq[.shot]]). Defaults to 'any'.
        #[structopt(short = "L", long, display_order = 1)]
        level: Option<String>,
        ///  The name of the show.
        #[structopt(short = "S", long, display_order = 2)]
        show: Option<String>,
        /// Provide one or more comma separated items to order the return by.
        #[structopt(short, long = "order-by", display_order = 3)]
        order_by: Option<String>,
    },
    #[structopt(display_order = 9)]
    /// Get a simple list of all packages.
    Packages {},
    #[structopt(display_order = 8)]
    /// Get a list of distributions
    Distributions {
        /// The package name. Otherwise we search for all packages.
        #[structopt(short = "N", long, display_order = 1)]
        package: Option<String>,
        ///  The version of the distributions (eg 1.2.3).
        #[structopt(short = "V", long, display_order = 2)]
        version: Option<String>,
        // /// Provide one or more comma separated items to order the return by.
        // #[structopt(short, long = "order-by", display_order = 3)]
        // order_by: Option<String>,
        /// The order direction. may be "asc" or "desc".
        #[structopt(short = "D", long = "order-direction", display_order = 3)]
        order_direction: Option<String>,
    },
    #[structopt(display_order = 10)]
    /// Search for package coordinates. Package coordinates are simply
    /// a package name and a set of Coords (Level, Role, Platform, Site).
    PkgCoords {
        /// The package name. Otherwise we search for all packages.
        #[structopt(short = "N", long, display_order = 1)]
        package: Option<String>,
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short = "R", long, display_order = 3)]
        role: Option<String>,
        /// Levelspec format show[.seq[.shot]]. Defaults to 'facility'.
        #[structopt(short = "L", long, display_order = 2)]
        level: Option<String>,
        /// The operating system name - (eg cent7_64). Defaults to 'any'.
        #[structopt(short = "P", long, display_order = 4)]
        platform: Option<String>,
        /// The location name (eg portland) - defaults to 'any'.
        #[structopt(short = "S", long, display_order = 5)]
        site: Option<String>,
        /// Search mode - ancestor (or 'down','d', '<'), exact (or 'e' or '.'),
        /// descendant (or 'up','u', '>').
        /// Defaults to 'ancestor'.
        #[structopt(short, long = "search", display_order = 6)]
        search_mode: Option<String>,
        // /// Limit the number of returned items.
        // #[structopt(short, long, display_order = 6)]
        // limit: Option<i32>,
        /// Provide one or more comma separated items to order the return by.
        #[structopt(short, long = "order-by", display_order = 7)]
        order_by: Option<String>,
    },
}
