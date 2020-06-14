use crate::xml_alias::{XmlAttribute, XmlName, XmlNamespace};
use crate::{
    consts::NS_MS_ASM_V1,
    debug::Path,
    serialize::{SerializableElement, SerializableValue, SerializeResult},
};
use std::{borrow::Cow, io::Write};
use xml::{writer::XmlEvent, EventWriter};

/// Type of assembly.
///
/// Only win32 is available
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum AssemblyType {
    #[allow(missing_docs)]
    Win32,
}

impl SerializableValue for AssemblyType {
    fn serialize(&self) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            AssemblyType::Win32 => "win32",
        })
    }
}

/// Supported process architecture
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ProcessArchitecture {
    /// x86
    X86,
    /// x86_64 / ia64 / amd64
    X86_64,
}

impl SerializableValue for ProcessArchitecture {
    fn serialize(&self) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            ProcessArchitecture::X86 => "x86",
            ProcessArchitecture::X86_64 => "ia64",
        })
    }
}

/// Specific version of assembly
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct AssemblyVersion {
    #[allow(missing_docs)]
    pub major: u32,
    #[allow(missing_docs)]
    pub minor: u32,
    #[allow(missing_docs)]
    pub build: u32,
    #[allow(missing_docs)]
    pub revision: Option<u32>,
}

impl AssemblyVersion {
    #[allow(missing_docs)]
    pub const fn new(major: u32, minor: u32, build: u32, revision: Option<u32>) -> Self {
        AssemblyVersion {
            major,
            minor,
            build,
            revision,
        }
    }
}

impl SerializableValue for AssemblyVersion {
    fn serialize(&self) -> Cow<'_, str> {
        Cow::Owned(format!(
            "{}.{}.{}.{}",
            self.major,
            self.minor,
            self.build,
            self.revision.unwrap_or(0)
        ))
    }
}

/// A 16-character hexadecimal string representing the last 8 bytes of the SHA-1 hash of the public key under which the application or assembly is signed.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PublicKeyToken(pub [u8; 8]);

impl SerializableValue for PublicKeyToken {
    fn serialize(&self) -> Cow<'_, str> {
        Cow::Owned(format!(
            "{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7]
        ))
    }
}

/// Specific assembly
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssemblyIdentity {
    #[allow(missing_docs)]
    pub r#type: AssemblyType,
    #[allow(missing_docs)]
    pub name: String,
    /// Language when assembly is language-specific
    pub language: Option<String>,
    #[allow(missing_docs)]
    pub process_architecture: Option<ProcessArchitecture>,
    #[allow(missing_docs)]
    pub version: Option<AssemblyVersion>,
    #[allow(missing_docs)]
    pub public_key_token: Option<PublicKeyToken>,
}

impl AssemblyIdentity {
    const ELEMENT_NAME: XmlName<'static> = XmlName {
        local_name: "assemblyIdentity",
        namespace: Some(NS_MS_ASM_V1),
        prefix: None,
    };

    const ATTRIBUTE_TYPE_NAME: XmlName<'static> = XmlName {
        local_name: "type",
        namespace: Some(NS_MS_ASM_V1),
        prefix: None,
    };
    const ATTRIBUTE_NAME_NAME: XmlName<'static> = XmlName {
        local_name: "name",
        namespace: Some(NS_MS_ASM_V1),
        prefix: None,
    };
    const ATTRIBUTE_LANGUAGE_NAME: XmlName<'static> = XmlName {
        local_name: "language",
        namespace: Some(NS_MS_ASM_V1),
        prefix: None,
    };
    const ATTRIBUTE_PROCESS_ARCHITECTURE_NAME: XmlName<'static> = XmlName {
        local_name: "processorArchitecture",
        namespace: Some(NS_MS_ASM_V1),
        prefix: None,
    };
    const ATTRIBUTE_VERSION_NAME: XmlName<'static> = XmlName {
        local_name: "version",
        namespace: Some(NS_MS_ASM_V1),
        prefix: None,
    };
    const ATTRIBUTE_PUBLIC_KEY_TOKEN_NAME: XmlName<'static> = XmlName {
        local_name: "publicKeyToken",
        namespace: Some(NS_MS_ASM_V1),
        prefix: None,
    };

    #[allow(missing_docs)]
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        AssemblyIdentity {
            r#type: AssemblyType::Win32,
            name: name.as_ref().to_string(),
            language: None,
            process_architecture: None,
            version: None,
            public_key_token: None,
        }
    }
}

impl SerializableElement for AssemblyIdentity {
    fn serialize<W: Write>(
        &self,
        writer: &mut EventWriter<W>,
        _path: Path<'_>,
    ) -> SerializeResult<()> {
        let mut attributes = Vec::<XmlAttribute>::new();

        let type_val = self.r#type.serialize();
        attributes.push(XmlAttribute {
            name: AssemblyIdentity::ATTRIBUTE_TYPE_NAME,
            value: &type_val,
        });

        attributes.push(XmlAttribute {
            name: AssemblyIdentity::ATTRIBUTE_NAME_NAME,
            value: &self.name,
        });

        if let Some(language) = &self.language {
            attributes.push(XmlAttribute {
                name: AssemblyIdentity::ATTRIBUTE_LANGUAGE_NAME,
                value: language,
            });
        }

        let process_architecture = if let Some(process_architecture) = &self.process_architecture {
            Some(process_architecture.serialize())
        } else {
            None
        };
        if let Some(process_architecture) = &process_architecture {
            attributes.push(XmlAttribute {
                name: AssemblyIdentity::ATTRIBUTE_PROCESS_ARCHITECTURE_NAME,
                value: process_architecture,
            });
        }

        let version = if let Some(version) = &self.version {
            Some(version.serialize())
        } else {
            None
        };
        if let Some(version) = &version {
            attributes.push(XmlAttribute {
                name: AssemblyIdentity::ATTRIBUTE_VERSION_NAME,
                value: version,
            });
        }

        let public_key_token = if let Some(public_key_token) = &self.public_key_token {
            Some(public_key_token.serialize())
        } else {
            None
        };
        if let Some(public_key_token) = &public_key_token {
            attributes.push(XmlAttribute {
                name: AssemblyIdentity::ATTRIBUTE_PUBLIC_KEY_TOKEN_NAME,
                value: public_key_token,
            });
        }

        writer.write(XmlEvent::StartElement {
            name: AssemblyIdentity::ELEMENT_NAME,
            attributes: Cow::Borrowed(&attributes),
            namespace: Cow::Borrowed(&XmlNamespace::empty()),
        })?;
        writer.write(XmlEvent::EndElement { name: None })?;

        Ok(())
    }
}
