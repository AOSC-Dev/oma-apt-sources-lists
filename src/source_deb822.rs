use std::{fmt, ops::Deref, str::FromStr};

use deb822_lossless::{Paragraph, ToDeb822Paragraph};

use crate::{
    deb822::{Repositories, RepositoryType},
    SourceEntry, SourceError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct SourceListDeb822 {
    pub entries: Vec<SourceEntry>,
}

impl fmt::Display for SourceListDeb822 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut uris = vec![];
        for i in &self.entries {
            if uris.contains(&(&i.url, i.source, &i.options)) {
                continue;
            }

            writeln!(fmt, "Types: {}", if i.source { "deb-src" } else { "deb" })?;
            writeln!(fmt, "URIs: {}", i.url)?;

            uris.push((&i.url, i.source, &i.options));

            let suites = self
                .entries
                .iter()
                .filter(|x| x.url == i.url && i.source == x.source && i.options == x.options)
                .map(|x| x.suite.clone());
            write!(fmt, "Suites: ")?;
            for i in suites {
                write!(fmt, "{} ", i)?;
            }
            writeln!(fmt)?;

            writeln!(fmt, "Components: {}", i.components.join(" "))?;

            for j in &i.options {
                let (k, v) = j.split_once('=').unwrap();
                writeln!(fmt, "{}: {}", k, v)?;
            }
        }

        Ok(())
    }
}

impl FromStr for SourceListDeb822 {
    type Err = SourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sources_list: Repositories = s.parse().map_err(|_| SourceError::SyntaxError {
            why: "".to_string(),
        })?;

        let mut entries = vec![];

        for source in sources_list.deref() {
            let p: Paragraph = source.to_paragraph();
            for suite in &source.suites {
                for url in &source.uris {
                    for source_type in &source.types {
                        let entry = SourceEntry {
                            enabled: source.enabled.unwrap_or(true),
                            source: *source_type == RepositoryType::Source,
                            url: url.to_string(),
                            suite: suite.to_string(),
                            components: source.components.clone(),
                            is_deb822: true,
                            options: p
                                .items()
                                .filter(|x| {
                                    !["Enabled", "Types", "URIs", "Suites", "Components"]
                                        .contains(&x.0.as_str())
                                })
                                .map(|x| format!("{}={}", x.0, x.1))
                                .collect::<Vec<_>>(),
                        };

                        entries.push(entry);
                    }
                }
            }
        }

        Ok(Self { entries })
    }
}

#[test]
fn test_parse_deb822() {
    let sources = SourceListDeb822::from_str(
        r"Types: deb
URIs: http://security.ubuntu.com/ubuntu/
Suites: noble-security
Components: restricted universe multiverse main
Signed-By: /usr/share/keyrings/ubuntu-archive-keyring.gpg

Types: deb-src
URIs: http://cn.archive.ubuntu.com/ubuntu/
Suites: noble noble-security
Components: restricted universe multiverse main
Signed-By: /usr/share/keyrings/ubuntu-archive-keyring.gpg

Types: deb
URIs: http://cn.archive.ubuntu.com/ubuntu/
Suites: noble
Components: restricted universe multiverse main
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
                    options: vec![
                        "Signed-By=/usr/share/keyrings/ubuntu-archive-keyring.gpg".to_string()
                    ],
                    url: "http://security.ubuntu.com/ubuntu/".to_string(),
                    suite: "noble-security".to_string(),
                    components: vec![
                        "restricted".to_string(),
                        "universe".to_string(),
                        "multiverse".to_string(),
                        "main".to_string(),
                    ],
                    is_deb822: true,
                },
                SourceEntry {
                    enabled: true,
                    source: true,
                    options: vec![
                        "Signed-By=/usr/share/keyrings/ubuntu-archive-keyring.gpg".to_string()
                    ],
                    url: "http://cn.archive.ubuntu.com/ubuntu/".to_string(),
                    suite: "noble".to_string(),
                    components: vec![
                        "restricted".to_string(),
                        "universe".to_string(),
                        "multiverse".to_string(),
                        "main".to_string(),
                    ],
                    is_deb822: true,
                },
                SourceEntry {
                    enabled: true,
                    source: true,
                    options: vec![
                        "Signed-By=/usr/share/keyrings/ubuntu-archive-keyring.gpg".to_string()
                    ],
                    url: "http://cn.archive.ubuntu.com/ubuntu/".to_string(),
                    suite: "noble-security".to_string(),
                    components: vec![
                        "restricted".to_string(),
                        "universe".to_string(),
                        "multiverse".to_string(),
                        "main".to_string(),
                    ],
                    is_deb822: true,
                },
                SourceEntry {
                    enabled: true,
                    source: false,
                    options: vec![
                        "Signed-By=/usr/share/keyrings/ubuntu-archive-keyring.gpg".to_string()
                    ],
                    url: "http://cn.archive.ubuntu.com/ubuntu/".to_string(),
                    suite: "noble".to_string(),
                    components: vec![
                        "restricted".to_string(),
                        "universe".to_string(),
                        "multiverse".to_string(),
                        "main".to_string(),
                    ],
                    is_deb822: true,
                },
            ]
        }
    );
}

#[test]
fn test_serialize_deb822() {
    let sources = SourceListDeb822 {
        entries: vec![
            SourceEntry {
                enabled: true,
                source: false,
                options: vec![
                    "Signed-By=/usr/share/keyrings/ubuntu-archive-keyring.gpg".to_string()
                ],
                url: "https://mirrors.ustc.edu.cn/ubuntu".to_string(),
                suite: "noble".to_string(),
                components: vec![
                    "main".to_string(),
                    "restricted".to_string(),
                    "universe".to_string(),
                    "multiverse".to_string(),
                ],
                is_deb822: true,
            },
            SourceEntry {
                enabled: true,
                source: false,
                options: vec![
                    "Signed-By=/usr/share/keyrings/ubuntu-archive-keyring.gpg".to_string()
                ],
                url: "https://mirrors.ustc.edu.cn/ubuntu".to_string(),
                suite: "noble-updates".to_string(),
                components: vec![
                    "main".to_string(),
                    "restricted".to_string(),
                    "universe".to_string(),
                    "multiverse".to_string(),
                ],
                is_deb822: true,
            },
            SourceEntry {
                enabled: true,
                source: false,
                options: vec![
                    "Signed-By=/usr/share/keyrings/ubuntu-archive-keyring.gpg".to_string()
                ],
                url: "https://mirrors.ustc.edu.cn/ubuntu".to_string(),
                suite: "noble-backports".to_string(),
                components: vec![
                    "main".to_string(),
                    "restricted".to_string(),
                    "universe".to_string(),
                    "multiverse".to_string(),
                ],
                is_deb822: true,
            },
        ],
    };

    assert_eq!(
        sources.to_string(),
        r#"Types: deb
URIs: https://mirrors.ustc.edu.cn/ubuntu
Suites: noble noble-updates noble-backports 
Components: main restricted universe multiverse
Signed-By: /usr/share/keyrings/ubuntu-archive-keyring.gpg
"#
    );
}
