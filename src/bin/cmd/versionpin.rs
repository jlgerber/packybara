/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use super::args::PbFind;
use super::utils::extract_coords;
use super::utils::truncate;
use packybara::db::traits::*;
use packybara::packrat::{Client, PackratDb};
use prettytable::{cell, format, row, table};

/// Pretty print the most appropriate versionpin from the database that matches the provided criteria
///
/// # Arguments
/// * `client` - A Client instance used to connect to the database
/// * `cmd` - A PbFind enum instance used to extract the relevant commandline arguments
///
/// # Returns
/// * a Unit if Ok, or a boxed error if Err
pub fn find(client: Client, cmd: PbFind) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::VersionPin {
        package,
        level,
        role,
        platform,
        site,
        search_mode,
        full_withs,
        ..
    } = cmd
    {
        let (level, role, platform, site, _mode) =
            extract_coords(&level, &role, &platform, &site, &search_mode);
        let mut pb = PackratDb::new(client);
        let result = pb
            .find_versionpin(package.as_str())
            .level(level.as_str())
            .role(role.as_str())
            .platform(platform.as_str())
            .site(site.as_str())
            .query()?;

        let mut table =
            table!([bFg => "PIN ID", "DISTRIBUTION", "ROLE", "LEVEL", "PLATFORM", "SITE", "WITHS"]);
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

        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    };

    Ok(())
}
