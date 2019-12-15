use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
#[structopt(about = "Set entities in db")]
pub enum PbSet {
    /// Set the distribution for a versionpin
    #[structopt(display_order = 1)]
    VersionPins {
        /// provide the versionpin id followed by the distribution id
        #[structopt(
            short,
            long = "dist-id",
            display_order = 1,
            min_values = 2,
            max_values = 2
        )]
        dist_ids: Option<Vec<i32>>,
    },
}
