//! Microsoft SxS assembly manifest generator
//!
//! # Examples
//!
//! ```
//! use sxs_manifest::*;
//!
//! # fn foo() -> error::SerializeResult<()> {
//! let mut manifest = AssemblyManifest::default();
//! manifest.compatibility.supported_os.insert(manifest::SupportedOS::Windows10);
//! let manifest = manifest.serialize_to_string()?;
//! # Ok(())
//! # }
//! ```
//!
#![deny(missing_docs)]

use std::io::Write;

/// XML writer config.
/// Re-exported from xml-rs.
pub use xml::writer::EmitterConfig;

mod consts;
mod debug;
/// Detailed types of manifest
pub mod manifest;
mod serialize;
mod xml_alias {
    pub use xml::{
        attribute::Attribute as XmlAttribute,
        common::XmlVersion,
        name::Name as XmlName,
        namespace::{self, Namespace as XmlNamespace},
    };
}

pub use manifest::AssemblyManifest;
use serialize::SerializeResult;

#[allow(missing_docs)]
pub mod error {
    pub use crate::serialize::{SerializeError, SerializeResult};
}

/// Serialization helper methods
impl AssemblyManifest {
    /// Serialize with default writer config
    pub fn serialize<W: Write>(&self, writer: W) -> SerializeResult<W> {
        self.serialize_with_config(EmitterConfig::new(), writer)
    }

    /// Serialize into string
    pub fn serialize_to_string_with_config(
        &self,
        config: EmitterConfig,
    ) -> SerializeResult<String> {
        let buf: Vec<u8> = Vec::new();

        let buf = self.serialize_with_config(config, buf)?;

        Ok(String::from_utf8(buf).unwrap())
    }

    /// Serialize into string with default writer config
    pub fn serialize_to_string(&self) -> SerializeResult<String> {
        self.serialize_to_string_with_config(EmitterConfig::new())
    }
}

#[test]
fn test_empty_manifest() {
    let manifest = AssemblyManifest::default();
    let mut config = EmitterConfig::new();
    config.indent_string = "".into();
    config.line_separator = "".into();
    let serialized = manifest.serialize_to_string_with_config(config).unwrap();

    assert_eq!(serialized, include_str!("tests/empty.xml"));
}

#[test]
fn test_supported_os_single() {
    use manifest::*;

    let mut manifest = AssemblyManifest::default();
    manifest
        .compatibility
        .supported_os
        .insert(SupportedOS::Windows10);
    let mut config = EmitterConfig::new();
    config.indent_string = "".into();
    config.line_separator = "".into();
    let serialized = manifest.serialize_to_string_with_config(config).unwrap();

    assert_eq!(serialized, include_str!("tests/supported_os_single.xml"));
}

#[test]
fn test_max_version_tested() {
    use manifest::*;

    let mut manifest = AssemblyManifest::default();
    manifest.compatibility.max_version_tested = Some(AssemblyVersion {
        major: 10,
        minor: 0,
        build: 18358,
        revision: Some(0),
    });
    let mut config = EmitterConfig::new();
    config.indent_string = "".into();
    config.line_separator = "".into();
    assert!(manifest
        .serialize_to_string_with_config(config.clone())
        .is_err());
    manifest
        .compatibility
        .supported_os
        .insert(SupportedOS::Windows10);
    let serialized = manifest.serialize_to_string_with_config(config).unwrap();

    assert_eq!(serialized, include_str!("tests/max_version_tested.xml"));
}
