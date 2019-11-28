/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Role {
    name: String,
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
        Role { name: parts }
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
    pub fn from_str<I: Into<String>>(role: I) -> Self {
        //let parts = role.split("_").map(|x| x.to_string()).collect::<Vec<_>>();
        let role = role.into();
        assert_eq!(role.matches(" ").count(), 0);
        assert_eq!(role.matches("__").count(), 0);
        assert!(role.chars().next() != Some('_'));
        Role { name: role.into() }
    }

    /// len returns the depth of the role hierarchy. Parent roles
    /// have a len of 1, and subroles have a len of 2 or greater.
    pub fn len(&self) -> usize {
        self.name.matches("_").count() + 1
    }

    /// Indicates whether or not a role is a subrole or a parent role
    pub fn is_subrole(&self) -> bool {
        self.len() > 1
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
    /// let role_child = Role::from_str("model_beta");
    /// let role_parent = Role::from_str("model");
    /// assert!(role_parent.is_ancestor_of(&role_child));
    /// ```
    pub fn is_ancestor_of(&self, other: &Role) -> bool {
        other.name.starts_with(&self.name)
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
    /// let role_child = Role::from_str("model_beta");
    /// let role_parent = Role::from_str("model");
    /// assert!(role_child.is_child_of(&role_parent));
    /// ```
    pub fn is_child_of(&self, other: &Role) -> bool {
        self.name.starts_with(&other.name)
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Index<usize> for Role {
    type Output = str;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.name.split("_").collect::<Vec<_>>()[idx]
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
            Role {
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
        let role = Role {
            name: "model_beta".to_string(),
        };

        let role_s = role.to_string();
        assert_eq!(role_s.as_str(), "model_beta");
    }

    #[test]
    fn can_identify_ancestor() {
        let role_child = Role::from_str("model_beta");
        let role_parent = Role::from_str("model");
        assert!(role_parent.is_ancestor_of(&role_child));
    }

    #[test]
    fn can_identify_parent() {
        let role_child = Role::from_str("model_beta");
        let role_parent = Role::from_str("model");
        assert!(role_child.is_child_of(&role_parent));
    }
}
