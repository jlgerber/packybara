/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use crate::coords_error::*;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::ops::Index;

/// This trait is specifically designed to get around the issue
/// with &String not implementing into for String
pub trait IntoString {
    fn into_string(self) -> String;
}

impl IntoString for &String {
    fn into_string(self) -> String {
        self.to_string()
    }
}
impl IntoString for &str {
    fn into_string(self) -> String {
        self.to_string()
    }
}

impl IntoString for String {
    fn into_string(self) -> String {
        self
    }
}

/// A Role may either be a parent, or subrole. Parent roles have
/// a single part internally, while subroles have multiple parts.
/// Internally, parts are stored as a vector of strings. However, Role
/// provides an abstraction that hides this, allowing us to deal with
/// more ergonomic representations.
///
/// The primary way of constructing a role is through the use of the
/// ```from_str``` constructor function. This takes an input and splits
/// it on `_`, resulting in a hierarchy of Strings internally.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, Hash)]
pub enum Role {
    Any,
    Named { name: String },
}

impl std::convert::AsRef<str> for Role {
    fn as_ref(&self) -> &str {
        match self {
            Role::Any => "any",
            Role::Named { name } => name,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Any => write!(f, "any"),
            Self::Named { ref name } => write!(f, "{}", name),
        }
    }
}

// test to make sure the role is ok
fn role_ok(name: &str) -> bool {
    name.matches(" ").count() == 0
        && name.matches("__").count() == 0
        && name.chars().next() != Some('_')
}

impl TryFrom<&str> for Role {
    type Error = CoordsError;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        if !role_ok(item) {
            return Err(CoordsError::FromStrToRoleError {
                input: item.to_string(),
            });
        }
        if item == "any" || item == "Any" {
            return Ok(Role::Any);
        }
        Ok(Role::Named {
            name: item.to_string(),
        })
    }
}

impl TryFrom<String> for Role {
    type Error = CoordsError;

    fn try_from(item: String) -> Result<Self, Self::Error> {
        if !role_ok(&item) {
            return Err(CoordsError::FromStrToRoleError {
                input: item.to_string(),
            });
        }
        if item == "any" || item == "Any" {
            return Ok(Role::Any);
        }
        Ok(Role::Named { name: item })
    }
}

impl Role {
    // TODO: Once i decide on error strategy, return Result<Self>
    /// New up a Role from a vector of &str,String, or &String
    ///
    /// # Arguments
    /// * `parts` - a Vector of str, &str, String, or &String comprising the role
    ///
    /// # Example
    ///
    /// ```
    /// use packybara::Role;
    ///
    /// let role = Role::from_parts(vec!["model", "beta"]);
    /// ```
    pub fn from_parts<I: AsRef<str>>(parts: Vec<I>) -> Self {
        let parts = parts
            .iter()
            .map(|x| x.as_ref())
            .collect::<Vec<&str>>()
            .join("_");
        Role::Named { name: parts }
    }

    // TODO: once i decide on error strategy, make this return Result<Self>
    /// Given a role string, convert to a role (eg foo_bar)
    ///
    /// # Arguments
    ///
    /// * `role` - The role as a str. (for example, `model_beta`)
    ///
    /// # Returns
    /// - Instance of Role
    ///
    /// # Example
    /// ```rust
    /// use packybara::Role;
    ///
    /// let role = Role::from_str("model_beta");
    /// ```
    pub fn from_str<I>(role: I) -> CoordsResult<Self>
    where
        I: Into<String>,
    {
        Role::try_from(role.into())
    }

    /// len returns the depth of the role hierarchy. Parent roles
    /// have a len of 1, and subroles have a len of 2 or greater.
    pub fn len(&self) -> usize {
        match self {
            Role::Any => 0,
            Role::Named { ref name } => name.matches("_").count() + 1,
        }
    }

    /// Indicates whether or not a role is a subrole or a parent role
    pub fn is_subrole(&self) -> bool {
        match self {
            Role::Any => false,
            Role::Named { ref name } => name.len() > 1,
        }
    }
    pub fn is_any(&self) -> bool {
        match self {
            Role::Any => true,
            _ => false,
        }
    }
    /// Test whether this role is an ancestor of another role.
    /// For instance, model is an ancestor of model_beta. For
    /// our purposes, model_beta is also an ancestor or model_beta.
    ///
    /// # Example
    ///
    /// ```rust
    /// use packybara::Role;
    ///
    /// let role_child = Role::from_str("model_beta").unwrap();
    /// let role_parent = Role::from_str("model").unwrap();
    /// assert!(role_parent.is_ancestor_of(&role_child));
    /// ```
    pub fn is_ancestor_of(&self, other: &Role) -> bool {
        // should this be true if both are any?
        if self.is_any() || other.is_any() {
            return false;
        }
        let me = if let Role::Named { ref name } = self {
            name
        } else {
            panic!("should not get here");
        };
        let other = if let Role::Named { ref name } = other {
            name
        } else {
            panic!("should not get here");
        };

        other.starts_with(me)
    }

    /// Test whether we are a child of another role
    ///
    /// # Arguments
    ///
    /// * `other` - The purported parent of Self
    ///
    /// # Returns
    /// - boolean indicating whether or not `other` is a child of `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// use packybara::Role;
    ///
    /// let role_child = Role::from_str("model_beta").unwrap();
    /// let role_parent = Role::from_str("model").unwrap();
    /// assert!(role_child.is_child_of(&role_parent));
    /// ```
    pub fn is_child_of(&self, other: &Role) -> bool {
        // should this be true if both are any?
        if self.is_any() || other.is_any() {
            return false;
        }
        let me = if let Role::Named { ref name } = self {
            name
        } else {
            panic!("should not get here");
        };
        let other = if let Role::Named { ref name } = other {
            name
        } else {
            panic!("should not get here");
        };
        me.starts_with(other)
    }
}

impl Index<usize> for Role {
    type Output = str;

    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            Role::Any => "any",
            Role::Named { ref name } => &name.split("_").collect::<Vec<_>>()[idx],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_initialize_from_parts() {
        let role = Role::from_parts(vec!["model", "beta"]);
        assert_eq!(
            role,
            Role::Named {
                name: "model_beta".to_string()
            }
        );
    }

    #[test]
    fn can_index_into() {
        let role = Role::from_parts(vec!["model", "beta"]);
        assert_eq!(&role[0], "model");
    }

    #[test]
    fn can_convert_to_string() {
        let role = Role::Named {
            name: "model_beta".to_string(),
        };

        let role_s = role.to_string();
        assert_eq!(role_s.as_str(), "model_beta");
    }

    #[test]
    fn can_identify_ancestor() {
        let role_child = Role::from_str("model_beta").unwrap();
        let role_parent = Role::from_str("model").unwrap();
        assert!(role_parent.is_ancestor_of(&role_child));
    }

    #[test]
    fn can_identify_parent() {
        let role_child = Role::from_str("model_beta").unwrap();
        let role_parent = Role::from_str("model").unwrap();
        assert!(role_child.is_child_of(&role_parent));
    }
}
