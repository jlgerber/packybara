/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use crate::distribution::Distribution;
use postgres::Client;

/// Responsible for finding a distribution
pub struct FindDistribution<'a> {
    client: &'a mut Client,
    package: &'a str,
    level: Option<&'a str>,
    role: Option<&'a str>,
    platform: Option<&'a str>,
    site: Option<&'a str>,
}

impl<'a> FindDistribution<'a> {
    pub fn new(client: &'a mut Client, package: &'a str) -> Self {
        FindDistribution {
            client,
            package,
            level: None,
            role: None,
            platform: None,
            site: None,
        }
    }

    pub fn level(&mut self, level_n: &'a str) -> &mut Self {
        self.level = Some(level_n);
        self
    }

    pub fn role(&mut self, role_n: &'a str) -> &mut Self {
        self.role = Some(role_n);
        self
    }

    pub fn platform(&mut self, platform_n: &'a str) -> &mut Self {
        self.platform = Some(platform_n);
        self
    }

    pub fn site(&mut self, site_n: &'a str) -> &mut Self {
        self.site = Some(site_n);
        self
    }

    pub fn query(&mut self) -> Result<Distribution, Box<dyn std::error::Error>> {
        let level = self.level.unwrap_or("facility");
        let role = self.role.unwrap_or("any");
        let platform = self.platform.unwrap_or("any");
        let site = self.site.unwrap_or("any");
        let mut result = Vec::new();
        for row in self.client.query(
            "SELECT distribution
            FROM find_distribution(
                $1, 
                role => $2, 
                platform => $3, 
                level=>$4, 
                site => $5)",
            &[&self.package, &role, &platform, &level, &site],
        )? {
            let distribution: &str = row.get(0);
            result.push(Distribution::new(distribution));
        }
        Ok(result.pop().unwrap())
    }
}

pub struct PackratDb {
    client: Client,
}

impl PackratDb {
    pub fn new(client: Client) -> Self {
        PackratDb { client }
    }

    pub fn find_distribution<'b>(&'b mut self, package: &'b str) -> FindDistribution {
        FindDistribution::new(&mut self.client, package)
    }
}
