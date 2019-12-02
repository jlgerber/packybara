use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
pub struct Pb {
    /// Set the log level. This may target one or more
    /// specific modules or be general.
    /// (levels: trace, debug, info, warn, error)
    #[structopt(long)]
    pub loglevel: Option<String>,

    /// Set the query location. Defaults to localhost if
    /// mode is test. otherwise it attempts to get from environment
    // #[structopt(short, long)]
    // pub location: Option<String>,

    // ///path to preference file
    // #[structopt(short, long)]
    // pub file: Option<String>,

    // /// Set the mode (production, development)
    // #[structopt(short, long)]
    // mode: Option<String>,

    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub cmd: PbSub,
}

#[derive(StructOpt, Debug, PartialEq)]
#[structopt(about = "PackybaraDb CRUD")]
pub enum PbSub {
    /// Find the specific distribution whose coords are closest to
    /// the supplied coords (level,role,platform,site)
    Distribution {
        /// The name of the package to search for
        #[structopt()]
        package: String,
        /// Levelspec format show[.seq[.shot]]. Defaults to 'facility'.
        #[structopt(short, long)]
        level: Option<String>,
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short, long)]
        role: Option<String>,
        /// OS - (eg cent7_64)
        #[structopt(long)]
        platform: Option<String>,
        /// The site - defaults to 'any'
        #[structopt(short, long)]
        site: Option<String>,
        /// Search mode - ancestor, exact, descendant.
        #[structopt(long = "search-mode")]
        search_mode: Option<String>,
        /// limit the number of returned items
        #[structopt(long)]
        limit: Option<i32>,
        /// provide one or more comma separated items to order the return by.
        #[structopt(long = "order-by")]
        order_by: Option<String>,
    },
    /// Find all distributions that meet criteria.
    Distributions {
        /// The name of the package
        #[structopt(short, long)]
        package: Option<String>,
        /// Levelspec format show[.seq[.shot]]. Defaults to 'facility'.
        #[structopt(short, long)]
        level: Option<String>,
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short, long)]
        role: Option<String>,
        /// OS - (eg cent7_64)
        #[structopt(long)]
        platform: Option<String>,
        /// The site - defaults to 'any'
        #[structopt(short, long)]
        site: Option<String>,
        /// Search mode - ancestor, exact, descendant
        #[structopt(long = "search-mode")]
        search_mode: Option<String>,
        /// limit the number of returned items
        #[structopt(long)]
        limit: Option<i32>,
        /// provide one or more comma separated items to order the return by.
        #[structopt(long = "order-by")]
        order_by: Option<String>,
    },
    /// Find the specific distribution whose coords are closest to
    /// the supplied coords (level,role,platform,site)
    DistributionWiths {
        /// The name of the package to search for
        #[structopt()]
        package: String,
        /// Levelspec format show[.seq[.shot]]. Defaults to 'facility'.
        #[structopt(short, long)]
        level: Option<String>,
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short, long)]
        role: Option<String>,
        /// OS - (eg cent7_64)
        #[structopt(long)]
        platform: Option<String>,
        /// The site - defaults to 'any'
        #[structopt(short, long)]
        site: Option<String>,
        /// Search mode - ancestor, exact, descendant
        #[structopt(long = "search-mode")]
        search_mode: Option<String>,
        /// limit the number of returned items
        #[structopt(long)]
        limit: Option<i32>,
        /// provide one or more comma separated items to order the return by.
        #[structopt(long = "order-by")]
        order_by: Option<String>,
    },
    Roles {
        /// The role (eg model or anim_beta). Defaults to 'any'.
        #[structopt(short, long)]
        role: Option<String>,
        /// Levelspec format show[.seq[.shot]]. Defaults to 'facility'.
        #[structopt(short, long)]
        level: Option<String>,
        /// OS - (eg cent7_64)
        #[structopt(long)]
        platform: Option<String>,
        /// The site - defaults to 'any'
        #[structopt(short, long)]
        site: Option<String>,
        /// Search mode - ancestor, exact, descendant
        #[structopt(long = "search-mode")]
        search_mode: Option<String>,
        /// limit the number of returned items
        #[structopt(long)]
        limit: Option<i32>,
        /// provide one or more comma separated items to order the return by.
        #[structopt(long = "order-by")]
        order_by: Option<String>,
    },
    Platforms {},
}
