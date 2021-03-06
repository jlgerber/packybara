/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
//! Structures designed to generate packages.xml
//!
use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;
use std::mem;
/// Determine if an XML element is closed (eg <foo />)
pub trait IsClosed {
    /// Indicates whether or not a Xml Node has contents or not
    fn is_closed(&self) -> bool;
}

impl IsClosed for XMLElement {
    fn is_closed(&self) -> bool {
        self.contents.is_none() && self.text.is_none()
    }
}

/// Top level element
#[xml_element("show")]
pub struct Show {
    #[sxs_type_attr]
    name: String,
    #[sxs_type_element(rename = "packages")]
    packages: Packages,
    #[sxs_type_element(rename = "roles")]
    roles: Roles,
}

impl Show {
    /// New up a Show instance
    ///
    /// #Arguments
    ///
    /// * `show` - The name of the show
    ///
    /// # Returns
    /// * Show instance
    pub fn new<I: Into<String>>(show: I) -> Self {
        Self {
            name: show.into(),
            packages: Packages::new(),
            roles: Roles::new(),
        }
    }

    /// Add a Package instance to the list of packages in the show
    ///
    /// # Arguments
    ///
    /// * `package` - A Package instance
    ///
    /// # Returns
    /// * None
    pub fn add_package(&mut self, package: Package) {
        self.packages.push(package)
    }

    /// Add a Package instance to the list of packages in the show and
    /// return Self. Used as part of a builder pattern.
    ///
    /// # Arguments
    ///
    /// * `package` - A Package instance
    ///
    /// # Returns
    /// * Self
    pub fn add_package_owned(mut self, package: Package) -> Self {
        self.packages.push(package);
        self
    }

    /// Add a Role instance to the list of roles on the show
    ///
    /// # Arguments
    ///
    /// * `role` - A Role instance
    ///
    /// # Returns
    /// * None
    pub fn add_role(&mut self, role: Role) {
        self.roles.push(role)
    }

    /// Add a Role instance to the list of roles on the show and return
    /// Self. Used as part of a builder pattern.
    ///
    /// # Arguments
    ///
    /// * `role` - A Role instance
    ///
    /// # Returns
    /// * Self
    pub fn add_role_owned(mut self, role: Role) -> Self {
        self.roles.push(role);
        self
    }
}

/// Element which represents a parent tag whose contents is a list of packages.
/// (ie  <package>... </package>)
#[xml_element("package")]
pub struct Packages {
    #[sxs_type_multi_element(rename = "package")]
    package: Vec<Package>,
}

impl Packages {
    /// New up an empty Packages instance.
    ///
    /// # Arguments
    ///
    /// * None
    ///
    /// # Returns
    ///
    /// * Packages instance
    pub fn new() -> Self {
        Self {
            package: Vec::new(),
        }
    }

    /// Add a package to the list of package contents
    ///
    /// # Arguments
    ///
    /// * `package` - Package instance
    ///
    /// # Returns
    /// * None
    pub fn push(&mut self, package: Package) {
        self.package.push(package)
    }
}

/// Rust representation of the xml element with the `package` tag. Unfortunately,
/// the tag would more accurately be considered a distribution than a package.
/// However, we stick with Package to be consistent with the output xml file.
#[xml_element("package")]
pub struct Package {
    /// Name of the distribution
    #[sxs_type_attr]
    name: String,
    /// Version of the distribution
    #[sxs_type_attr]
    version: String,
    /// Optional platform name
    #[sxs_type_attr]
    os: Option<String>,
    /// Optional site name
    #[sxs_type_attr]
    site: Option<String>,
    /// withs contents
    #[sxs_type_multi_element(rename = "with")]
    withs: Vec<With>,
}

