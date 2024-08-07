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
    fn bump_pre(&mut self, preid: Option<&str>) -> String;
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
    /// // default preid: beta
    /// let mut version = parse("1.2.3");
    /// assert_eq!(version.bump_pre(None), "1.3.3-beta.1");
    ///
    /// // default pre value: 1
    /// version = parse("1.2.3-rc");
    /// assert_eq!(version.bump_pre(None), "1.2.3-rc.1");
    ///
    /// version = parse("1.2.3-beta.4");
    /// assert_eq!(version.bump_pre(None), "1.2.3-beta.5");
    /// assert_eq!(version.bump_pre(Some("rc")), "1.2.3-rc.1");
    /// ```
    fn bump_pre(&mut self, preid: Option<&str>) -> String {
        let mut has_preid = false;
        let current_preid;
        match preid {
            Some(value) => {
                if value.trim().len() == 0 {
                    current_preid = "beta";
                } else {
                    current_preid = value.trim();
                    has_preid = true;
                }
            }
            None => {
                current_preid = "beta";
            }
        }

        if self.pre.is_empty() {
            self.minor += 1;
            self.pre = Prerelease::new(format!("{}.{}", current_preid, "1").as_str()).unwrap();
            return self.to_string();
        }

        let mut pre_arr = self.pre.as_str().split(".").collect::<Vec<&str>>();
        if pre_arr.len() == 0 {
            pre_arr.push(&current_preid);
            pre_arr.push("1");
            self.pre = Prerelease::new(pre_arr.join(".").as_str()).unwrap();
            return self.to_string();
        }

        let pre_value = pre_arr.pop().unwrap();
        #[allow(unused_assignments)]
        let mut next_value = pre_value.to_string();

        match pre_value.parse::<u32>() {
            Ok(value) => {
                if has_preid {
                    if current_preid == pre_arr.join(".") {
                        next_value = (value + 1).to_string();
                        pre_arr.push(next_value.as_str());
                    } else {
                        pre_arr.clear();
                        pre_arr.push(&current_preid);
                        pre_arr.push("1");
                    }
                } else {
                    if pre_arr.len() == 0 {
                        pre_arr.push(&current_preid);
                    }

                    next_value = (value + 1).to_string();
                    pre_arr.push(next_value.as_str());
                }
            }
            Err(_) => {
                if has_preid {
                    pre_arr.clear();
                    pre_arr.push(&current_preid);
                } else {
                    pre_arr.push(pre_value);
                }

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
