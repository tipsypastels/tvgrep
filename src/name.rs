use anyhow::{Error, Result, ensure};
use kstring::KString;
use std::{
    cmp::{Ordering, Reverse},
    fmt,
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArticleName {
    pub group: GroupName,
    pub page: PageName,
}

impl ArticleName {
    pub fn display_without_main(&self) -> impl fmt::Display {
        struct DisplayWithoutMain<'a>(&'a ArticleName);
        impl fmt::Display for DisplayWithoutMain<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if self.0.group.is_main() {
                    write!(f, "{}", self.0.page)
                } else {
                    write!(f, "{}", self.0)
                }
            }
        }
        DisplayWithoutMain(self)
    }
}

impl FromStr for ArticleName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((group, page)) = s.split_once('/') {
            Ok(Self {
                group: group.parse()?,
                page: page.parse()?,
            })
        } else {
            Ok(Self {
                group: GroupName::default(),
                page: s.parse()?,
            })
        }
    }
}

impl fmt::Display for ArticleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.group, self.page)
    }
}

impl PartialEq<str> for ArticleName {
    fn eq(&self, other: &str) -> bool {
        if let Some((group, page)) = other.split_once('/') {
            &self.group == group && &self.page == page
        } else {
            self.group.is_main() && &self.page == other
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Group Name                                 */
/* -------------------------------------------------------------------------- */

const GROUP_NAME_MAIN: &str = "Main";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupName(KString);

impl GroupName {
    pub fn is_main(&self) -> bool {
        self.0 == GROUP_NAME_MAIN
    }
}

impl Default for GroupName {
    fn default() -> Self {
        Self(KString::from_static(GROUP_NAME_MAIN))
    }
}

impl FromStr for GroupName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        ensure!(!s.is_empty(), "group name must not be empty");
        Ok(Self(KString::from_ref(s)))
    }
}

impl fmt::Display for GroupName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialOrd for GroupName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GroupName {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.is_main())
            .cmp(&Reverse(other.is_main()))
            .then(self.0.cmp(&other.0))
    }
}

impl PartialEq<str> for GroupName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

/* -------------------------------------------------------------------------- */
/*                                  Page Name                                 */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PageName(KString);

impl FromStr for PageName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        ensure!(!s.is_empty(), "page name must not be empty");
        Ok(Self(KString::from_ref(s)))
    }
}

impl fmt::Display for PageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<str> for PageName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a(s: &str) -> ArticleName {
        s.parse().unwrap()
    }

    fn g(s: &str) -> GroupName {
        s.parse().unwrap()
    }

    #[test]
    fn article_name_display_defaults_to_main() {
        assert_eq!(a("Foo").to_string(), "Main/Foo");
    }

    #[test]
    fn article_name_display_without_main() {
        assert_eq!(a("Foo").display_without_main().to_string(), "Foo");
        assert_eq!(
            a("Other/Foo").display_without_main().to_string(),
            "Other/Foo"
        );
    }

    #[test]
    fn article_name_partial_eq() {
        assert_eq!(a("A/B"), *"A/B");
        assert_eq!(a("Foo"), *"Main/Foo");
        assert_eq!(a("Main/Foo"), *"Main/Foo");
    }

    #[test]
    fn article_name_main_sorts_first() {
        let subject = a("Foo");
        let before = a("A_Before/Bar");
        let after = a("Z_After/Baz");

        let mut items = vec![before.clone(), subject.clone(), after.clone()];
        items.sort();

        assert_eq!(items, [subject, before, after]);
    }

    #[test]
    fn group_name_main_sorts_first() {
        let subject = g(GROUP_NAME_MAIN);
        let before = g("A_Before");
        let after = g("Z_After");

        let mut items = vec![before.clone(), subject.clone(), after.clone()];
        items.sort();

        assert_eq!(items, [subject, before, after]);
    }
}
