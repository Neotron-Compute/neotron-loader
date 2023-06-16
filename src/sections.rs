//! Code and Types for handling Sections
//!
//! These are only interesting to linkers.

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

/// Represents a section in the section table.
#[derive(Debug, Clone)]
pub struct Header {
    sh_name_offset: u32,
    sh_type: u32,
    sh_flags: u32,
    sh_addr: u32,
    sh_offset: u32,
    sh_size: u32,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u32,
    sh_entsize: u32,
}

impl Header {
    /// Size of a section header entry
    pub const SIZE_IN_BYTES: u16 = 0x28;

    /// Section header table entry unused
    pub const SHT_NULL: u32 = 0x0;

    /// Program data    
    pub const SHT_PROGBITS: u32 = 0x1;

    /// Symbol table
    pub const SHT_SYMTAB: u32 = 0x2;

    /// String table
    pub const SHT_STRTAB: u32 = 0x3;

    /// Relocation entries with addends
    pub const SHT_RELA: u32 = 0x4;

    /// Symbol hash table
    pub const SHT_HASH: u32 = 0x5;

    /// Dynamic linking information
    pub const SHT_DYNAMIC: u32 = 0x6;

    /// Notes
    pub const SHT_NOTE: u32 = 0x7;

    /// Program space with no data (bss)
    pub const SHT_NOBITS: u32 = 0x8;

    /// Relocation data, no addends
    pub const SHT_REL: u32 = 0x9;

    /// Dynamic linker symbol table
    pub const SHT_DYNSYM: u32 = 0x0B;

    /// Array of constructors
    pub const SHT_INIT_ARRAY: u32 = 0x0E;

    /// Array of destructors
    pub const SHT_FINI_ARRAY: u32 = 0x0F;

    /// Array of pre-constructors
    pub const SHT_PREINIT_ARRAY: u32 = 0x10;

    /// Section group
    pub const SHT_GROUP: u32 = 0x11;

    /// Extended section indicies
    pub const SHT_SYMTAB_SHNDX: u32 = 0x12;

    /// Create a new section header.
    pub fn new<DS>(loader: &Loader<DS>, idx: u16) -> Result<Self, Error<DS::Error>>
    where
        DS: Source,
    {
        let section_table_offset = loader.e_shoff + u32::from(Self::SIZE_IN_BYTES) * u32::from(idx);

        let sh_name_offset = loader.data_source.read_u32_le(section_table_offset)?;
        let sh_type = loader
            .data_source
            .read_u32_le(section_table_offset + 0x04)?;
        let sh_flags = loader
            .data_source
            .read_u32_le(section_table_offset + 0x08)?;
        let sh_addr = loader
            .data_source
            .read_u32_le(section_table_offset + 0x0C)?;
        let sh_offset = loader
            .data_source
            .read_u32_le(section_table_offset + 0x10)?;
        let sh_size = loader
            .data_source
            .read_u32_le(section_table_offset + 0x14)?;
        let sh_link = loader
            .data_source
            .read_u32_le(section_table_offset + 0x18)?;
        let sh_info = loader
            .data_source
            .read_u32_le(section_table_offset + 0x1C)?;
        let sh_addralign = loader
            .data_source
            .read_u32_le(section_table_offset + 0x20)?;
        let sh_entsize = loader
            .data_source
            .read_u32_le(section_table_offset + 0x24)?;

        Ok(Self {
            sh_name_offset,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset,
            sh_size,
            sh_link,
            sh_info,
            sh_addralign,
            sh_entsize,
        })
    }

    /// Return the `sh_name_offset` field    
    pub fn sh_name_offset(&self) -> u32 {
        self.sh_name_offset
    }

    /// Get the string name for this section.
    pub fn sh_name<'a, DS: Source>(
        &self,
        loader: &Loader<DS>,
        buffer: &'a mut [u8],
    ) -> Result<&'a str, Error<DS::Error>> {
        let string_section_idx = loader.e_shstrndx;
        let string_section_header = Self::new(loader, string_section_idx)?;
        let string_start = string_section_header.sh_offset + self.sh_name_offset;

        for b in buffer.iter_mut() {
            *b = 0x00;
        }

        loader.data_source.read(string_start, buffer)?;

        // If this returns an error, our buffer doesn't have a null in it. Which means we used all the bytes.
        let cstr =
            core::ffi::CStr::from_bytes_until_nul(buffer).map_err(|_| Error::NotEnoughSpace)?;

        if let Ok(s) = cstr.to_str() {
            Ok(s)
        } else {
            Err(Error::InvalidString)
        }
    }

    /// Return the `sh_type` field        
    pub fn sh_type(&self) -> u32 {
        self.sh_type
    }

    /// Return the `sh_flags` field        
    pub fn sh_flags(&self) -> u32 {
        self.sh_flags
    }

    /// Return the `sh_addr` field        
    pub fn sh_addr(&self) -> u32 {
        self.sh_addr
    }

    /// Return the `sh_offset` field        
    pub fn sh_offset(&self) -> u32 {
        self.sh_offset
    }

    /// Return the `sh_size` field        
    pub fn sh_size(&self) -> u32 {
        self.sh_size
    }

    /// Return the `sh_link` field        
    pub fn sh_link(&self) -> u32 {
        self.sh_link
    }

    /// Return the `sh_info` field        
    pub fn sh_info(&self) -> u32 {
        self.sh_info
    }

    /// Return the `sh_addralign` field        
    pub fn sh_addralign(&self) -> u32 {
        self.sh_addralign
    }

    /// Return the `sh_entsize` field        
    pub fn sh_entsize(&self) -> u32 {
        self.sh_entsize
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
