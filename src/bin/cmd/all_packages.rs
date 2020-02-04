/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use super::args::{PbAdd, PbFind};
use packybara::db::traits::*;
use packybara::packrat::{Client, PackratDb, Transaction};
use packybara::traits::TransactionHandler;
use prettytable::{cell, format, row, table};
use whoami;

/// Pretty print the set of packages from the database that match the provided criteria
///
/// # Arguments
/// * `client` - A Client instance used to connect to the database
/// * `cmd` - A PbFind enum instance used to extract the relevant commandline arguments
///
/// # Returns
/// * a Unit if Ok, or a boxed error if Err
pub fn find(client: Client, cmd: PbFind) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Packages { .. } = cmd {
        //let (level, role, site, site, mode) =
        //extract_coords(&level, &role, &site, &site, &search_mode);
        let mut pb = PackratDb::new(client);
        let mut results = pb.find_all_packages();
        //results.order_by(order_by.as_ref().map(Deref::deref));
        let results = results.query()?;
        // For now I do this. I need to add packge handling into the query
        // either by switching functions or handling the sql on this end

        let mut table = table!([bFg => "Name"]);
        for result in results {
            table.add_row(row![result.name]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    };

    Ok(())
}

/// Add one or more packages
pub fn add<'a>(tx: Transaction<'a>, cmd: PbAdd) -> Result<(), Box<dyn std::error::Error>> {
    //let mut pb = PackratDb::new(client);
    if let PbAdd::Packages { mut names, .. } = cmd {
        let comment = "Auto Comment - packages added";
        let username = whoami::username();
        //let resu  lts =
        let result = PackratDb::add_packages(tx)
            .packages(&mut names)
            .create()?
            .commit(&username, &comment)?;
        println!("{:?}", result);
    }
    Ok(())
}
