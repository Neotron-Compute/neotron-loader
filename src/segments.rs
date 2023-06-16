//! Code and types for handling Segments.
//!
//! These live in the program header table. They are useful for building loaders.

// ============================================================================
// Imports
// ============================================================================

use crate::{Error, Loader, Source};

// ============================================================================
// Constants
// ============================================================================

// ============================================================================
// Static Variables
// ============================================================================

// ============================================================================
// Types
// ============================================================================

/// Represents a program header
#[derive(Debug, Clone)]
pub struct Header {
    p_type: u32,
    p_offset: u32,
    p_vaddr: u32,
    p_paddr: u32,
    p_filesz: u32,
    p_memsz: u32,
    p_flags: u32,
    p_align: u32,
}

impl Header {
    /// Size of a program header entry
    pub const SIZE_IN_BYTES: u16 = 0x20;

    /// Program header table entry unused.
    pub const PT_NULL: u32 = 0x00000000;
    /// Loadable segment.
    pub const PT_LOAD: u32 = 0x00000001;
    /// Dynamic linking information.
    pub const PT_DYNAMIC: u32 = 0x00000002;
    /// Interpreter information.
    pub const PT_INTERP: u32 = 0x00000003;
    /// Auxiliary information.
    pub const PT_NOTE: u32 = 0x00000004;
    /// Reserved.
    pub const PT_SHLIB: u32 = 0x00000005;
    /// Segment containing program header table itself.
    pub const PT_PHDR: u32 = 0x00000006;
    /// Thread-Local Storage template.
    pub const PT_TLS: u32 = 0x00000007;
    /// Stack.
    pub const PT_GNU_STACK: u32 = 0x6474E551;

    /// Create a new section header.
    pub fn new<DS>(loader: &Loader<DS>, idx: u16) -> Result<Self, Error<DS::Error>>
    where
        DS: Source,
    {
        let ph_table_offset = loader.e_phoff + u32::from(Self::SIZE_IN_BYTES) * u32::from(idx);

        let p_type = loader.data_source.read_u32_le(ph_table_offset)?;
        let p_offset = loader.data_source.read_u32_le(ph_table_offset + 0x04)?;
        let p_vaddr = loader.data_source.read_u32_le(ph_table_offset + 0x08)?;
        let p_paddr = loader.data_source.read_u32_le(ph_table_offset + 0x0C)?;
        let p_filesz = loader.data_source.read_u32_le(ph_table_offset + 0x10)?;
        let p_memsz = loader.data_source.read_u32_le(ph_table_offset + 0x14)?;
        let p_flags = loader.data_source.read_u32_le(ph_table_offset + 0x18)?;
        let p_align = loader.data_source.read_u32_le(ph_table_offset + 0x1C)?;

        Ok(Self {
            p_type,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_flags,
            p_align,
        })
    }

    /// Get the `p_type` field.
    ///
    /// This is the type of segment, e.g. `PT_LOAD`.
    pub fn p_type(&self) -> u32 {
        self.p_type
    }

    /// Get the `p_offset` field
    ///
    /// This is the start of the segment data within this ELF file.
    pub fn p_offset(&self) -> u32 {
        self.p_offset
    }

    /// Get the `p_vaddr` field
    ///
    /// This is the virtual memory load address.
    pub fn p_vaddr(&self) -> u32 {
        self.p_vaddr
    }

    /// Get the `p_paddr` field
    ///
    /// This is the physical memory load address.
    pub fn p_paddr(&self) -> u32 {
        self.p_paddr
    }

    /// Get the `p_filesz` field
    ///
    /// This is how much space is used by this segment on disk.
    pub fn p_filesz(&self) -> u32 {
        self.p_filesz
    }

    /// Get the `p_memsz` field
    ///
    /// This is how much space is used by this segment in RAM.
    pub fn p_memsz(&self) -> u32 {
        self.p_memsz
    }

    /// Get the `p_flags` field
    ///
    /// A bitfield indicating whether the segment is executable (`0x01`),
    /// writable (`0x02`) or readable (`0x04`).
    pub fn p_flags(&self) -> u32 {
        self.p_flags
    }

    /// Get the `p_align` field
    ///
    /// 0 or 1 means no alignment, otherwise is a power-of-2 indicating
    /// alignment for this segment.
    pub fn p_align(&self) -> u32 {
        self.p_align
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
