use super::common::AssemblyIdentity;
use crate::{
    consts::NS_MS_ASM_V1,
    debug::Path,
    serialize::{SerializableElement, SerializeResult},
    xml_alias::{XmlName, XmlNamespace},
};
use std::{borrow::Cow, io::Write};
use xml::{writer::XmlEvent, EventWriter};

/// Dependencies of assembly
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Dependency {
    /// Dependent assemblies
    pub dependent_assemblies: Vec<AssemblyIdentity>,
}

impl Dependency {
    const ELEMENT_NAME: XmlName<'static> = XmlName {
        local_name: "dependency",
        namespace: Some(NS_MS_ASM_V1),
        prefix: None,
    };

    const ELEMENT_ASSEMBLY_NAME: XmlName<'static> = XmlName {
        local_name: "dependentAssembly",
        namespace: Some(NS_MS_ASM_V1),
        prefix: None,
    };
}

impl SerializableElement for Dependency {
    fn serialize<W: Write>(
        &self,
        writer: &mut EventWriter<W>,
        path: Path<'_>,
    ) -> SerializeResult<()> {
        for (idx, assembly) in (&self.dependent_assemblies).into_iter().enumerate() {
            writer.write(XmlEvent::StartElement {
                name: Dependency::ELEMENT_NAME,
                attributes: Cow::Borrowed(&[]),
                namespace: Cow::Borrowed(&XmlNamespace::empty()),
            })?;
            writer.write(XmlEvent::StartElement {
                name: Dependency::ELEMENT_ASSEMBLY_NAME,
                attributes: Cow::Borrowed(&[]),
                namespace: Cow::Borrowed(&XmlNamespace::empty()),
            })?;

            assembly.serialize(writer, path.appended(idx.into()))?;

            writer.write(XmlEvent::EndElement { name: None })?;
            writer.write(XmlEvent::EndElement { name: None })?;
        }

        Ok(())
    }
}
