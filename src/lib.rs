//! # Neotron Loader
//!
//! Handles loading Neotron Executables into the Neotron OS.

#![no_std]

// ============================================================================
// Imports
// ============================================================================

pub mod sections;
pub mod segments;
pub mod traits;

#[doc(inline)]
pub use sections::Header as SectionHeader;

#[doc(inline)]
pub use segments::Header as ProgramHeader;

#[doc(inline)]
pub use traits::Source;

// ============================================================================
// Constants
// ============================================================================

// ============================================================================
// Static Variables
// ============================================================================

// ============================================================================
// Types
// ============================================================================

/// The ways this API can fail
#[derive(Debug, Clone)]
pub enum Error<E>
where
    E: core::fmt::Debug,
{
    /// The ELF file didn't look right
    NotAnElfFile,
    /// It was an ELF file, but not what Neotron can handle
    WrongElfFile,
    /// There was a problem with the data source.
    Source(E),
    /// Couldn't fit string into given buffer
    NotEnoughSpace,
    /// Section name wasn't UTF-8
    InvalidString,
}

impl<E> From<E> for Error<E>
where
    E: core::fmt::Debug,
{
    fn from(value: E) -> Error<E> {
        Error::Source(value)
    }
}

/// An object that can load and parse an ELF file.
pub struct Loader<DS> {
    /// Where we get the bytes from
    data_source: DS,
    /// The memory address of the entry point
    e_entry: u32,
    /// The offset of the program header table
    e_phoff: u32,
    /// The offset of the section header table
    e_shoff: u32,
    /// The number of program header entries
    e_phnum: u16,
    /// The number of section header entries
    e_shnum: u16,
    /// The index of the section header containing section names.
    e_shstrndx: u16,
}

impl<DS> Loader<DS>
where
    DS: Source,
{
    /// Indicates ARM machine
    const EM_ARM: u16 = 0x0028;
    /// For offset 0x10, indicates a binary
    const ET_EXEC: u16 = 0x0002;
    /// Standard ELF magic header
    const ELF_MAGIC: u32 = 0x7F454C46;
    /// 32-bit, little-endian, version 1, SysV
    const DESIRED_ELF_VERSION: u32 = 0x01010100;

    /// Make a new loader
    pub fn new(data_source: DS) -> Result<Loader<DS>, Error<DS::Error>> {
        let elf_header = data_source.read_u32_be(0x00)?;
        if elf_header != Self::ELF_MAGIC {
            // File doesn't start 0x7F E L F
            return Err(Error::NotAnElfFile);
        }
        let class_endian_version_abi = data_source.read_u32_be(0x04)?;
        if class_endian_version_abi != Self::DESIRED_ELF_VERSION {
            return Err(Error::WrongElfFile);
        }

        // Ignore ABI version at 0x08..0x10

        let elf_type = data_source.read_u16_le(0x10)?;
        if elf_type != Self::ET_EXEC {
            // File is not a binary
            return Err(Error::WrongElfFile);
        }

        let elf_machine = data_source.read_u16_le(0x12)?;
        if elf_machine != Self::EM_ARM {
            // File is not a ARM
            return Err(Error::WrongElfFile);
        }

        let elf_version = data_source.read_u32_le(0x14)?;
        if elf_version != 1 {
            // File is not a ELF
            return Err(Error::WrongElfFile);
        }

        let e_entry = data_source.read_u32_le(0x18)?;
        let e_phoff = data_source.read_u32_le(0x1C)?;
        let e_shoff = data_source.read_u32_le(0x20)?;
        let e_phentsize = data_source.read_u16_le(0x2A)?;

        if e_phentsize != ProgramHeader::SIZE_IN_BYTES {
            return Err(Error::WrongElfFile);
        }

        let e_phnum = data_source.read_u16_le(0x2C)?;
        let e_shentsize = data_source.read_u16_le(0x2E)?;

        if e_shentsize != SectionHeader::SIZE_IN_BYTES {
            return Err(Error::WrongElfFile);
        }

        let e_shnum = data_source.read_u16_le(0x30)?;

        let e_shstrndx = data_source.read_u16_le(0x32)?;

        let loader = Loader {
            data_source,
            e_entry,
            e_phoff,
            e_shoff,
            e_phnum,
            e_shnum,
            e_shstrndx,
        };
        Ok(loader)
    }

    /// Create a section header iterator.
    pub fn iter_section_headers(&self) -> IterSectionHeaders<DS> {
        IterSectionHeaders {
            parent: self,
            next_section: 0,
        }
    }

    /// Create a program header iterator.
    pub fn iter_program_headers(&self) -> IterProgramHeaders<DS> {
        IterProgramHeaders {
            parent: self,
            next_program_header: 0,
        }
    }

    /// The memory address of the entry point
    pub fn e_entry(&self) -> u32 {
        self.e_entry
    }

    /// The offset of the program header table
    pub fn e_phoff(&self) -> u32 {
        self.e_phoff
    }

    /// The offset of the section header table
    pub fn e_shoff(&self) -> u32 {
        self.e_shoff
    }

    /// The number of program header entries
    pub fn e_phnum(&self) -> u16 {
        self.e_phnum
    }

    /// The number of section header entries
    pub fn e_shnum(&self) -> u16 {
        self.e_shnum
    }

    /// Return the start offset for valid segments.
    ///
    /// Any segment with a `p_offset` less than this probably isn't valid.
    pub fn segment_start_offset(&self) -> u32 {
        self.e_phoff() + u32::from(self.e_phnum()) * u32::from(ProgramHeader::SIZE_IN_BYTES)
    }
}

/// Allows you to iterate through the section headers.
///
/// Created with `loader.iter_section_headers()`.
pub struct IterSectionHeaders<'a, DS> {
    parent: &'a Loader<DS>,
    next_section: u16,
}

impl<'a, DS> Iterator for IterSectionHeaders<'a, DS>
where
    DS: Source,
{
    type Item = Result<SectionHeader, Error<DS::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_section == self.parent.e_shnum {
            return None;
        }

        let current_section = self.next_section;
        self.next_section = self.next_section.wrapping_add(1);

        Some(SectionHeader::new(self.parent, current_section))
    }
}

/// Allows you to iterate through the program headers.
///
/// Created with `loader.iter_program_headers()`.
pub struct IterProgramHeaders<'a, DS> {
    parent: &'a Loader<DS>,
    next_program_header: u16,
}

impl<'a, DS> Iterator for IterProgramHeaders<'a, DS>
where
    DS: Source,
{
    type Item = Result<ProgramHeader, Error<DS::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_program_header == self.parent.e_phnum {
            return None;
        }

        let current_program_header = self.next_program_header;
        self.next_program_header = self.next_program_header.wrapping_add(1);

        Some(ProgramHeader::new(self.parent, current_program_header))
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
