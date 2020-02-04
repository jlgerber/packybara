use super::args::PbFind;
use packybara::db::find_all::revisions::{OrderDirection, OrderRevisionBy};
use packybara::db::traits::*;
use packybara::packrat::{Client, PackratDb};
use prettytable::{cell, format, row, table};
use std::ops::Deref;
use std::str::FromStr;

/// Pretty print the set of revisions from the database that match the provided criteria
///
/// # Arguments
/// * `client` - A Client instance used to connect to the database
/// * `cmd` - A PbFind enum instance used to extract the relevant commandline arguments
///
/// # Returns
/// * a Unit if Ok, or a boxed error if Err
pub fn find(client: Client, cmd: PbFind) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Revisions {
        id,
        transaction_id,
        author,
        order_by,
        order_direction,
        limit,
        ..
    } = cmd
    {
        let mut pb = PackratDb::new(client);
        let mut results = pb.find_all_revisions();
        let order_by_vec = if let Some(ref ob) = order_by {
            ob.split(",")
                .map(|x| OrderRevisionBy::from_str(x))
                .filter_map(Result::ok)
                .collect::<Vec<OrderRevisionBy>>()
        } else {
            vec![]
        };
        let order_dir = if let Some(ref dir) = order_direction {
            OrderDirection::from_str(dir).ok()
        } else {
            None
        };
        results
            .id_opt(id)
            .transaction_id_opt(transaction_id)
            .author_opt(author.as_ref().map(Deref::deref))
            .order_by(order_by_vec)
            .order_direction_opt(order_dir)
            .limit_opt(limit);
        let results = results.query()?;
        // For now I do this. I need to add packge handling into the query
        // either by switching functions or handling the sql on this end
        let mut table = table!([bFg => "ID","TRANSACTION ID", "AUTHOR", "DATETIME", "COMMENT"]);
        for result in results {
            table.add_row(row![
                result.id,
                result.transaction_id,
                result.author,
                result.datetime.format("%F %r"),
                result.comment,
            ]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    };

    Ok(())
}
