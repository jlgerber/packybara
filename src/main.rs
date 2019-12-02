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
    //println!("{:#?}", cmd);
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
        PbSub::DistributionWiths { .. } => {
            cmd::distribution_withs::process(client, cmd)?;
        }
        _ => println!("not supported"),
    }

    Ok(())
}
