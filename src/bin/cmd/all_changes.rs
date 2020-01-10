use super::args::PbFind;
use packybara::db::traits::*;
use packybara::packrat::{Client, PackratDb};
use prettytable::{cell, format, row, table};

pub fn find(client: Client, cmd: PbFind) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Changes { transaction_id, .. } = cmd {
        let mut pb = PackratDb::new(client);
        let mut results = pb.find_all_changes();
        let results = results.transaction_id(transaction_id).query()?;
        // For now I do this. I need to add packge handling into the query
        // either by switching functions or handling the sql on this end
        let mut table = table!([bFg => "ID","TX ID", "ACTION", "LEVEL", "ROLE", "PLATFORM","SITE", "PACKAGE", "OLD", "NEW"]);
        for result in results {
            table.add_row(row![
                result.id,
                result.transaction_id,
                result.action,
                result.level,
                result.role,
                result.platform,
                result.site,
                result.package,
                result.old,
                result.new
            ]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    };

    Ok(())
}
