/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use packybara::packrat::{Client, NoTls, PackratDb};
use packybara::{SearchAttribute, SearchMode};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
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

#[derive(StructOpt, Debug)]
#[structopt(about = "PackybaraDb CRUD")]
pub enum PbSub {
    /// provide list of packages for given platform(s)
    Distribution {
        /// The name of the package
        #[structopt(short, long)]
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
        /// Search mode - ancestor, exact,
        #[structopt(long = "search-mode")]
        search_mode: Option<String>,
        /// limit the number of returned items
        #[structopt(long)]
        limit: Option<i32>,
        /// provide one or more comma separated items to order the return by.
        #[structopt(long = "order-by")]
        order_by: Option<String>,
        /// Search for all distributions, or just the closest match to the parameters
        #[structopt(short, long)]
        all: bool,
    },
    Platform {},
}

fn extract_coords<'a>(
    level: &'a Option<String>,
    role: &'a Option<String>,
    platform: &'a Option<String>,
    site: &'a Option<String>,
    mode: &'a Option<String>,
) -> (String, String, String, String, String) {
    let r = role.clone().unwrap_or("any".to_string());
    let l = level.clone().unwrap_or("facility".to_string());
    let p = platform.clone().unwrap_or("any".to_string());
    let s = site.clone().unwrap_or("any".to_string());
    let m = mode.clone().unwrap_or("ancestor".to_string());

    (l, r, p, s, m)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Pb::from_args();
    //println!("{:#?}", opt);
    let client = Client::connect(
        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
        NoTls,
    )?;
    let Pb { cmd, .. } = opt;
    if let PbSub::Distribution {
        package,
        level,
        role,
        platform,
        site,
        search_mode,
        order_by,
        all,
        ..
    } = cmd
    {
        let (level, role, platform, site, mode) =
            extract_coords(&level, &role, &platform, &site, &search_mode);
        // println!(
        //     "{} {} {} {} {} {}",
        //     package, level, role, platform, site, mode,
        // );
        let mut pb = PackratDb::new(client);
        if all == true {
            let mut results = pb.find_all_distributions();
            results
                .level(level.as_str())
                .role(role.as_str())
                .platform(platform.as_str())
                .site(site.as_str())
                .search_mode(SearchMode::from_str(mode.as_str())?);
            if let Some(ref order) = order_by {
                let orders = order
                    .split(",")
                    .map(|x| SearchAttribute::from_str(x).unwrap_or(SearchAttribute::Unknown))
                    .collect::<Vec<SearchAttribute>>();
                results.order_by(orders);
            }
            let results = results.query()?;
            for result in results {
                println!("{}", result);
            }
        } else {
            let result = pb
                .find_distribution(package.as_str())
                .level(level.as_str())
                .role(role.as_str())
                .platform(platform.as_str())
                .site(site.as_str())
                .query()?;
            println!("{}", result);
        }
    };
    return Ok(());
    /*
    let mut result = PackratDb::new(client)
        .find_distribution("houdini")
        .level("bayou")
        .role("fx_beta")
        .platform("cent7_64")
        .site("portland")
        .query()?;
    println!("{:?}", result);
    println!(
        "package: {} version: {}",
        result.package(),
        result.version()
    );
    println!("\n\nFIND THE DISTRIBUTION");

    pb.find_distribution("maya")
        .level("bayou")
        .role("fx_beta")
        .platform("cent7_64")
        .site("portland")
        .query()?;

    println!("{:?}", result);
    println!(
        "package: {} version: {}",
        result.package(),
        result.version()
    );

    println!("\n\nFIND DISTRIBUTIONS");

    let results = pb
        .find_distributions("maya")
        .level("bayou")
        .role("fx_beta")
        .platform("cent7_64")
        .site("portland")
        .query()?;

    for result in results {
        println!("{}", result);
    }

    println!("\n\nFIND WITHS");
    let results = pb
        .find_distribution_withs("maya")
        .level("bayou")
        .role("fx_beta")
        .platform("cent7_64")
        .site("portland")
        .query()?;

    for result in results {
        println!("{}", result);
    }

    println!("\n\nFIND ALL DISTRIBUTIONS");
    let results = pb
        .find_all_distributions()
        .level("facility")
        .role("any")
        .platform("any")
        .site("any")
        .order_by(vec![
            //packybara::SearchAttribute::Level,
            packybara::SearchAttribute::Package,
        ])
        .limit(15)
        .search_mode(SearchMode::Descendant)
        .query()?;
    for result in results {
        println!("{}", result);
    }
    Ok(())
    */
}
