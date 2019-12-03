use super::args::PbSub;
use packybara::packrat::{Client, PackratDb};
//use packybara::OrderSiteBy;
use prettytable::{cell, format, row, table};
use std::ops::Deref;
//use std::str::FromStr;
pub fn process(client: Client, cmd: PbSub) -> Result<(), Box<dyn std::error::Error>> {
    if let PbSub::AllSites { site, .. } = cmd {
        //let (level, role, site, site, mode) =
        //extract_coords(&level, &role, &site, &site, &search_mode);
        let mut pb = PackratDb::new(client);
        let mut results = pb.find_all_sites();
        results.name_opt(site.as_ref().map(Deref::deref));
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
