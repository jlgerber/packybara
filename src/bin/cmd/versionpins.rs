use super::args::{PbFind, PbSet};
use super::utils::extract_coords;
use super::utils::truncate;
use packybara::db::update::versionpins::VersionPinChange;
use packybara::packrat::{Client, PackratDb};
use packybara::{LtreeSearchMode, SearchAttribute};
use prettytable::{cell, format, row, table};
use std::str::FromStr;

pub fn process(client: Client, cmd: PbFind) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::VersionPins {
        package,
        level,
        role,
        platform,
        site,
        search_mode,
        order_by,
        full_withs,
        ..
    } = cmd
    {
        let (level, role, platform, site, mode) =
            extract_coords(&level, &role, &platform, &site, &search_mode);
        let mut pb = PackratDb::new(client);
        // we have to assign the results to a variable first
        // because we will be calling setters in optional blocks.
        // since they returm &mut ref, we cant chain the first calls
        // immediately..
        let mut results = pb.find_all_versionpins();
        results
            .level(level.as_str())
            .role(role.as_str())
            .platform(platform.as_str())
            .site(site.as_str())
            .search_mode(LtreeSearchMode::from_str(mode.as_str())?);
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
        let results = if let Some(ref package) = package {
            results
                .into_iter()
                .filter(|x| x.distribution.package() == package)
                .collect::<Vec<_>>()
        } else {
            results
        };
        let mut table =
            table!([bFg => "PIN ID", "DISTRIBUTION", "ROLE", "LEVEL", "PLATFORM", "SITE", "WITHS"]);
        for result in results {
            let withs = result.withs.unwrap_or(Vec::new());
            let withs = if withs.len() > 0 {
                if full_withs {
                    format!("[{}]", withs.join(","))
                } else {
                    format!("[{}...]", truncate(withs.join(",").as_ref(), 40))
                }
            } else {
                "[]".to_string()
            };
            table.add_row(row![
                result.versionpin_id,
                result.distribution,
                result.coords.role,
                result.coords.level,
                result.coords.platform,
                result.coords.site,
                withs,
            ]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    };

    Ok(())
}

/// Add one or more roles
pub fn set(client: Client, cmd: PbSet) -> Result<(), Box<dyn std::error::Error>> {
    let PbSet::VersionPins {
        dist_ids, vpin_ids, ..
    } = cmd;
    assert_eq!(dist_ids.len(), vpin_ids.len());
    let mut pb = PackratDb::new(client);
    let mut results = pb.update_versionpins();
    log::debug!("versionpins: {:?} distribtions: {:?}", dist_ids, vpin_ids);
    let mut updates = Vec::new();
    for cnt in 0..dist_ids.len() {
        let change = VersionPinChange::new(vpin_ids[cnt], Some(dist_ids[cnt]), None);
        updates.push(change);
    }
    let results = results.changes(&mut updates).update()?;
    println!("{}", results);
    Ok(())
}
