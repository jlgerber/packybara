use super::args::{PbAdd, PbFind};
use packybara::db::traits::*;
use packybara::packrat::{Client, PackratDb, Transaction};
use packybara::traits::TransactionHandler;
use packybara::OrderLevelBy;
use prettytable::{cell, format, row, table};
use std::ops::Deref;
use std::str::FromStr;
use whoami;

pub fn find(client: Client, cmd: PbFind) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Levels {
        level,
        show,
        depth,
        order_by,
        ..
    } = cmd
    {
        let mut pb = PackratDb::new(client);
        let mut results = pb.find_all_levels();
        results
            .level_opt(level.as_ref().map(Deref::deref))
            .show_opt(show.as_ref().map(Deref::deref))
            .depth_opt(depth);
        if let Some(ref order) = order_by {
            let orders = order
                .split(",")
                .map(|x| {
                    OrderLevelBy::from_str(x).unwrap_or_else(|y| {
                        log::warn!("invalid order-by argument:'{}'. {}", x, y);
                        OrderLevelBy::Name
                    })
                })
                .collect::<Vec<OrderLevelBy>>();
            results.order_by(orders);
        }
        let results = results.query()?;
        // For now I do this. I need to add packge handling into the query
        // either by switching functions or handling the sql on this end

        let mut table = table!([bFg => "LEVEL", "SHOW"]);
        for result in results {
            table.add_row(row![result.level, result.show]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    };

    Ok(())
}

/// Add one or more levels
pub fn add<'a>(tx: Transaction<'a>, cmd: PbAdd) -> Result<(), Box<dyn std::error::Error>> {
    if let PbAdd::Levels { mut names, .. } = cmd {
        let comment = "Auto Comment - levels added";
        let username = whoami::username();

        let results = PackratDb::add_levels(tx)
            .levels(&mut names)
            .create()?
            .commit(&username, &comment)?;
        println!("{}", results);
    }
    Ok(())
}
