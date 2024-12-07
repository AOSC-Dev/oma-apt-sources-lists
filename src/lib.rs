//! Crate for fetching detailed information from all available apt sources.
//!
//! The information retrieved from the provided `SourcesList` and accompanying iterator preserves
//! newlines and comments, so that these files can be modified and overwritten to preserve this data.
//!
//! Active source entries will be parsed into `SourceEntry`'s, which can be handled or serialized
//! back into text. Formatting of these lines are not preserved.

mod deb822;
mod errors;
pub mod source_deb822;
mod source_entry;
mod source_line;
mod sources_list;

#[cfg(test)]
mod tests;

pub use self::errors::*;
pub use self::source_entry::*;
pub use self::source_line::*;
pub use self::sources_list::*;
pub use deb822::signature::Signature;
