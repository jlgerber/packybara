use super::args::PbSub;
use super::utils::extract_coords;
use packybara::packrat::{Client, PackratDb};
use packybara::{SearchAttribute, SearchMode};
use std::ops::Deref;
use std::str::FromStr;

pub fn process(client: Client, cmd: PbSub) -> Result<(), Box<dyn std::error::Error>> {
    if let PbSub::Roles {
        level,
        role,
        platform,
        site,
        search_mode,
        order_by,
        ..
    } = cmd
    {
        //let (level, role, platform, site, mode) =
        //extract_coords(&level, &role, &platform, &site, &search_mode);
        let mut pb = PackratDb::new(client);
        let mut results = pb.find_all_roles();
        results
            .role_opt(role.as_ref().map(Deref::deref))
            .level_opt(level.as_ref().map(Deref::deref))
            .platform_opt(platform.as_ref().map(Deref::deref))
            .site_opt(site.as_ref().map(Deref::deref));
        if let Some(ref mode) = search_mode {
            results.search_mode(SearchMode::from_str(mode)?);
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
        for result in results {
            println!("{}", result);
        }
    };

    Ok(())
}
