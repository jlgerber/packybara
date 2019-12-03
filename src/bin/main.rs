/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use packybara::packrat::{Client, NoTls};
use structopt::StructOpt;
mod cmd;
use cmd::args::*;
use env_logger;
use env_logger::Env;
use std::env;

//use log;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Pb::from_args();
    if let Pb {
        loglevel: Some(ref level),
        ..
    } = opt
    {
        env::set_var("RUST_LOG", level);
    }
    env_logger::from_env(Env::default().default_filter_or("warn")).init();
    let client = Client::connect(
        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
        NoTls,
    )?;
    let Pb { cmd, .. } = opt;
    match cmd {
        PbSub::VersionPin { .. } => {
            cmd::versionpin::process(client, cmd)?;
        }
        PbSub::VersionPins { .. } => {
            cmd::versionpins::process(client, cmd)?;
        }
        PbSub::AllRoles { .. } => {
            cmd::all_roles::process(client, cmd)?;
        }
        PbSub::AllPlatforms { .. } => {
            cmd::all_platforms::process(client, cmd)?;
        }
        PbSub::AllSites { .. } => {
            cmd::all_sites::process(client, cmd)?;
        }
        PbSub::Roles { .. } => {
            cmd::roles::process(client, cmd)?;
        }
        PbSub::Withs { .. } => {
            cmd::withs::process(client, cmd)?;
        } // _ => println!("not supported"),
    }

    Ok(())
}