impl Package {
    /// New up a Package isntance. (Note that we use the terminology of the packages.xml target file.
    /// However, the Package struct should probably be named Distribution)
    pub fn new<I: Into<String>>(name: I, version: I, os: Option<I>, site: Option<I>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            withs: Vec::new(),
            os: os.map(|x| x.into()),
            site: site.map(|x| x.into()),
        }
    }

    /// New up a Package instance given the name and version of a distribution
    ///
    /// # Arguments
    ///
    /// * `name` - The package name
    /// * `version` - The package version
    ///
    /// # Returns
    /// * Package instance
    pub fn from_name_and_version<I: Into<String>>(name: I, version: I) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            withs: Vec::new(),
            os: None,
            site: None,
        }
    }

    /// Construct a Package from a distribution, returning an Option
    ///
    /// # Arguments
    ///
    /// * `dist` - The distribution, eg foo-1.2.3
    ///
    /// # Returns
    /// * Some(package) if successful
    /// * None otherwise
    pub fn from_dist(dist: &str) -> Option<Self> {
        if let &[name, version] = &*dist.split("-").collect::<Vec<_>>() {
            Some(Self {
                name: name.into(),
                version: version.into(),
                withs: Vec::new(),
                os: None,
                site: None,
            })
        } else {
            None
        }
    }

    /// Add a `with`ß to the list of withs
    ///
    /// # Arguments
    ///
    /// * `with` - An Option wrapped with package name
    pub fn add_with(&mut self, with: With) {
        self.withs.push(with);
    }

    /// Add a with package, returning Self. Used as part of a builder pattern.
    ///
    /// # Arguments
    ///
    /// * `with` - An Option wrapped with package name
    ///
    /// # Returns
    /// * Self
    pub fn add_with_owned(mut self, with: With) -> Self {
        self.withs.push(with);
        self
    }

    /// Set the os field.
    ///
    /// # Arguments
    ///
    /// * `os` - An Option wrapped platform name
    pub fn set_os<I: Into<String>>(&mut self, os: Option<I>) {
        self.os = os.map(|x| x.into());
    }
    /// Set the os field and return Self. Used as part of a builder pattern
    ///
    /// # Arguments
    ///
    /// * `os` - An Option wrapped platform name
    ///
    /// # Returns
    /// * Self
    pub fn set_os_owned<I: Into<String>>(mut self, os: Option<I>) -> Self {
        self.os = os.map(|x| x.into());
        self
    }

    /// Set the site.
    ///
    /// # Arguments
    ///
    /// * `site` - An Option wrapped site name
    ///
    /// Returns
    /// None
    pub fn set_site<I: Into<String>>(&mut self, site: Option<I>) {
        self.site = site.map(|x| x.into());
    }

    /// Set the site and return Self. Used as part of a builder pattern
    ///
    /// # Arguments
    ///
    /// * `site` - An Option wrapped site name
    ///
    /// # Returns
    ///
    /// * Self
    pub fn set_site_owned<I: Into<String>>(mut self, site: Option<I>) -> Self {
        self.site = site.map(|x| x.into());
        self
    }
}

/// Element that represents a named package.
#[xml_element("with")]
pub struct With {
    #[sxs_type_attr]
    package: String,
}

impl With {
    /// New up a With
    ///
    /// # Arguments
    ///
    /// * `package` - Name of the package
    ///
    /// # Returns
    /// * With instance
    pub fn new<I: Into<String>>(package: I) -> Self {
        Self {
            package: package.into(),
        }
    }
}

/// The element whose contents is a list of Role instances.
/// (ie <role>...</role>)
#[xml_element("role")]
pub struct Roles {
    #[sxs_type_multi_element(rename = "role")]
    role: Vec<Role>,
}
impl Roles {
    /// New up an instance of Roles
    pub fn new() -> Roles {
        Roles { role: Vec::new() }
    }
    /// Add a role into the list of child roles
    ///
    /// # Arguments
    ///
    /// * `role` - An instance of type Role
    pub fn push(&mut self, role: Role) {
        self.role.push(role)
    }

    pub fn last_role(&self) -> Option<&str> {
        let role_len = self.role.len();
        if role_len == 0 {
            return None;
        }
        Some(
            // get unchecked TODO
            self.role[role_len - 1].name.as_str(),
        )
    }
}

/// Role element contains a list of Package instances
#[xml_element("role")]
pub struct Role {
    /// Name of the Role
    #[sxs_type_attr]
    name: String,
    /// packages tag
    #[sxs_type_element(rename = "packages")]
    packages: Packages,
}

