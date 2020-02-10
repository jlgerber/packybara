use super::args::{PbFind, PbSet};
use super::utils::extract_coords;
/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use super::utils::truncate;
use packybara::db::search_attribute::OrderDirection;
use packybara::db::traits::*;
use packybara::db::update::versionpins::VersionPinChange;
use packybara::packrat::{Client, PackratDb, Transaction};
use packybara::traits::TransactionHandler;
use packybara::{LtreeSearchMode, SearchAttribute};
use prettytable::{cell, format, row, table};
use std::str::FromStr;
use whoami;

/// Pretty print the set of version pins from the database that match the provided criteria
///
/// # Arguments
/// * `client` - A Client instance used to connect to the database
/// * `cmd` - A PbFind enum instance used to extract the relevant commandline arguments
///
/// # Returns
/// * a Unit if Ok, or a boxed error if Err
pub fn find(client: Client, cmd: PbFind) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::VersionPins {
        package,
        version,
        level,
        isolate_facility,
        role,
        platform,
        site,
        search_mode,
        order_by,
        order_direction,
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
        let package_opt = match package {
            Some(ref p) => Some(p.as_str()),
            None => None,
        };
        let version_opt = match version {
            Some(ref p) => Some(p.as_str()),
            None => None,
        };
        results
            .some_package(package_opt)
            .some_version(version_opt)
            .level(level.as_str())
            .isolate_facility(isolate_facility)
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
        if let Some(ref dir) = order_direction {
            let direction = OrderDirection::from_str(dir);
            if direction.is_ok() {
                let direction = direction.unwrap();
                results.order_direction(direction);
            } else {
                log::warn!("unable to apply search direction request {} to query", dir);
            }
        }
        let results = results.query()?;
        // For now I do this. I need to add packge handling into the query
        // either by switching functions or handling the sql on this end
        // this has been moved into the queyr
        // let results = if let Some(ref package) = package {
        //     results
        //         .into_iter()
        //         .filter(|x| x.distribution.package() == package)
        //         .collect::<Vec<_>>()
        // } else {
        //     results
        // };
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

/// Add one or more versionpin changes
pub fn set<'a>(tx: Transaction<'a>, cmd: PbSet) -> Result<(), Box<dyn std::error::Error>> {
    let PbSet::VersionPins {
        dist_ids,
        vpin_ids,
        comment,
        ..
    } = cmd;
    assert_eq!(dist_ids.len(), vpin_ids.len());
    let username = whoami::username();
    let mut update_versionpins = PackratDb::update_versionpins(tx);
    for cnt in 0..dist_ids.len() {
        let change = VersionPinChange::new(vpin_ids[cnt], Some(dist_ids[cnt]), None);
        update_versionpins = update_versionpins.change(change);
    }

    let update_cnt = update_versionpins.update()?.commit(&username, &comment)?;
    println!("{}", update_cnt);
    Ok(())
}
