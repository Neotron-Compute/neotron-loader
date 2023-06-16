//! Traits to help us load information from different kinds of data source.

// ============================================================================
// Imports
// ============================================================================

// ============================================================================
// Constants
// ============================================================================

// ============================================================================
// Static Variables
// ============================================================================

// ============================================================================
// Types
// ============================================================================

/// The error raised if you are reading from a [`Source`] which is a slice of
/// bytes.
#[derive(Debug, Clone)]
pub struct SliceError;

/// Describes something we can read data from
pub trait Source {
    type Error: core::fmt::Debug;

    /// Read some bytes from the source.
    ///
    /// The bytes are read from the given offset, and there must be enough data
    /// to fill `buffer` completely, otherwise an error is returned.
    fn read(&self, offset: u32, buffer: &mut [u8]) -> Result<(), Self::Error>;

    /// Read a 32-bit big-endian value.
    fn read_u32_be(&self, offset: u32) -> Result<u32, Self::Error> {
        let mut bytes = [0; 4];
        self.read(offset, &mut bytes)?;
        Ok(u32::from_be_bytes(bytes))
    }

    /// Read a 32-bit little-endian value.
    fn read_u32_le(&self, offset: u32) -> Result<u32, Self::Error> {
        let mut bytes = [0; 4];
        self.read(offset, &mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    /// Read a 16-bit big-endian value.
    fn read_u16_be(&self, offset: u32) -> Result<u16, Self::Error> {
        let mut bytes = [0; 2];
        self.read(offset, &mut bytes)?;
        Ok(u16::from_be_bytes(bytes))
    }

    /// Read a 16-bit little-endian value.
    fn read_u16_le(&self, offset: u32) -> Result<u16, Self::Error> {
        let mut bytes = [0; 2];
        self.read(offset, &mut bytes)?;
        Ok(u16::from_le_bytes(bytes))
    }

    /// Read an 8-bit value.
    fn read_u8(&self, offset: u32) -> Result<u8, Self::Error> {
        let mut bytes = [0; 1];
        self.read(offset, &mut bytes)?;
        Ok(bytes[0])
    }
}

impl Source for &[u8] {
    type Error = SliceError;

    fn read(&self, offset: u32, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let desired_len = buffer.len();
        assert!(offset < (usize::MAX - desired_len) as u32);
        let offset = offset as usize;
        if let Some(sub_slice) = self.get(offset..offset + desired_len) {
            buffer.copy_from_slice(sub_slice);
            Ok(())
        } else {
            Err(SliceError)
        }
    }
}

// ============================================================================
// Functions
// ============================================================================

// ============================================================================
// Tests
// ============================================================================

// ============================================================================
// End of File
// ============================================================================
