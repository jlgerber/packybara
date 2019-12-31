use super::args::PbFind;
use packybara::packrat::{Client, PackratDb};
//use packybara::OrderSiteBy;
use prettytable::{cell, format, row, table};
//use std::ops::Deref;
//use std::str::FromStr;
pub fn find(client: Client, cmd: PbFind) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::VersionPinWiths { versionpin_id, .. } = cmd {
        //let (level, role, site, site, mode) =
        //extract_coords(&level, &role, &site, &site, &search_mode);
        let mut pb = PackratDb::new(client);
        let results = pb.find_all_versionpin_withs(versionpin_id).query()?;
        // For now I do this. I need to add packge handling into the query
        // either by switching functions or handling the sql on this end

        let mut table = table!([bFg => "ID", "VERSIONPIN ID", "WITH", "ORDER"]);
        for result in results {
            table.add_row(row![result.id, result.vpin_id, result.with, result.order]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    };

    Ok(())
}
