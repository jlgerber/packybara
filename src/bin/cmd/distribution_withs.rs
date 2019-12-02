use super::args::PbSub;
use super::utils::extract_coords;
use packybara::packrat::{Client, PackratDb};
use packybara::SearchAttribute;
use prettytable::{cell, format, row, table};
use std::str::FromStr;

pub fn process(client: Client, cmd: PbSub) -> Result<(), Box<dyn std::error::Error>> {
    if let PbSub::DistributionWiths {
        package,
        level,
        role,
        platform,
        site,
        search_mode,
        order_by,
        ..
    } = cmd
    {
        let (level, role, platform, site, _mode) =
            extract_coords(&level, &role, &platform, &site, &search_mode);
        let mut pb = PackratDb::new(client);
        // we have to assign the results to a variable first
        // because we will be calling setters in optional blocks.
        // since they returm &mut ref, we cant chain the first calls
        // immediately..
        let mut results = pb.find_distribution_withs(package.as_str());
        results
            .level(level.as_str())
            .role(role.as_str())
            .platform(platform.as_str())
            .site(site.as_str());
        if let Some(ref order) = order_by {
            let orders = order
                .split(",")
                .map(|x| SearchAttribute::from_str(x).unwrap_or(SearchAttribute::Unknown))
                .collect::<Vec<SearchAttribute>>();
            results.order_by(orders);
        }
        let results = results.query()?;
        let mut table =
            table!([bFg => "PIN ID", "DISTRIBUTION", "ROLE", "LEVEL", "PLATFORM", "SITE"]);
        for result in results {
            table.add_row(row![
                result.versionpin_id,
                result.distribution,
                result.coords.role,
                result.coords.level,
                result.coords.platform,
                result.coords.site,
            ]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    };

    Ok(())
}
