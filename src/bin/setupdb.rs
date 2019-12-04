/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
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
    #[structopt(short, long)]
    loglevel: Option<String>,

    #[structopt(short, long)]
    populate: bool,
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
    let drop_schema = "DROP SCHEMA IF EXISTS audit CASCADE";
    let drop_db = "DROP DATABASE IF EXISTS packrat";
    let create_db = "CREATE DATABASE packrat WITH ENCODING = UTF8";
    log::info!("executing SQL\n{}", drop_db);
    let mut client = Client::connect(
        "host=127.0.0.1 user=postgres dbname=postgres password=example port=5432",
        NoTls,
    )?;
    client.execute(drop_schema, &[])?;
    client.execute(drop_db, &[])?;
    client.execute(create_db, &[])?;

    let mut client = Client::connect(
        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
        NoTls,
    )?;
    let packrat_sql = concat!(env!("CARGO_MANIFEST_DIR"), "/sql/packrat.sql");
    let procs_sql = concat!(env!("CARGO_MANIFEST_DIR"), "/sql/procs.sql");
    let audit_sql = concat!(env!("CARGO_MANIFEST_DIR"), "/sql/vendor/audit.sql");
    let populate_sql = concat!(env!("CARGO_MANIFEST_DIR"), "/sql/populate.sql");
    let register_audits_sql = concat!(env!("CARGO_MANIFEST_DIR"), "/sql/register_audits.sql");

    for file_path in &[
        packrat_sql,
        procs_sql,
        audit_sql,
        populate_sql,
        register_audits_sql,
    ] {
        let contents = fs::read_to_string(file_path)?;
        log::info!("batch executing {}", file_path);
        log::debug!("{}", contents);
        client.batch_execute(contents.as_str())?;
    }
    Ok(())
}
