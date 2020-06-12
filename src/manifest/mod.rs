use crate::debug::Path;
use crate::xml_alias::{namespace, XmlAttribute, XmlName, XmlNamespace, XmlVersion};
use crate::{
    consts::NS_MS_ASM_V1,
    serialize::{SerializableElement, SerializableValue, SerializeResult},
};
use std::borrow::Cow;
use std::io::Write;
use xml::writer::{EmitterConfig, XmlEvent};

mod compatibility;
pub use compatibility::*;
mod common;
pub use common::*;
mod dependency;
pub use dependency::*;

/// Version of manifest
///
/// Only 1.0 is valid.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ManifestVersion {
    /// 1.0
    V1_0,
}

impl SerializableValue for ManifestVersion {
    fn serialize(&self) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            ManifestVersion::V1_0 => "1.0",
        })
    }
}

/// Assembly manifest
#[derive(Debug, Clone, PartialEq)]
pub struct AssemblyManifest {
    /// Version of manifest
    pub manifest_version: ManifestVersion,
    /// Compatibility info
    pub compatibility: Compatibility,
    /// Can specify SxS dependencies
    pub dependency: Dependency,
}

impl Default for AssemblyManifest {
    fn default() -> Self {
        AssemblyManifest {
            manifest_version: ManifestVersion::V1_0,
            compatibility: Compatibility::default(),
            dependency: Dependency::default(),
        }
    }
}

impl AssemblyManifest {
    const ELEMENT_NAME: XmlName<'static> = XmlName {
        local_name: "assembly",
        namespace: Some(NS_MS_ASM_V1),
        prefix: None,
    };
    const ATTRIBUTE_MANIFEST_VERSION_NAME: XmlName<'static> = XmlName {
        local_name: "manifestVersion",
        namespace: Some(NS_MS_ASM_V1),
        prefix: None,
    };
}

/// Implementation of common serialization
impl AssemblyManifest {
    /// Serialize manifest with custom config & writer
    pub fn serialize_with_config<W: Write>(
        &self,
        config: EmitterConfig,
        writer: W,
    ) -> SerializeResult<W> {
        let mut writer = config.create_writer(writer);
        writer.write(XmlEvent::StartDocument {
            version: XmlVersion::Version10,
            encoding: Some("UTF-8"),
            standalone: Some(true),
        })?;
        writer.write(XmlEvent::StartElement {
            name: AssemblyManifest::ELEMENT_NAME,
            attributes: Cow::Borrowed(&[XmlAttribute {
                name: AssemblyManifest::ATTRIBUTE_MANIFEST_VERSION_NAME,
                value: &self.manifest_version.serialize(),
            }]),
            namespace: Cow::Owned({
                let mut ns = XmlNamespace::empty();
                ns.put(namespace::NS_NO_PREFIX.to_string(), NS_MS_ASM_V1);
                ns
            }),
        })?;

        self.compatibility
            .serialize(&mut writer, Path::new("compatibility".into()))?;
        self.dependency
            .serialize(&mut writer, Path::new("dependency".into()))?;

        writer.write(XmlEvent::EndElement { name: None })?;

        Ok(writer.into_inner())
    }
}
