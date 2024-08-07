#![warn(missing_docs)]

//! [Semantic Versioning](https://semver.org/) is a guideline for how version numbers are assigned and incremented. This crate is based on [semver](https://docs.rs/semver) and has some extended functions, such as **bump version**.

// #![warn(missing_docs)]
use semver::{Prerelease, Version};

/// Bump trait can be used to define bump version
pub trait Bump {
    /// bump major and return version string
    fn bump_major(&mut self) -> String;

    /// bump minor and return version string
    fn bump_minor(&mut self) -> String;

    /// bump patch and return version string
    fn bump_patch(&mut self) -> String;

    /// bump pre and return version string
    fn bump_pre(&mut self) -> String;
}
impl Bump for Version {
    /// ```
    /// use crate::core_semver::{parse,Bump};
    ///
    /// let mut version = core_semver::parse("1.2.3-beta.4");
    /// assert_eq!(version.bump_major(), "2.0.0");
    /// ```
    fn bump_major(&mut self) -> String {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
        self.pre = Prerelease::EMPTY;
        self.to_string()
    }

    /// ```
    /// use crate::core_semver::{parse,Bump};
    ///
    /// let mut version = parse("1.2.3-beta.4");
    /// assert_eq!(version.bump_minor(), "1.3.0");
    /// ```
    fn bump_minor(&mut self) -> String {
        self.minor += 1;
        self.patch = 0;
        self.pre = Prerelease::EMPTY;
        self.to_string()
    }

    /// ```
    /// use crate::core_semver::{parse,Bump};
    ///
    /// let mut version = parse("1.2.3-beta.4");
    /// assert_eq!(version.bump_patch(), "1.2.4");
    /// ```
    fn bump_patch(&mut self) -> String {
        self.patch += 1;
        self.pre = Prerelease::EMPTY;
        self.to_string()
    }

    /// ```
    /// use crate::core_semver::{parse,Bump};
    ///
    /// let mut version = parse("1.2.3-beta.4");
    /// assert_eq!(version.bump_pre(), "1.2.3-beta.5");
    ///
    /// version = parse("1.2.3-rc");
    /// assert_eq!(version.bump_pre(), "1.2.3-rc.1");
    /// ```
    fn bump_pre(&mut self) -> String {
        let mut pre_arr = self.pre.as_str().split(".").collect::<Vec<&str>>();
        let pre_value = pre_arr.last().unwrap();
        #[allow(unused_assignments)]
        let mut next_value = pre_value.to_string();

        match pre_value.parse::<u32>() {
            Ok(value) => {
                pre_arr.pop();
                next_value = (value + 1).to_string();
                pre_arr.push(next_value.as_str());
            }
            Err(_) => {
                pre_arr.push("1");
            }
        }

        self.pre = Prerelease::new(pre_arr.join(".").as_str()).unwrap();

        self.to_string()
    }
}

/// parse version string based on [semver](https://docs.rs/semver)
///
/// ```
/// use core_semver::parse;
///
/// let version = parse("1.2.3-beta.4");
/// assert_eq!(version.major, 1);
/// assert_eq!(version.minor, 2);
/// assert_eq!(version.patch, 3);
/// assert_eq!(version.pre.as_str(), "beta.4");
/// assert_eq!(version.to_string(), "1.2.3-beta.4")
/// ```
pub fn parse(str: &str) -> Version {
    Version::parse(str).unwrap()
}
