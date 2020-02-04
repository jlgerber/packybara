/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use env_logger;
use env_logger::Env;
use log;
use packybara::packrat::{Client, NoTls};
use std::env;
use std::fs;
use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
pub struct Build {
    /// Set the loglevel
    #[structopt(short, long, display_order = 1)]
    loglevel: Option<String>,
    /// drop the database, then create it from scratch. Then add in the triggers.
    #[structopt(short, long, display_order = 2)]
    rebuild: bool,
    /// populate the database with functions
    #[structopt(short = "f", long = "funcs", display_order = 3)]
    procs: bool,
    /// populate the database with scratch data
    #[structopt(short, long, display_order = 4)]
    populate: bool,
    /// Do everything, in the following order: rebuild, procs, populate
    #[structopt(short, long, display_order = 5)]
    all: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Build::from_args();
    if let Build {
        loglevel: Some(ref level),
        ..
    } = opt
    {
        env::set_var("RUST_LOG", level);
    }
    env_logger::from_env(Env::default().default_filter_or("warn")).init();
    let Build {
        rebuild,
        procs,
        populate,
        all,
        ..
    } = opt;
    let drop_schema = "DROP SCHEMA IF EXISTS audit CASCADE";
    let drop_db = "DROP DATABASE IF EXISTS packrat";
    let create_db = "CREATE DATABASE packrat WITH ENCODING = UTF8";
    log::info!("executing SQL\n{}", drop_db);
    let mut client = Client::connect(
        "host=127.0.0.1 user=postgres dbname=postgres password=example port=5432",
        NoTls,
    )?;
    if rebuild || all {
        client.execute(drop_schema, &[])?;
        client.execute(drop_db, &[])?;
        client.execute(create_db, &[])?;
    }

    let mut client = Client::connect(
        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
        NoTls,
    )?;
    let mut files = Vec::new();

    if rebuild || all {
        let packrat_sql = concat!(env!("CARGO_MANIFEST_DIR"), "/sql/packrat.sql");
        let audit_sql = concat!(env!("CARGO_MANIFEST_DIR"), "/sql/vendor/audit.sql");
        let register_audits_sql = concat!(env!("CARGO_MANIFEST_DIR"), "/sql/register_audits.sql");
        files.push(packrat_sql);
        files.push(audit_sql);
        files.push(register_audits_sql);
    }
    if procs || all {
        let procs_sql = concat!(env!("CARGO_MANIFEST_DIR"), "/sql/procs.sql");
        files.push(procs_sql);
    }
    if populate || all {
        let populate_sql = concat!(env!("CARGO_MANIFEST_DIR"), "/sql/populate.sql");
        files.push(populate_sql);
    }

    for file_path in &files {
        let contents = fs::read_to_string(file_path)?;
        log::info!("batch executing {}", file_path);
        log::debug!("{}", contents);
        client.batch_execute(contents.as_str())?;
    }
    Ok(())
}
