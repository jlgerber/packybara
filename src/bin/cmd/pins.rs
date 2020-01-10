use super::args::PbFind;
use packybara::db::traits::*;
use packybara::packrat::{Client, PackratDb};
use packybara::{LtreeSearchMode, SearchAttribute};
use prettytable::{cell, format, row, table};
use std::ops::Deref;
use std::str::FromStr;

pub fn find(client: Client, cmd: PbFind) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Pins {
        level,
        role,
        platform,
        site,
        search_mode,
        order_by,
        ..
    } = cmd
    {
        let mut pb = PackratDb::new(client);
        let mut results = pb.find_pins();
        results
            .role_opt(role.as_ref().map(Deref::deref))
            .level_opt(level.as_ref().map(Deref::deref))
            .platform_opt(platform.as_ref().map(Deref::deref))
            .site_opt(site.as_ref().map(Deref::deref));
        if let Some(ref mode) = search_mode {
            results.search_mode(LtreeSearchMode::from_str(mode)?);
        }
        if let Some(ref order) = order_by {
            let orders = order
                .split(",")
                .map(|x| SearchAttribute::from_str(x).unwrap_or(SearchAttribute::Unknown))
                .collect::<Vec<SearchAttribute>>();
            results.order_by(orders);
        }
        let results = results.query()?;
        // For now I do this. I need to add packge handling into the query
        // either by switching functions or handling the sql on this end
        let mut table = table!([bFg => "ROLE", "LEVEL", "PLATFORM", "SITE"]);
        for result in results {
            table.add_row(row![
                result.role,
                result.level,
                result.platform,
                result.site
            ]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    };

    Ok(())
}
