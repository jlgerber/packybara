use super::args::PbSub;
use super::utils::extract_coords;
use packybara::packrat::{Client, PackratDb};

pub fn process(client: Client, cmd: PbSub) -> Result<(), Box<dyn std::error::Error>> {
    if let PbSub::Distribution {
        package,
        level,
        role,
        platform,
        site,
        search_mode,
        ..
    } = cmd
    {
        let (level, role, platform, site, _mode) =
            extract_coords(&level, &role, &platform, &site, &search_mode);
        let mut pb = PackratDb::new(client);
        let result = pb
            .find_distribution(package.as_str())
            .level(level.as_str())
            .role(role.as_str())
            .platform(platform.as_str())
            .site(site.as_str())
            .query()?;
        println!("{}", result);
    };

    Ok(())
}
