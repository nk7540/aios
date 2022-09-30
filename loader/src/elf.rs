type Elf64_Addr = usize;
type Elf64_Off = u64;
type Elf64_XWord = u64;
type Elf64_Word = u32;
type Elf64_Half = u16;

#[repr(C)]
pub struct Elf64_Ehdr {
    pub a: [u8; 24],
    pub e_entry: Elf64_Addr,
    pub e_phoff: Elf64_Off,
    pub b: [u8; 14],
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
}

#[repr(C)]
pub struct Elf64_Phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_XWord,
    pub p_memsz: Elf64_XWord,
    pub p_align: Elf64_XWord,
}

// impl Elf64_Ehdr {
//   fn phdrs(&self) -> &[Elf64_Phdr] {
//     let phdr_ptr = self.e_phoff as *const Elf64_Phdr;   

//     for i in 0..self.e_phnum {

//     }
//   }
// }
