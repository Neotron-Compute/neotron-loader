static ELF_DATA: &[u8] = include_bytes!("../test.elf");

#[test]
fn parse_elf() {
    let loader = neotron_loader::Loader::new(&ELF_DATA[..]).unwrap();
    assert_eq!(0x2000_12a9, loader.e_entry());
    assert_eq!(0x0000_0034, loader.e_phoff());
    assert_eq!(0x0000_757C, loader.e_shoff());
    assert_eq!(6, loader.e_phnum());
    assert_eq!(20, loader.e_shnum());
}

#[test]
fn program_headers() {
    let loader = neotron_loader::Loader::new(&ELF_DATA[..]).unwrap();
    let segments: Result<Vec<neotron_loader::ProgramHeader>, _> =
        loader.iter_program_headers().collect();
    let segments = segments.unwrap();
    assert_eq!(6, segments.len());

    // PHDR off    0x00000034 vaddr 0x20000034 paddr 0x20000034 align 2**2
    //      filesz 0x000000c0 memsz 0x000000c0 flags r--

    assert_eq!(0x0000_00c0, segments[0].p_filesz());
    assert_eq!(0x0000_00c0, segments[0].p_memsz());
    assert_eq!(0x0000_0034, segments[0].p_offset());
    assert_eq!(0x2000_0034, segments[0].p_paddr());
    assert_eq!(neotron_loader::ProgramHeader::PT_PHDR, segments[0].p_type());

    // LOAD off    0x00000000 vaddr 0x20000000 paddr 0x20000000 align 2**16
    //      filesz 0x000000f4 memsz 0x000000f4 flags r--

    assert_eq!(0x0000_00f4, segments[1].p_filesz());
    assert_eq!(0x0000_00f4, segments[1].p_memsz());
    assert_eq!(0x0000_0000, segments[1].p_offset());
    assert_eq!(0x2000_0000, segments[1].p_paddr());
    assert_eq!(neotron_loader::ProgramHeader::PT_LOAD, segments[1].p_type());

    // LOAD off    0x00001000 vaddr 0x20001000 paddr 0x20001000 align 2**16
    //      filesz 0x00000444 memsz 0x00000444 flags r-x

    assert_eq!(0x0000_0444, segments[2].p_filesz());
    assert_eq!(0x0000_0444, segments[2].p_memsz());
    assert_eq!(0x0000_1000, segments[2].p_offset());
    assert_eq!(0x2000_1000, segments[2].p_paddr());
    assert_eq!(neotron_loader::ProgramHeader::PT_LOAD, segments[2].p_type());

    // LOAD off    0x00001444 vaddr 0x20001444 paddr 0x20001444 align 2**16
    //      filesz 0x00000038 memsz 0x00000038 flags r--

    assert_eq!(0x0000_0038, segments[3].p_filesz());
    assert_eq!(0x0000_0038, segments[3].p_memsz());
    assert_eq!(0x0000_1444, segments[3].p_offset());
    assert_eq!(0x2000_1444, segments[3].p_paddr());
    assert_eq!(neotron_loader::ProgramHeader::PT_LOAD, segments[3].p_type());

    // LOAD off    0x0000147c vaddr 0x2000147c paddr 0x2000147c align 2**16
    //      filesz 0x00000000 memsz 0x00000004 flags rw-

    assert_eq!(0x0000_0000, segments[4].p_filesz());
    assert_eq!(0x0000_0004, segments[4].p_memsz());
    assert_eq!(0x0000_147c, segments[4].p_offset());
    assert_eq!(0x2000_147c, segments[4].p_paddr());
    assert_eq!(neotron_loader::ProgramHeader::PT_LOAD, segments[4].p_type());

    // STACK off    0x00000000 vaddr 0x00000000 paddr 0x00000000 align 2**64
    //      filesz 0x00000000 memsz 0x00000000 flags rw-

    assert_eq!(0x0000_0000, segments[5].p_filesz());
    assert_eq!(0x0000_0000, segments[5].p_memsz());
    assert_eq!(0x0000_0000, segments[5].p_offset());
    assert_eq!(0x0000_0000, segments[5].p_paddr());
    assert_eq!(
        neotron_loader::ProgramHeader::PT_GNU_STACK,
        segments[5].p_type()
    );
}

