/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Distribution {
    name: String,
}

impl Distribution {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Distribution { name: name.into() }
    }
    /// Retrieve the name of the package
    pub fn package(&self) -> &str {
        self.name.split("-").next().unwrap()
    }
    /// retrieve the version of the distribution
    pub fn version(&self) -> &str {
        self.name.split("-").skip(1).next().unwrap()
    }
    /// Retrieve the name of the distribution
    pub fn distribution(&self) -> &str {
        self.name.as_str()
    }
}
