use super::args::PbSub;
use super::utils::extract_coords;
use packybara::packrat::{Client, PackratDb};
use packybara::{SearchAttribute, SearchMode};
use std::str::FromStr;

pub fn process(client: Client, cmd: PbSub) -> Result<(), Box<dyn std::error::Error>> {
    if let PbSub::Distributions {
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
        let (level, role, platform, site, mode) =
            extract_coords(&level, &role, &platform, &site, &search_mode);
        let mut pb = PackratDb::new(client);
        let mut results = pb.find_all_distributions();
        results
            .level(level.as_str())
            .role(role.as_str())
            .platform(platform.as_str())
            .site(site.as_str())
            .search_mode(SearchMode::from_str(mode.as_str())?);
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
        if let Some(ref package) = package {
            for result in results {
                if result.distribution.package() == package {
                    println!("{}", result);
                }
            }
        } else {
            for result in results {
                println!("{}", result);
            }
        }
    };

    Ok(())
}
