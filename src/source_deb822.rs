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

            if i.trusted {
                writeln!(fmt, "Trusted: yes")?;
            }

            if let Some(signed_by) = &i.signed_by {
                writeln!(fmt, "Signed-By:{}", signed_by)?;
            }

            for j in &i.options {
                writeln!(fmt, "{}: {}", j.0, j.1.join(" "))?;
            }
        }

        Ok(())
    }
}

impl FromStr for SourceListDeb822 {
    type Err = SourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sources_list: Repositories =
            s.parse().map_err(|e| SourceError::SyntaxError { why: e })?;

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
                            components: source.components.clone().unwrap_or(vec![]),
                            is_deb822: true,
                            options: p
                                .items()
                                .filter(|x| {
                                    ![
                                        "Enabled",
                                        "Types",
                                        "URIs",
                                        "Suites",
                                        "Components",
                                        "Architectures",
                                        "Signed-By",
                                        "Trusted",
                                    ]
                                    .contains(&x.0.as_str())
                                })
                                .map(|x| {
                                    (
                                        x.0,
                                        x.1.split_ascii_whitespace()
                                            .map(|x| x.to_string())
                                            .collect::<Vec<_>>(),
                                    )
                                })
                                .collect::<Vec<_>>(),
                            archs: source.architectures.clone(),
                            trusted: source.trusted.unwrap_or(false),
                            signed_by: source.signature.clone(),
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
fn test_deb822_flat_repo() {
    use crate::deb822::signature::Signature;

    let s = r#"Types: deb
URIs: https://github.com/CrossPaste/crosspaste-desktop/releases/latest/download/
Suites: ./
Signed-By: /etc/apt/trusted.gpg.d/crosspaste.asc
"#;

    let sources = SourceListDeb822::from_str(s);

    assert_eq!(
        sources.unwrap(),
        SourceListDeb822 {
            entries: vec![SourceEntry {
                enabled: true,
                source: false,
                options: vec![],
                url: "https://github.com/CrossPaste/crosspaste-desktop/releases/latest/download/".to_string(),
                suite: "./".to_string(),
                components: vec![],
                signed_by: Some(Signature::KeyPath(vec![
                    "/etc/apt/trusted.gpg.d/crosspaste.asc".into()
                ])),
                is_deb822: true,
                archs: None,
                trusted: false,
            }]
        }
    );
}

#[test]
fn test_parse_deb822() {
    use crate::deb822::signature::Signature;
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
                    options: vec![],
                    url: "http://security.ubuntu.com/ubuntu/".to_string(),
                    suite: "noble-security".to_string(),
                    components: vec![
                        "restricted".to_string(),
                        "universe".to_string(),
                        "multiverse".to_string(),
                        "main".to_string(),
                    ],
                    signed_by: Some(Signature::KeyPath(vec![
                        "/usr/share/keyrings/ubuntu-archive-keyring.gpg".into()
                    ])),
                    is_deb822: true,
                    archs: None,
                    trusted: false,
                },
                SourceEntry {
                    enabled: true,
                    source: true,
                    options: vec![],
                    url: "http://cn.archive.ubuntu.com/ubuntu/".to_string(),
                    suite: "noble".to_string(),
                    components: vec![
                        "restricted".to_string(),
                        "universe".to_string(),
                        "multiverse".to_string(),
                        "main".to_string(),
                    ],
                    is_deb822: true,
                    signed_by: Some(Signature::KeyPath(vec![
                        "/usr/share/keyrings/ubuntu-archive-keyring.gpg".into()
                    ])),
                    archs: None,
                    trusted: false,
                },
                SourceEntry {
                    enabled: true,
                    source: true,
                    options: vec![],
                    url: "http://cn.archive.ubuntu.com/ubuntu/".to_string(),
                    suite: "noble-security".to_string(),
                    components: vec![
                        "restricted".to_string(),
                        "universe".to_string(),
                        "multiverse".to_string(),
                        "main".to_string(),
                    ],
                    is_deb822: true,
                    archs: None,
                    signed_by: Some(Signature::KeyPath(vec![
                        "/usr/share/keyrings/ubuntu-archive-keyring.gpg".into()
                    ])),
                    trusted: false,
                },
                SourceEntry {
                    enabled: true,
                    source: false,
                    options: vec![],
                    url: "http://cn.archive.ubuntu.com/ubuntu/".to_string(),
                    suite: "noble".to_string(),
                    components: vec![
                        "restricted".to_string(),
                        "universe".to_string(),
                        "multiverse".to_string(),
                        "main".to_string(),
                    ],
                    is_deb822: true,
                    archs: None,
                    signed_by: Some(Signature::KeyPath(vec![
                        "/usr/share/keyrings/ubuntu-archive-keyring.gpg".into()
                    ])),
                    trusted: false,
                },
            ]
        }
    );
}

#[test]
fn test_serialize_deb822() {
    use crate::deb822::signature::Signature;
    let sources = SourceListDeb822 {
        entries: vec![
            SourceEntry {
                enabled: true,
                source: false,
                options: vec![],
                url: "https://mirrors.ustc.edu.cn/ubuntu".to_string(),
                suite: "noble".to_string(),
                components: vec![
                    "main".to_string(),
                    "restricted".to_string(),
                    "universe".to_string(),
                    "multiverse".to_string(),
                ],
                is_deb822: true,
                archs: None,
                signed_by: Some(Signature::KeyPath(vec![
                    "/usr/share/keyrings/ubuntu-archive-keyring.gpg".into(),
                ])),
                trusted: false,
            },
            SourceEntry {
                enabled: true,
                source: false,
                options: vec![],
                url: "https://mirrors.ustc.edu.cn/ubuntu".to_string(),
                suite: "noble-updates".to_string(),
                components: vec![
                    "main".to_string(),
                    "restricted".to_string(),
                    "universe".to_string(),
                    "multiverse".to_string(),
                ],
                is_deb822: true,
                archs: None,
                signed_by: Some(Signature::KeyPath(vec![
                    "/usr/share/keyrings/ubuntu-archive-keyring.gpg".into(),
                ])),
                trusted: false,
            },
            SourceEntry {
                enabled: true,
                source: false,
                options: vec![],
                url: "https://mirrors.ustc.edu.cn/ubuntu".to_string(),
                suite: "noble-backports".to_string(),
                components: vec![
                    "main".to_string(),
                    "restricted".to_string(),
                    "universe".to_string(),
                    "multiverse".to_string(),
                ],
                is_deb822: true,
                archs: None,
                signed_by: Some(Signature::KeyPath(vec![
                    "/usr/share/keyrings/ubuntu-archive-keyring.gpg".into(),
                ])),
                trusted: false,
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
