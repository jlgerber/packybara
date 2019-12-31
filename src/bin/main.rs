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
//trait RetType: std::error::Error + std::fmt::Display {}

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
    let mut client = Client::connect(
        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
        NoTls,
    )?;

    let Pb { crud, .. } = opt;
    match crud {
        PbCrud::Find { cmd } => match cmd {
            PbFind::VersionPin { .. } => {
                cmd::versionpin::find(client, cmd)?;
            }
            PbFind::VersionPins { .. } => {
                cmd::versionpins::find(client, cmd)?;
            }
            PbFind::Roles { .. } => {
                cmd::all_roles::find(client, cmd)?;
            }
            PbFind::Platforms { .. } => {
                cmd::all_platforms::find(client, cmd)?;
            }
            PbFind::Sites { .. } => {
                cmd::all_sites::find(client, cmd)?;
            }
            PbFind::Levels { .. } => {
                cmd::all_levels::find(client, cmd)?;
            }
            PbFind::Pins { .. } => {
                cmd::pins::find(client, cmd)?;
            }
            PbFind::VersionPinWiths { .. } => {
                cmd::versionpin_withs::find(client, cmd)?;
            }
            PbFind::Withs { .. } => {
                cmd::withs::find(client, cmd)?;
            }
            PbFind::Packages { .. } => {
                cmd::all_packages::find(client, cmd)?;
            }
            PbFind::Distributions { .. } => {
                cmd::all_distributions::find(client, cmd)?;
            }
            PbFind::PkgCoords { .. } => {
                cmd::pkgcoords::find(client, cmd)?;
            }
            PbFind::Revisions { .. } => {
                cmd::all_revisions::find(client, cmd)?;
            }
            PbFind::Changes { .. } => {
                cmd::all_changes::find(client, cmd)?;
            }
        },
        PbCrud::Add { cmd } => match cmd {
            PbAdd::Packages { .. } => {
                let tx = client.transaction().unwrap();
                cmd::all_packages::add(tx, cmd)?;
            }
            PbAdd::Levels { .. } => {
                let tx = client.transaction().unwrap();
                cmd::all_levels::add(tx, cmd)?;
            }
            PbAdd::Roles { .. } => {
                let tx = client.transaction().unwrap();
                cmd::all_roles::add(tx, cmd)?;
            }
            PbAdd::Platforms { .. } => {
                let tx = client.transaction().unwrap();
                cmd::all_platforms::add(tx, cmd)?;
            }
            PbAdd::Withs { .. } => {
                let tx = client.transaction().unwrap();
                cmd::withs::add(tx, cmd)?;
            }
        },
        PbCrud::Set { cmd } => match cmd {
            PbSet::VersionPins { .. } => {
                let tx = client.transaction().unwrap();
                cmd::versionpins::set(tx, cmd)?;
            }
        },
        _ => println!("Not implemented"),
    }

    Ok(())
}
