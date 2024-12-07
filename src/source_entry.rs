use deb822::signature::Signature;

use super::*;
use std::fmt;
use std::str::FromStr;

/// An apt source entry that is active on the system.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SourceEntry {
    /// Whether the entry is enabled or not.
    pub enabled: bool,
    /// Whether this is a binary or source repo.
    pub source: bool,
    /// Some repos may have special options defined.
    pub options: Vec<(String, Vec<String>)>,
    /// The URL of the repo.
    pub url: String,
    /// The suite of the repo would be as `bionic` or `cosmic`.
    pub suite: String,
    /// Components that have been enabled for this repo.
    pub components: Vec<String>,
    /// Architectures binaries from this repository run on
    pub archs: Option<Vec<String>>,
    /// signed-by
    pub signed_by: Option<Signature>,
    /// Trusted
    pub trusted: bool,
    pub is_deb822: bool,
}

impl fmt::Display for SourceEntry {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.is_deb822 {
            // deb822 的情况跟 lines 的情况不一样
            // deb822 是一个结构体内放好几个 suite
            // 而 lines 只能放一个
            Err(fmt::Error)
        } else {
            if !self.enabled {
                fmt.write_str("# ")?;
            }

            fmt.write_str(if self.source { "deb-src " } else { "deb " })?;
            let mut options_string = vec![];
            if !self.options.is_empty() {
                for (k, v) in &self.options {
                    options_string.push(format!("{k}={}", v.join(",")));
                }
            }

            if !options_string.is_empty() {
                write!(fmt, "[{}] ", options_string.join(" "))?;
            }

            write!(
                fmt,
                "{} {} {}",
                self.url,
                self.suite,
                self.components.join(" ")
            )
        }
    }
}

impl FromStr for SourceEntry {
    type Err = SourceError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut components = Vec::new();
        let mut options = None;
        let url;

        let mut fields = line.split_whitespace();

        let source = match fields
            .next()
            .ok_or(SourceError::MissingField { field: "source" })?
        {
            "deb" => false,
            "deb-src" => true,
            other => {
                return Err(SourceError::InvalidValue {
                    field: "source",
                    value: other.to_owned(),
                })
            }
        };

        let field = fields
            .next()
            .ok_or(SourceError::MissingField { field: "url" })?;
        if let Some(field) = field.strip_prefix('[') {
            let mut leftover: Option<String> = None;
            let mut field = field.to_string();

            if let Some(pos) = field.find(']') {
                if pos == field.len() - 1 {
                    options = Some(field[..pos].into());
                } else {
                    options = Some(field[..pos].into());
                    leftover = Some(field[pos + 1..].into());
                }
            } else {
                loop {
                    let next = fields
                        .next()
                        .ok_or(SourceError::MissingField { field: "option" })?;
                    if let Some(pos) = next.find(']') {
                        field.push(' ');
                        field.push_str(&next[..pos]);
                        if pos != next.len() - 1 {
                            leftover = Some(next[pos + 1..].into());
                        }
                        break;
                    } else {
                        field.push(' ');
                        field.push_str(next);
                    }
                }

                options = Some(field);
                options = options.map(|x| x.trim().to_string());
            }

            url = match leftover {
                Some(field) => field,
                None => fields
                    .next()
                    .ok_or(SourceError::MissingField { field: "url" })?
                    .into(),
            };
        } else {
            url = field.into();
        }

        if options.as_ref().map_or(false, String::is_empty) {
            options = None;
        }

        let suite = fields
            .next()
            .ok_or(SourceError::MissingField { field: "suite" })?
            .into();

        for field in fields {
            components.push(field.into());
        }

        let mut options = options
            .map(|x| {
                x.split_ascii_whitespace()
                    .map(|x| x.split_once('=').unwrap_or((x, "")))
                    .map(|x| {
                        (
                            x.0.to_string(),
                            x.1.split(',').map(|x| x.to_string()).collect::<Vec<_>>(),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let mut archs = None;

        if let Some(pos) = options.iter().position(|x| x.0 == "arch") {
            archs = Some(options.remove(pos).1);
        }

        let mut signed_by = None;

        if let Some(pos) = options.iter().position(|x| x.0 == "signed-by") {
            signed_by = Some(Signature::KeyPath(
                options
                    .remove(pos)
                    .1
                    .iter()
                    .map(|x| x.into())
                    .collect::<Vec<_>>(),
            ))
        }

        let mut trusted = false;

        if let Some(pos) = options.iter().position(|x| x.0 == "trusted") {
            trusted = options.remove(pos).1.first().is_some_and(|x| x == "yes")
        }

        Ok(SourceEntry {
            enabled: true,
            source,
            url,
            suite,
            components,
            options,
            is_deb822: false,
            archs,
            signed_by,
            trusted,
        })
    }
}

impl SourceEntry {
    pub fn url(&self) -> &str {
        let mut url: &str = &self.url;
        while url.ends_with('/') {
            url = &url[..url.len() - 1];
        }

        url
    }

    /// The base filename to be used when storing files for this entries.
    pub fn filename(&self) -> String {
        let mut url = self.url();
        if let Some(pos) = url.find("//") {
            url = &url[pos..];
        }

        url.replace('/', "_")
    }

    /// Returns the root URL for this entry's dist path.
    ///
    /// For an entry such as:
    ///
    /// ```toml
    /// deb http://us.archive.ubuntu.com/ubuntu/ cosmic main
    /// ```
    ///
    /// The path that will be returned will be:
    ///
    /// ```toml
    /// http://us.archive.ubuntu.com/ubuntu/dists/cosmic
    /// ```
    pub fn dist_path(&self) -> String {
        [self.url(), "/dists/", &self.suite].concat()
    }

    pub fn dist_path_get(&self, path: &str) -> String {
        let url = self.url();
        [url, "/dists/", &self.suite, "/", path].concat()
    }

    /// Iterator that returns each of the dist components that are to be fetched.
    pub fn dist_components(&self) -> impl Iterator<Item = String> + '_ {
        let url = self.url();
        self.components
            .iter()
            .map(move |component| [url, "/dists/", &self.suite, "/", component].concat())
    }

    /// Returns the root URL for this entry's pool path.
    ///
    /// For an entry such as:
    ///
    /// ```toml
    /// deb http://us.archive.ubuntu.com/ubuntu/ cosmic main
    /// ```
    ///
    /// The path that will be returned will be:
    ///
    /// ```toml
    /// http://us.archive.ubuntu.com/ubuntu/pool/cosmic
    /// ```
    pub fn pool_path(&self) -> String {
        [self.url(), "/pool/"].concat()
    }
}
