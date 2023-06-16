//! Test the elf loader

use neotron_loader as ldr;

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    Loader(ldr::Error<ldr::traits::SliceError>),
    MissingArgument,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<ldr::Error<ldr::traits::SliceError>> for Error {
    fn from(value: ldr::Error<ldr::traits::SliceError>) -> Self {
        Error::Loader(value)
    }
}

fn main() -> Result<(), Error> {
    let mut args = std::env::args_os();
    let _example_name = args.next();
    let filename = args.next().ok_or(Error::MissingArgument)?;
    let data = std::fs::read(&filename)?;
    let loader = ldr::Loader::new(&data[..])?;

    println!("Loaded ELF {}", filename.to_string_lossy());
    println!("Entry Point: 0x{:08x}", loader.e_entry());

    let segment_start_addr = loader.segment_start_offset();
    let mut total_ram_used = 0;
    for (idx, ph) in loader.iter_program_headers().enumerate() {
        let ph = ph.expect("PH loaded OK");
        let p_type = match ph.p_type() {
            ldr::ProgramHeader::PT_NULL => "PT_NULL",
            ldr::ProgramHeader::PT_LOAD => "PT_LOAD",
            ldr::ProgramHeader::PT_DYNAMIC => "PT_DYNAMIC",
            ldr::ProgramHeader::PT_INTERP => "PT_INTERP",
            ldr::ProgramHeader::PT_NOTE => "PT_NOTE",
            ldr::ProgramHeader::PT_SHLIB => "PT_SHLIB",
            ldr::ProgramHeader::PT_PHDR => "PT_PHDR",
            ldr::ProgramHeader::PT_TLS => "PT_TLS",
            ldr::ProgramHeader::PT_GNU_STACK => "PT_GNU_STACK",
            _ => "PT_???",
        };

        let ignored = if ph.p_offset() >= segment_start_addr {
            "OK"
        } else {
            "Ignored"
        };

        let data_bytes = ph.p_filesz();
        let zero_bytes = ph.p_memsz() - data_bytes;
        let load_addr = ph.p_paddr();

        total_ram_used += ph.p_memsz();

        println!("PH {idx:02}: p_type = {p_type:12}, data_bytes=0x{data_bytes:04x}, zero_bytes=0x{zero_bytes:04x}, load_addr=0x{load_addr:08x} ({ignored})");
    }

    println!("Total RAM used: {total_ram_used} bytes");

    for (idx, sh) in loader.iter_section_headers().enumerate() {
        let sh = sh.expect("SH loaded OK");
        let sh_type = match sh.sh_type() {
            ldr::SectionHeader::SHT_NULL => "SHT_NULL",
            ldr::SectionHeader::SHT_PROGBITS => "SHT_PROGBITS",
            ldr::SectionHeader::SHT_SYMTAB => "SHT_SYMTAB",
            ldr::SectionHeader::SHT_STRTAB => "SHT_STRTAB",
            ldr::SectionHeader::SHT_RELA => "SHT_RELA",
            ldr::SectionHeader::SHT_HASH => "SHT_HASH",
            ldr::SectionHeader::SHT_DYNAMIC => "SHT_DYNAMIC",
            ldr::SectionHeader::SHT_NOTE => "SHT_NOTE",
            ldr::SectionHeader::SHT_NOBITS => "SHT_NOBITS",
            ldr::SectionHeader::SHT_REL => "SHT_REL",
            ldr::SectionHeader::SHT_DYNSYM => "SHT_DYNSYM",
            ldr::SectionHeader::SHT_INIT_ARRAY => "SHT_INIT_ARRAY",
            ldr::SectionHeader::SHT_FINI_ARRAY => "SHT_FINI_ARRAY",
            ldr::SectionHeader::SHT_PREINIT_ARRAY => "SHT_PREINIT_ARRAY",
            ldr::SectionHeader::SHT_GROUP => "SHT_GROUP",
            ldr::SectionHeader::SHT_SYMTAB_SHNDX => "SHT_SYMTAB_SHNDX",
            _ => "???",
        };

        let mut buffer = [0u8; 64];
        let name = sh.sh_name(&loader, &mut buffer).unwrap_or("E_TOO_LONG");
        println!("SH {idx:02}: {sh:08x?} ({sh_type}, {name:?})");
    }

    Ok(())
}