impl Role {
    /// New up a Role instance
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the Role, provided as a type which implements
    /// Into<String>
    ///
    /// # Returns
    /// * Role instance
    pub fn new<I: Into<String>>(name: I) -> Self {
        Self {
            name: name.into(),
            packages: Packages::new(),
        }
    }

    /// Add a Package instance to the list of contents of the node
    ///
    /// # Arguments
    ///
    /// * `package` - Package instance
    ///
    /// # Returns
    /// * None
    pub fn add_package(&mut self, package: Package) {
        self.packages.push(package);
    }

    /// add a package to the packages managed by the role, returning Self. Used
    /// as part of a builder pattern.
    ///
    /// # Parameters
    ///
    /// * `package` - An instance of Package
    pub fn add_package_owned(mut self, package: Package) -> Self {
        self.packages.push(package);
        self
    }
}

/// Converter which generates an XMLElement Tree from an entry which impls  Into<XMLElement>
///
/// # Example
///
/// ```rust
/// use simple_xml_serialize::XMLElement;
///
/// fn main() {
///      let mut show = Show::new("FACILITY");
///
///      show.add_package(
///      Package::new("maya", "2018.5.1", None, None)
///         .set_os_owned(Some("cent7_64"))
///         .add_with_owned(With::new("xerces"))
///         .add_with_owned(With::new("mayapipeline"))
///      );
///
///     let xml = ToXml::new().to_xml(show);
/// }
pub struct ToXml {
    pub prune_closed: bool,
}

impl ToXml {
    /// New up an instance of the ToXml converter
    pub fn new() -> Self {
        Self { prune_closed: true }
    }
    /// Convert entry into xml element provided it implw Into<XMLElemet>
    ///
    /// # Arguments
    ///
    /// * `entry` - instance of an item that implements Into<XLElement>
    ///
    /// # Returns
    /// * XMLElement instance
    pub fn to_xml(&self, entry: impl Into<XMLElement>) -> XMLElement {
        let mut xml = entry.into();
        if self.prune_closed {
            Self::prune_closed_contents(&mut xml);
        }
        xml
    }

    /// Given an element, generate a pretty string rep
    ///
    /// # Arguments
    ///
    /// * `elem` - A reference to an XMLElement
    ///
    /// # Returns
    /// * A pretty formatted string representation of the xml element
    pub fn to_pretty_string(elem: &XMLElement) -> String {
        elem.to_string_pretty("\n", "  ")
    }

    /// Given an element, generate a string representation
    ///
    /// # Arguments
    ///
    /// * `elem` - A reference to an XMLElement
    ///
    /// # Returns
    /// * A formatted string representation of the xml element
    pub fn to_string(elem: &XMLElement) -> String {
        elem.to_string()
    }

