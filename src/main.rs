/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use packybara::PackratDb;
use postgres::{Client, NoTls};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::connect(
        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
        NoTls,
    )?;
    let mut pb = PackratDb::new(client);
    let result = pb
        .find_distribution("houdini")
        .level("bayou")
        .role("fx_beta")
        .platform("cent7_64")
        .site("portland")
        .query()?;

    println!("{:?}", result);
    println!(
        "package: {} version: {}",
        result.package(),
        result.version()
    );

    pb.find_distribution("maya")
        .level("bayou")
        .role("fx_beta")
        .platform("cent7_64")
        .site("portland")
        .query()?;

    println!("{:?}", result);
    println!(
        "package: {} version: {}",
        result.package(),
        result.version()
    );
    Ok(())
}
