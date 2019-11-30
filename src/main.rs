/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use packybara::packrat::{Client, NoTls, PackratDb};

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

    let results = pb
        .find_distributions("maya")
        .level("bayou")
        .role("fx_beta")
        .platform("cent7_64")
        .site("portland")
        .query()?;

    for result in results {
        println!("{}", result);
    }

    let results = pb
        .find_distribution_withs("maya")
        .level("bayou")
        .role("fx_beta")
        .platform("cent7_64")
        .site("portland")
        .query()?;

    for result in results {
        println!("{}", result);
    }
    Ok(())
}
