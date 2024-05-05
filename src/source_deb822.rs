use std::{fmt, fs, path::Path, str::FromStr};

use oma_debcontrol::Paragraph;

use crate::{sources_list, SourceEntry, SourceError, SourceResult};

#[derive(Clone, Debug, PartialEq)]
pub struct SourceListDeb822 {
    entries: Vec<SourceEntry>,
}

impl fmt::Display for SourceListDeb822 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self)
    }
}

impl FromStr for SourceListDeb822 {
    type Err = SourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = oma_debcontrol::parse_str(s)
            .map_err(|e| SourceError::SyntaxError { why: e.to_string() })?;

        let mut entries = vec![];

        for i in p {
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
                suite: i
                    .fields
                    .iter()
                    .find(|x| x.name == "Suites")
                    .map(|x| x.value.to_string())
                    .ok_or(SourceError::MissingField { field: "Suites" })?,
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
            };

            entries.push(entry);
        }

        Ok(Self { entries })
    }
}

fn deb822_options(i: &Paragraph) -> Option<String> {
    let s = i
        .fields
        .iter()
        .filter(|x| !["Types", "URIS", "Suites", "Components"].contains(&x.name))
        .map(|x| format!("{}={}", x.name, x.value))
        .collect::<Vec<_>>()
        .join(",");

    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

impl SourceListDeb822 {
    pub fn new<P: AsRef<Path>>(path: P) -> SourceResult<Self> {
        let path = path.as_ref();
        let data = fs::read_to_string(path).map_err(|why| SourceError::SourcesListOpen {
            path: path.to_path_buf(),
            why,
        })?;

        let sources_file = data.parse::<SourceListDeb822>()?;

        Ok(sources_file)
    }
    /// Scans every file in **/etc/apt/sources.list.d**, including **/etc/apt/sources.list**.
    ///
    /// Note that this will parse every source list into memory before returning.
    pub fn scan() -> SourceResult<Vec<Self>> {
        scan_inner("/")
    }

    /// Scans every file in **/etc/apt/sources.list.d**, including **/etc/apt/sources.list**. (from root argument)
    ///
    /// Note that this will parse every source list into memory before returning.
    pub fn scan_from_root<P: AsRef<Path>>(root: P) -> SourceResult<Vec<Self>> {
        scan_inner(root)
    }

    pub fn new_from_paths<P: AsRef<Path>, I: Iterator<Item = P>>(
        paths: I,
    ) -> SourceResult<Vec<Self>> {
        let files = paths
            .map(SourceListDeb822::new)
            .collect::<SourceResult<Vec<SourceListDeb822>>>()?;

        Ok(files)
    }

    pub fn entries(&self) -> impl Iterator<Item = &SourceEntry> {
        self.entries.iter()
    }
}

fn scan_inner<P: AsRef<Path>>(dir: P) -> Result<Vec<SourceListDeb822>, SourceError> {
    let paths = sources_list(dir)?;

    SourceListDeb822::new_from_paths(paths.iter())
}
