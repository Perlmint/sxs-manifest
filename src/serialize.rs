use crate::debug::Path;
use std::{borrow::Cow, io::Write};
use xml::{writer::Error as XmlError, EventWriter};

/// Error can be occured while serialization.
///
/// It can be partially written, even though error is occured.
#[derive(Debug, thiserror::Error)]
pub enum SerializeError {
    /// Error from XmlWriter
    #[error("XmlWrite failed - {0}")]
    XmlWriteError(#[from] XmlError),
    /// Manifest is invalid.
    #[error("Invalid data found at {path}. {detail}")]
    Invalid {
        /// Path of invalid value from manifest root
        path: String,
        /// Detailed reason. It can be a hint to fix error.
        detail: String,
    },
}

/// Serialization result
pub type SerializeResult<R> = std::result::Result<R, SerializeError>;

pub trait SerializableElement {
    fn serialize<W: Write>(
        &self,
        writer: &mut EventWriter<W>,
        path: Path<'_>,
    ) -> SerializeResult<()>;
}

pub trait SerializableValue {
    fn serialize(&self) -> Cow<'_, str>;
}
