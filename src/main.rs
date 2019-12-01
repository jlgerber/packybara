/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use packybara::packrat::{Client, NoTls}; // PackratDb};
                                         //use packybara::{SearchAttribute, SearchMode};
                                         //use std::str::FromStr;
use structopt::StructOpt;
mod cmd;
use cmd::args::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Pb::from_args();
    //println!("{:#?}", opt);
    let client = Client::connect(
        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
        NoTls,
    )?;
    let Pb { cmd, .. } = opt;
    println!("{:#?}", cmd);
    match cmd {
        PbSub::Distribution { .. } => {
            cmd::distribution::process(client, cmd)?;
        }
        PbSub::Distributions { .. } => {
            cmd::distributions::process(client, cmd)?;
        }
        PbSub::Roles { .. } => {
            cmd::roles::process(client, cmd)?;
        }
        _ => println!("not supported"),
    }

    Ok(())
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
