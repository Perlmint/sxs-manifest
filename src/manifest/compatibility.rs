use super::common::AssemblyVersion;
use crate::xml_alias::{namespace, XmlAttribute, XmlName, XmlNamespace};
use crate::{
    consts::{NS_MS_ASM_V1, NS_MS_COMPAT_V1},
    debug::Path,
    serialize::{SerializableElement, SerializableValue, SerializeError, SerializeResult},
};
use std::{borrow::Cow, collections::HashSet, io::Write};
use xml::{writer::XmlEvent, EventWriter};

/// SupportedOS
///
/// reference [https://docs.microsoft.com/en-us/windows/win32/sysinfo/targeting-your-application-at-windows-8-1](https://docs.microsoft.com/en-us/windows/win32/sysinfo/targeting-your-application-at-windows-8-1)
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum SupportedOS {
    /// WindowServer2016, WindowServer2019
    Windows10,
    /// WindowsServer2012R2
    Windows8_1,
    /// WindowsServer2012
    Windows8,
    /// WindowsServer2008R2
    Windows7,
    /// WindowServer2008
    WindowsVista,
}

impl SupportedOS {
    const ELEMENT_NAME: XmlName<'static> = XmlName {
        local_name: "supportedOS",
        namespace: Some(NS_MS_ASM_V1),
        prefix: None,
    };
    const ATTRIBUTE_ID_NAME: XmlName<'static> = XmlName {
        local_name: "Id",
        namespace: Some(NS_MS_ASM_V1),
        prefix: None,
    };
}

impl SerializableValue for SupportedOS {
    fn serialize(&self) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            SupportedOS::Windows10 => "{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}",
            SupportedOS::Windows8_1 => "{1f676c76-80e1-4239-95bb-83d0f6d0da78}",
            SupportedOS::Windows8 => "{4a2f28e3-53b9-4441-ba9c-d69d4a4a6e38}",
            SupportedOS::Windows7 => "{35138b9a-5d96-4fbd-8e2d-a2440225f93a}",
            SupportedOS::WindowsVista => "{e2011457-1546-43c5-a5fe-008deee3d3f0}",
        })
    }
}

/// Predefined Windows versions
///
/// This versions are used with [`Compatibility::max_version_tested`](../struct.Compatibility.html#structfield.max_version_tested)
///
/// reference: [https://docs.microsoft.com/en-us/windows/release-information/](https://docs.microsoft.com/en-us/windows/release-information/)
pub mod windows_version {
    #![allow(missing_docs)]
    use super::AssemblyVersion;
    pub const WINDOWS_10_1507: AssemblyVersion = AssemblyVersion::new(10, 0, 10240, Some(0));
    pub const WINDOWS_10_1511: AssemblyVersion = AssemblyVersion::new(10, 0, 10586, Some(0));
    pub const WINDOWS_10_1607: AssemblyVersion = AssemblyVersion::new(10, 0, 14393, Some(0));
    pub const WINDOWS_10_1703: AssemblyVersion = AssemblyVersion::new(10, 0, 15063, Some(0));
    pub const WINDOWS_10_1709: AssemblyVersion = AssemblyVersion::new(10, 0, 16299, Some(0));
    pub const WINDOWS_10_1803: AssemblyVersion = AssemblyVersion::new(10, 0, 17134, Some(0));
    pub const WINDOWS_10_1809: AssemblyVersion = AssemblyVersion::new(10, 0, 17763, Some(0));
    pub const WINDOWS_10_1903: AssemblyVersion = AssemblyVersion::new(10, 0, 18362, Some(0));
    pub const WINDOWS_10_2004: AssemblyVersion = AssemblyVersion::new(10, 0, 19041, Some(0));
}

/// Compatibility info about assembly
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Compatibility {
    /// Supported os
    pub supported_os: HashSet<SupportedOS>,
    /// Maximum version of Windows thatt the application tested against.
    ///
    /// This value is required to use [XAML Islands](https://docs.microsoft.com/en-us/windows/apps/desktop/modernize/xaml-islands).
    pub max_version_tested: Option<AssemblyVersion>,
}

impl Compatibility {
    const ELEMENT_NAME: XmlName<'static> = XmlName {
        local_name: "compatibility",
        namespace: Some(NS_MS_COMPAT_V1),
        prefix: None,
    };
    const ELEMENT_APPLICATTION_NAME: XmlName<'static> = XmlName {
        local_name: "application",
        namespace: Some(NS_MS_COMPAT_V1),
        prefix: None,
    };
    const ELEMENT_MAXVERSION_TESTED_NAME: XmlName<'static> = XmlName {
        local_name: "maxversiontested",
        namespace: Some(NS_MS_COMPAT_V1),
        prefix: None,
    };
    const ATTRIBUTE_ID_NAME: XmlName<'static> = XmlName {
        local_name: "Id",
        namespace: Some(NS_MS_COMPAT_V1),
        prefix: None,
    };
}

impl SerializableElement for Compatibility {
    fn serialize<W: Write>(
        &self,
        writer: &mut EventWriter<W>,
        path: Path<'_>,
    ) -> SerializeResult<()> {
        if self.supported_os.is_empty() {
            return if self.max_version_tested.is_some() {
                Err(SerializeError::Invalid {
                    path: format!("{}.{}", &path, "maxversion_tested"),
                    detail: "maxversion_tested requires at least one supported_os".to_string(),
                })?
            } else {
                Ok(())
            };
        }

        writer.write(XmlEvent::StartElement {
            name: Compatibility::ELEMENT_NAME,
            attributes: Cow::Borrowed(&[]),
            namespace: Cow::Owned({
                let mut ns = XmlNamespace::empty();
                ns.put(namespace::NS_NO_PREFIX.to_string(), NS_MS_COMPAT_V1);
                ns
            }),
        })?;

        writer.write(XmlEvent::StartElement {
            name: Compatibility::ELEMENT_APPLICATTION_NAME,
            attributes: Cow::Borrowed(&[]),
            namespace: Cow::Owned(XmlNamespace::empty()),
        })?;

        if let Some(maxversion) = &self.max_version_tested {
            writer.write(XmlEvent::StartElement {
                name: Compatibility::ELEMENT_MAXVERSION_TESTED_NAME,
                attributes: Cow::Borrowed(&[XmlAttribute {
                    name: Compatibility::ATTRIBUTE_ID_NAME,
                    value: &maxversion.serialize(),
                }]),
                namespace: Cow::Owned(XmlNamespace::empty()),
            })?;
            writer.write(XmlEvent::EndElement { name: None })?;
        }

        for os in &self.supported_os {
            writer.write(XmlEvent::StartElement {
                name: SupportedOS::ELEMENT_NAME,
                attributes: Cow::Borrowed(&[XmlAttribute {
                    name: SupportedOS::ATTRIBUTE_ID_NAME,
                    value: &os.serialize(),
                }]),
                namespace: Cow::Owned(XmlNamespace::empty()),
            })?;
            writer.write(XmlEvent::EndElement { name: None })?;
        }

        writer.write(XmlEvent::EndElement { name: None })?;

        writer.write(XmlEvent::EndElement { name: None })?;

        Ok(())
    }
}
