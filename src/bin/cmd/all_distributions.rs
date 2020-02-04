/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use super::args::PbFind;
use packybara::db::traits::*;
use packybara::packrat::{Client, PackratDb};
use packybara::OrderDirection;
use prettytable::{cell, format, row, table};
use std::ops::Deref;
use std::str::FromStr;

/// Pretty print the set of distributions from the database that match the provided criteria
///
/// # Arguments
/// * `client` - A Client instance used to connect to the database
/// * `cmd` - A PbFind enum instance used to extract the relevant commandline arguments
///
/// # Returns
/// * a Unit if Ok, or a boxed error if Err
pub fn find(client: Client, cmd: PbFind) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Distributions {
        package,
        version,
        order_direction,
        ..
    } = cmd
    {
        let mut pb = PackratDb::new(client);
        let mut results = pb.find_all_distributions();
        let dir =
            OrderDirection::from_str(order_direction.as_ref().map(Deref::deref).unwrap_or("asc"))?;
        results
            .package_opt(package.as_ref().map(Deref::deref))
            .version_opt(version.as_ref().map(Deref::deref))
            .order_direction_opt(Some(dir));

        // if let Some(ref order) = order_by {
        //     let orders = order
        //         .split(",")
        //         .map(|x| {
        //             OrderPlatformBy::from_str(x).unwrap_or_else(|y| {
        //                 log::warn!("invalid order-by argument:'{}'. {}", x, y);
        //                 OrderPlatformBy::Name
        //             })
        //         })
        //         .collect::<Vec<OrderPlatformBy>>();
        //     results.order_by(orders);
        // }
        let results = results.query()?;
        // For now I do this. I need to add packge handling into the query
        // either by switching functions or handling the sql on this end

        let mut table = table!([bFg => "Id", "Package", "Version"]);
        for result in results {
            table.add_row(row![result.id, result.package, result.version]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    };

    Ok(())
}
/*
/// Add one or more roles
pub fn add(client: Client, cmd: PbAdd) -> Result<(), Box<dyn std::error::Error>> {
    if let PbAdd::Platforms { mut names, .. } = cmd {
        let mut pb = PackratDb::new(client);
        let mut results = pb.add_platforms();
        let results = results.platforms(&mut names).create()?;
        println!("{}", results);
    }
    Ok(())
}
*/
