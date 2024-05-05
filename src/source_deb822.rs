use std::{fmt, str::FromStr};

use oma_debcontrol::Paragraph;

use crate::{SourceEntry, SourceError};

#[derive(Clone, Debug, PartialEq)]
pub struct SourceListDeb822 {
    pub entries: Vec<SourceEntry>,
}

impl fmt::Display for SourceListDeb822 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut uris = vec![];
        for i in &self.entries {
            if uris.contains(&(&i.url, i.source)) {
                continue;
            }

            writeln!(fmt, "Types: {}", if i.source { "deb-src" } else { "deb" })?;
            writeln!(fmt, "URIs: {}", i.url)?;

            uris.push((&i.url, i.source));

            let suites = self
                .entries
                .iter()
                .filter(|x| x.url == i.url)
                .map(|x| x.suite.clone());
            write!(fmt, "Suites: ")?;
            for i in suites {
                write!(fmt, "{} ", i)?;
            }
            writeln!(fmt)?;

            writeln!(fmt, "Components: {}", i.components.join(" "))?;

            if let Some(opts) = &i.options {
                let opts = opts.split(',');

                for i in opts {
                    let (k, v) = i.split_once('=').unwrap();
                    writeln!(fmt, "{}: {}", k, v)?;
                }
            }
        }

        Ok(())
    }
}

impl FromStr for SourceListDeb822 {
    type Err = SourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = oma_debcontrol::parse_str(s)
            .map_err(|e| SourceError::SyntaxError { why: e.to_string() })?;

        let mut entries = vec![];

        for i in p {
            for j in i
                .fields
                .iter()
                .find(|x| x.name == "Suites")
                .map(|x| x.value.to_string())
                .ok_or(SourceError::MissingField { field: "Suites" })?
                .split_ascii_whitespace()
            {
                let entry = SourceEntry {
                    enabled: true,
                    source: i
                        .fields
                        .iter()
                        .find(|x| x.name == "Types")
                        .map(|x| x.value != "deb")
                        .unwrap_or(false),
                    options: deb822_options(&i),
                    url: i
                        .fields
                        .iter()
                        .find(|x| x.name == "URIs")
                        .map(|x| x.value.to_string())
                        .ok_or(SourceError::MissingField { field: "URIs" })?,
                    suite: j.to_string(),
                    components: i
                        .fields
                        .iter()
                        .find(|x| x.name == "Components")
                        .map(|x| x.value.to_string())
                        .ok_or(SourceError::MissingField {
                            field: "Components",
                        })?
                        .split_ascii_whitespace()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>(),
                    is_deb822: true,
                };

                entries.push(entry);
            }
        }

        Ok(Self { entries })
    }
}

fn deb822_options(i: &Paragraph) -> Option<String> {
    let s = i
        .fields
        .iter()
        .filter(|x| !["Types", "URIs", "Suites", "Components"].contains(&x.name))
        .map(|x| format!("{}={}", x.name, x.value))
        .collect::<Vec<_>>()
        .join(",");

    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

#[test]
fn test() {
    let sources = SourceListDeb822::from_str(
        r"Types: deb
URIs: https://mirrors.ustc.edu.cn/ubuntu
Suites: noble noble-updates noble-backports
Components: main restricted universe multiverse
Signed-By: /usr/share/keyrings/ubuntu-archive-keyring.gpg
",
    );

    assert_eq!(
        sources.unwrap(),
        SourceListDeb822 {
            entries: vec![
                SourceEntry {
                    enabled: true,
                    source: false,
                    options: Some(
                        "Signed-By=/usr/share/keyrings/ubuntu-archive-keyring.gpg".to_string()
                    ),
                    url: "https://mirrors.ustc.edu.cn/ubuntu".to_string(),
                    suite: "noble".to_string(),
                    components: vec![
                        "main".to_string(),
                        "restricted".to_string(),
                        "universe".to_string(),
                        "multiverse".to_string()
                    ],
                    is_deb822: true,
                },
                SourceEntry {
                    enabled: true,
                    source: false,
                    options: Some(
                        "Signed-By=/usr/share/keyrings/ubuntu-archive-keyring.gpg".to_string()
                    ),
                    url: "https://mirrors.ustc.edu.cn/ubuntu".to_string(),
                    suite: "noble-updates".to_string(),
                    components: vec![
                        "main".to_string(),
                        "restricted".to_string(),
                        "universe".to_string(),
                        "multiverse".to_string()
                    ],
                    is_deb822: true,
                },
                SourceEntry {
                    enabled: true,
                    source: false,
                    options: Some(
                        "Signed-By=/usr/share/keyrings/ubuntu-archive-keyring.gpg".to_string()
                    ),
                    url: "https://mirrors.ustc.edu.cn/ubuntu".to_string(),
                    suite: "noble-backports".to_string(),
                    components: vec![
                        "main".to_string(),
                        "restricted".to_string(),
                        "universe".to_string(),
                        "multiverse".to_string(),
                    ],
                    is_deb822: true,
                }
            ]
        }
    );
}
