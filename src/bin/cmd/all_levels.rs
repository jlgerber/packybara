use super::args::PbSub;
use packybara::packrat::{Client, PackratDb};
use packybara::OrderLevelBy;
use prettytable::{cell, format, row, table};
use std::ops::Deref;
use std::str::FromStr;
pub fn process(client: Client, cmd: PbSub) -> Result<(), Box<dyn std::error::Error>> {
    if let PbSub::Levels {
        level,
        show,
        order_by,
        ..
    } = cmd
    {
        let mut pb = PackratDb::new(client);
        let mut results = pb.find_all_levels();
        results
            .level_opt(level.as_ref().map(Deref::deref))
            .show_opt(show.as_ref().map(Deref::deref));
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