    // Given an XMLElement node, prune its closed contents. This is used
    // to remove empty roles in the case that it is empty in the Snow element.alloc//
    //
    // # Arguments
    //
    // * `elem` - mutable reference to an XMLElement
    fn prune_closed_contents(elem: &mut XMLElement) {
        if elem.contents.is_some() {
            let contents = elem.contents.take().unwrap();
            let contents = contents
                .into_iter()
                .filter(|x| !x.is_closed())
                .collect::<Vec<_>>();
            let mut contents = if contents.len() > 0 {
                Some(contents)
            } else {
                None
            };
            mem::swap(&mut elem.contents, &mut contents);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_serialize_show_no_role() {
        let mut show = Show::new("FACILITY");
        show.add_package(
            Package::new("maya", "2018.5.1", None, None)
                .set_os_owned(Some("cent7_64"))
                .add_with_owned(With::new("xerces"))
                .add_with_owned(With::new("mayapipeline")),
        );
        show.add_package(
            Package::new("houdini", "17.5.432", None, None)
                .add_with_owned(With::new("houd_pipeline"))
                .add_with_owned(With::new("houd_camera")),
        );
        let converter = ToXml::new();
        let xml = converter.to_xml(show);
        assert_eq!(xml.to_string_pretty("\n", "  ").as_str(),
        "<show name=\"FACILITY\">\n  <packages>\n    <package name=\"maya\" version=\"2018.5.1\" os=\"cent7_64\">\n      <withs package=\"xerces\"/>\n      <withs package=\"mayapipeline\"/>\n    </package>\n    <package name=\"houdini\" version=\"17.5.432\">\n      <withs package=\"houd_pipeline\"/>\n      <withs package=\"houd_camera\"/>\n    </package>\n  </packages>\n</show>"
        );
    }
    #[test]
    fn can_serialize_show_no_withs() {
        let mut show = Show::new("FACILITY");
        show.add_package(Package::new("maya", "2018.5.1", None, None));
        show.add_package(Package::new("houdini", "17.5.432", None, None));
        let converter = ToXml::new();
        let xml = converter.to_xml(show);
        assert_eq!(xml.to_string_pretty("\n", "  ").as_str(), 
        "<show name=\"FACILITY\">\n  <packages name=\"maya\" version=\"2018.5.1\"/>\n  <packages name=\"houdini\" version=\"17.5.432\"/>\n</show>"
        );
    }

    #[test]
    fn can_serialize_show_with_roles() {
        let mut show = Show::new("FACILITY");
        show.add_package(
            Package::new("maya", "2018.5.1", None, None)
                .add_with_owned(With::new("xerces"))
                .add_with_owned(With::new("mayapipeline")),
        );
        show.add_package(
            Package::new("houdini", "17.5.432", None, None)
                .add_with_owned(With::new("houd_pipeline"))
                .add_with_owned(With::new("houd_camera")),
        );

        show.add_role(
            Role::new("model")
                .add_package_owned(
                    Package::new("maya", "2020.1.0", None, None)
                        .add_with_owned(With::new("xerces"))
                        .add_with_owned(With::new("mayapipeline"))
                        .add_with_owned(With::new("modelpipeline")),
                )
                .add_package_owned(Package::new("zbrush", "14", None, None))
                .add_package_owned(
                    Package::new("atomic", "1.2.3", None, None)
                        .add_with_owned(With::new("vray"))
                        .add_with_owned(With::new("vray_for_maya")),
                ),
        );

        let xml = XMLElement::from(show);
        assert_eq!(
            xml.to_string_pretty("\n", "  ").as_str(), 
            "<show name=\"FACILITY\">\n  <packages>\n    <package name=\"maya\" version=\"2018.5.1\">\n      <withs package=\"xerces\"/>\n      <withs package=\"mayapipeline\"/>\n    </package>\n    <package name=\"houdini\" version=\"17.5.432\">\n      <withs package=\"houd_pipeline\"/>\n      <withs package=\"houd_camera\"/>\n    </package>\n  </packages>\n  <roles>\n    <role name=\"model\">\n      <packages>\n        <package name=\"maya\" version=\"2020.1.0\">\n          <withs package=\"xerces\"/>\n          <withs package=\"mayapipeline\"/>\n          <withs package=\"modelpipeline\"/>\n        </package>\n        <package name=\"zbrush\" version=\"14\"/>\n        <package name=\"atomic\" version=\"1.2.3\">\n          <withs package=\"vray\"/>\n          <withs package=\"vray_for_maya\"/>\n        </package>\n      </packages>\n    </role>\n  </roles>\n</show>"
        );
    }
}

pub mod xml {
    use crate::db::find_all::versionpins::FindAllVersionPinsError;
    use crate::db::traits::PBFind;
    use crate::io;
    use crate::packrat::PackratDb;
    use crate::LtreeSearchMode;
    use crate::SearchAttribute;
    use crate::{Platform, Role, Site};
    use log;
    use snafu::{ResultExt, Snafu};
    use std::fs::File;
    use std::io::Write;

    /// Error type returned from  FindAllPackagesError
    #[derive(Debug, Snafu)]
    pub enum PackagesXmlError {
        /// PackybaraDbQueryError - error when calling try_from_parts
        #[snafu(display("Error querying the database  {}: {}", msg, source))]
        PackybaraDbQueryError {
            msg: &'static str,
            source: FindAllVersionPinsError, //std::boxed::Box<dyn std::error::Error>,
        },
        ///PackybaraDbWriteError
        #[snafu(display("Error writing to the database  {}: {}", msg, source))]
        PackybaraDbWriteError {
            msg: &'static str,
            source: std::boxed::Box<dyn std::error::Error>,
        },
        ///FileSystemWriteError
        #[snafu(display("Error writing to the database  {}: {}", msg, source))]
        IoError {
            msg: &'static str,
            source: std::io::Error,
        },
    }

    pub fn write_xml<'a>(
        db: &'a mut PackratDb,
        show: String,
        output: String,
    ) -> Result<(), PackagesXmlError> {
        // get a list of version pins for the show
        //let mut db = PackratDb::new(client);
        let vpins = db
            .find_all_versionpins()
            .isolate_facility(true)
            .level(show.as_str())
            .search_mode(LtreeSearchMode::Descendant)
            .order_by(vec![SearchAttribute::Role, SearchAttribute::Package])
            .query()
            .context(PackybaraDbQueryError {
                msg: "Unable to get version pins from db",
            })?;
        // get a list of withs for the show
        // iterate through version pins, creating appropriate data structure for outgoing
        let mut show = io::Show::new(show);
        let mut last_role: Option<Role> = None;
        let mut role_packages = Vec::new();
        for row in vpins {
            let package = row.distribution.package();
            let version = row.distribution.version();
            let site = row.coords.site();
            let platform = row.coords.platform();
            // TODO: do not know how seq/shot is handled in packages.xml
            //let level = row.coords.level(); // hwo is this handled?
            let role = row.coords.role();
            let mut package = io::Package::new(package, version, None, None);
            if let Some(withs) = row.withs {
                for with in withs {
                    package.add_with(io::With::new(with));
                }
            }
            if site != &Site::Any {
                package.set_site(Some(site.to_string()));
            }
            if platform != &Platform::Any {
                package.set_os(Some(platform.to_string()));
            }
            if role != &Role::Any {
                let role_str = role.to_string();
                log::debug!("role != Any - role: {}", role_str.as_str());
                // if our last iter was a role
                if let Some(ref last) = last_role {
                    let last_role_str = last.to_string();
                    log::debug!("extracted last role = {}", last_role_str.as_str());
                    // if the current role is the same as the last
                    // role, we add the package into our list
                    if role == last {
                        role_packages.push(package);
                    } else {
                        // otherwise we drain the list of saved packages,
                        // adding them in to the current role
                        let mut role_tag = io::Role::new(last_role_str);
                        for pkg in role_packages.drain(..) {
                            role_tag.add_package(pkg);
                        }
                        log::debug!("adding {} role to show", &role_tag.name);
                        show.add_role(role_tag);
                        // and we push the current package into our list,
                        // which is now empty
                        role_packages.push(package);
                    }
                } else {
                    log::debug!("last role was None");
                    // in the case where our last iter was NOT a role
                    // role packages should be zero sized
                    assert_eq!(role_packages.len(), 0);
                    // add in the package
                    role_packages.push(package);
                }
                log::debug!("setting role: {} to last_role", role.to_string().as_str());
                last_role = Some(role.clone());
            } else {
                // handle remaining
                if let Some(role) = last_role {
                    let role_str = role.to_string();
                    log::debug!("role == Any. last_role = {}", role_str.as_str());
                    let mut role_tag = io::Role::new(role_str);
                    for pkg in role_packages.drain(..) {
                        role_tag.add_package(pkg);
                    }
                    log::debug!("adding {} role to show", &role_tag.name);
                    show.add_role(role_tag);
                }
                show.add_package(package);
                last_role = None;
            }
        }
        // serialise to disk
        let xml_writer = io::ToXml::new();
        let show = xml_writer.to_xml(show);
        let xml_str = io::ToXml::to_pretty_string(&show);
        let mut output = File::create(output).context(IoError {
            msg: "Unable to create packages.xml on disk",
        })?;

        write!(output, "{}", xml_str).context(IoError {
            msg: "Unable to write packages.xml to disk",
        })?;

        Ok(())
    }
}