#[test]
fn section_headers() {
    let loader = neotron_loader::Loader::new(&ELF_DATA[..]).unwrap();
    let sections: Result<Vec<neotron_loader::SectionHeader>, _> =
        loader.iter_section_headers().collect();
    let sections = sections.unwrap();
    assert_eq!(20, sections.len());

    //   0                 00000000 00000000
    assert_eq!(0x00000000, sections[0].sh_size());
    assert_eq!(0x00000000, sections[0].sh_addr());
    //   1 .text           00000444 20001000 TEXT
    assert_eq!(0x000_00444, sections[1].sh_size());
    assert_eq!(0x200_01000, sections[1].sh_addr());
    //   2 .rodata         00000038 20001444 DATA
    assert_eq!(0x000_00038, sections[2].sh_size());
    assert_eq!(0x200_01444, sections[2].sh_addr());
    //   3 .data           00000000 2000147c DATA
    assert_eq!(0x000_00000, sections[3].sh_size());
    assert_eq!(0x200_0147c, sections[3].sh_addr());
    //   4 .bss            00000004 2000147c BSS
    assert_eq!(0x000_00004, sections[4].sh_size());
    assert_eq!(0x200_0147c, sections[4].sh_addr());
    //   5 .uninit         00000000 20001480 BSS
    assert_eq!(0x000_00000, sections[5].sh_size());
    assert_eq!(0x200_01480, sections[5].sh_addr());
    //   6 .debug_abbrev   000001fb 00000000 DEBUG
    assert_eq!(0x000_001fb, sections[6].sh_size());
    assert_eq!(0x000_00000, sections[6].sh_addr());
    //   7 .debug_info     0000125f 00000000 DEBUG
    assert_eq!(0x000_0125f, sections[7].sh_size());
    assert_eq!(0x000_00000, sections[7].sh_addr());
    //   8 .debug_aranges  00000148 00000000 DEBUG
    assert_eq!(0x000_00148, sections[8].sh_size());
    assert_eq!(0x000_00000, sections[8].sh_addr());
    //   9 .debug_ranges   000004d8 00000000 DEBUG
    assert_eq!(0x000_004d8, sections[9].sh_size());
    assert_eq!(0x000_00000, sections[9].sh_addr());
    //  10 .debug_str      00001d3f 00000000 DEBUG
    assert_eq!(0x000_01d3f, sections[10].sh_size());
    assert_eq!(0x000_00000, sections[10].sh_addr());
    //  11 .debug_pubnames 0000081c 00000000 DEBUG
    assert_eq!(0x000_0081c, sections[11].sh_size());
    assert_eq!(0x000_00000, sections[11].sh_addr());
    //  12 .debug_pubtypes 00000048 00000000 DEBUG
    assert_eq!(0x000_00048, sections[12].sh_size());
    assert_eq!(0x000_00000, sections[12].sh_addr());
    //  13 .ARM.attributes 00000030 00000000
    assert_eq!(0x000_00030, sections[13].sh_size());
    assert_eq!(0x000_00000, sections[13].sh_addr());
    //  14 .debug_frame    00000510 00000000 DEBUG
    assert_eq!(0x000_00510, sections[14].sh_size());
    assert_eq!(0x000_00000, sections[14].sh_addr());
    //  15 .debug_line     00001322 00000000 DEBUG
    assert_eq!(0x000_01322, sections[15].sh_size());
    assert_eq!(0x000_00000, sections[15].sh_addr());
    //  16 .comment        00000013 00000000
    assert_eq!(0x000_00013, sections[16].sh_size());
    assert_eq!(0x000_00000, sections[16].sh_addr());
    //  17 .symtab         000002e0 00000000
    assert_eq!(0x000_002e0, sections[17].sh_size());
    assert_eq!(0x000_00000, sections[17].sh_addr());
    //  18 .shstrtab       000000d0 00000000
    assert_eq!(0x000_000d0, sections[18].sh_size());
    assert_eq!(0x000_00000, sections[18].sh_addr());
    //  19 .strtab         000004b8 00000000
    assert_eq!(0x000_004b8, sections[19].sh_size());
    assert_eq!(0x000_00000, sections[19].sh_addr());
}
