#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMAGE_DOS_HEADER {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMAGE_FILE_HEADER {
    pub Machine: u16,
    pub NumberOfSections: u16,
    pub TimeDateStamp: u32,
    pub PointerToSymbolTable: u32,
    pub NumberOfSymbols: u32,
    pub SizeOfOptionalHeader: u16,
    pub Characteristics: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: u32,
    pub Size: u32,
}

/// This structure comes directly after the [`OptionalHeader`]
pub type DATA_DIRECTORY_TABLE = [IMAGE_DATA_DIRECTORY; 16];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMAGE_OPTIONAL_HEADER_32 {
    /// The unsigned integer that identifies the state of the image file.
    ///
    /// The most common number is 0x10B, which identifies it as a normal executable
    /// file. 0x107 identifies it as a ROM image, and 0x20B identifies it as a
    /// PE32+ executable.
    pub Magic: u16,
    /// The linker major version number.
    pub MajorLinkerVersion: u8,
    /// The linker minor version number.
    pub MinorLinkerVersion: u8,
    /// The size of the code (text) section, or the sum of all code sections if there
    /// are multiple sections.
    pub SizeOfCode: u32,
    /// The size of the initialized data section, or the sum of all such sections if
    /// there are multiple data sections.
    pub SizeOfInitializedData: u32,
    /// The size of the uninitialized data section (BSS), or the sum of all such
    /// sections if there are multiple BSS sections.
    pub SizeOfUninitializedData: u32,
    /// The address of the entry point relative to the image base when the executable
    /// file is loaded into memory.
    ///
    /// Depending on the image type, this field represents:
    /// - Program image: this is the starting address.
    /// - Device drivers: this is the address of the initialization function.
    /// - DLLs: an entry point is optional.
    ///
    /// # Note
    ///
    /// When no entry point is present, this field must be zero.
    pub AddressOfEntryPoint: u32,
    /// The address that is relative to the image base of the beginning-of-code
    /// section when it is loaded into memory.
    pub BaseOfCode: u32,
    /// The address that is relative to the image base of the beginning-of-data
    /// section when it is loaded into memory.
    pub BaseOfData: u32,
    /// The preferred address of the first byte of image when loaded into memory;
    /// must be a multiple of 64 K.
    ///
    /// The default for common formats are:
    /// - DLLs: 0x10000000
    /// - Windows CE EXEs: 0x00010000
    /// - Windows NT/2000/XP/95/98/Me: 0x00400000
    pub ImageBase: u32,
    /// The alignment (in bytes) of sections when they are loaded into memory.
    ///
    /// This value must be greater than or equal to FileAlignment. The default is
    /// the page size for the architecture.
    pub SectionAlignment: u32,
    /// The alignment factor (in bytes) that is used to align the raw data of
    /// sections in the image file.
    ///
    /// This value should be a power of 2 between 512 and 64 K, inclusive. The
    /// default is 512. If the SectionAlignment is less than the architecture's
    /// page size, then FileAlignment must match SectionAlignment.
    pub FileAlignment: u32,
    /// The major version number of the required operating system.
    pub MajorOperatingSystemVersion: u16,
    /// The minor version number of the required operating system.
    pub MinorOperatingSystemVersion: u16,
    /// The major version number of the image.
    pub MajorImageVersion: u16,
    /// The minor version number of the image.
    pub MinorImageVersion: u16,
    /// The major version number of the subsystem.
    pub MajorSubsystemVersion: u16,
    /// The minor version number of the subsystem.
    pub MinorSubsystemVersion: u16,
    /// Reserved, must be zero.
    pub Win32VersionValue: u32,
    /// The size (in bytes) of the image, including all headers, as the image is
    /// loaded in memory.
    ///
    /// It must be a multiple of SectionAlignment.
    pub SizeOfImage: u32,
    /// The combined size of an MS-DOS stub, PE header, and section headers rounded
    /// up to a multiple of FileAlignment.
    pub SizeOfHeaders: u32,
    /// The image file checksum. The algorithm for computing the checksum is
    /// incorporated into IMAGHELP.DLL. The following are checked for validation at
    /// load time: all drivers, any DLL loaded at boot time, and any DLL that is
    /// loaded into a critical Windows process.
    pub CheckSum: u32,
    /// The subsystem that is required to run this image. For more information, see
    /// Windows Subsystem.
    pub Subsystem: u16,
    /// For more information, see DLL Characteristics later in this specification.
    pub DllCharacteristics: u16,
    /// The size of the stack to reserve. Only SizeOfStackCommit is committed; the
    /// rest is made available one page at a time until the reserve size is reached.
    pub SizeOfStackReserve: u32,
    /// The size of the stack to commit.
    pub SizeOfStackCommit: u32,
    /// The size of the local heap space to reserve.
    ///
    /// Only SizeOfHeapCommit is committed; the rest is made available one page at a
    /// time until the reserve size is reached.
    pub SizeOfHeapReserve: u32,
    /// The size of the local heap space to commit.
    pub SizeOfHeapCommit: u32,
    /// Reserved, must be zero.
    pub LoaderFlags: u32,
    /// The number of data-directory entries in the remainder of the optional header.
    /// Each describes a location and size.
    pub NumberOfRvaAndSizes: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMAGE_OPTIONAL_HEADER_64 {
    /// The unsigned integer that identifies the state of the image file.
    ///
    /// The most common number is 0x10B, which identifies it as a normal executable
    /// file. 0x107 identifies it as a ROM image, and 0x20B identifies it as a
    /// PE32+ executable.
    pub Magic: u16,
    /// The linker major version number.
    pub MajorLinkerVersion: u8,
    /// The linker minor version number.
    pub MinorLinkerVersion: u8,
    /// The size of the code (text) section, or the sum of all code sections if there
    /// are multiple sections.
    pub SizeOfCode: u32,
    /// The size of the initialized data section, or the sum of all such sections if
    /// there are multiple data sections.
    pub SizeOfInitializedData: u32,
    /// The size of the uninitialized data section (BSS), or the sum of all such
    /// sections if there are multiple BSS sections.
    pub SizeOfUninitializedData: u32,
    /// The address of the entry point relative to the image base when the executable
    /// file is loaded into memory.
    ///
    /// Depending on the image type, this field represents:
    /// - Program image: this is the starting address.
    /// - Device drivers: this is the address of the initialization function.
    /// - DLLs: an entry point is optional.
    ///
    /// # Note
    ///
    /// When no entry point is present, this field must be zero.
    pub AddressOfEntryPoint: u32,
    /// The address that is relative to the image base of the beginning-of-code
    /// section when it is loaded into memory.
    pub BaseOfCode: u32,
    /// The preferred address of the first byte of image when loaded into memory;
    /// must be a multiple of 64 K.
    ///
    /// The default for common formats are:
    /// - DLLs: 0x10000000
    /// - Windows CE EXEs: 0x00010000
    /// - Windows NT/2000/XP/95/98/Me: 0x00400000
    pub ImageBase: u64,
    /// The alignment (in bytes) of sections when they are loaded into memory.
    ///
    /// This value must be greater than or equal to FileAlignment. The default is
    /// the page size for the architecture.
    pub SectionAlignment: u32,
    /// The alignment factor (in bytes) that is used to align the raw data of
    /// sections in the image file.
    ///
    /// This value should be a power of 2 between 512 and 64 K, inclusive. The
    /// default is 512. If the SectionAlignment is less than the architecture's
    /// page size, then FileAlignment must match SectionAlignment.
    pub FileAlignment: u32,
    /// The major version number of the required operating system.
    pub MajorOperatingSystemVersion: u16,
    /// The minor version number of the required operating system.
    pub MinorOperatingSystemVersion: u16,
    /// The major version number of the image.
    pub MajorImageVersion: u16,
    /// The minor version number of the image.
    pub MinorImageVersion: u16,
    /// The major version number of the subsystem.
    pub MajorSubsystemVersion: u16,
    /// The minor version number of the subsystem.
    pub MinorSubsystemVersion: u16,
    /// Reserved, must be zero.
    pub Win32VersionValue: u32,
    /// The size (in bytes) of the image, including all headers, as the image is
    /// loaded in memory.
    ///
    /// It must be a multiple of SectionAlignment.
    pub SizeOfImage: u32,
    /// The combined size of an MS-DOS stub, PE header, and section headers rounded
    /// up to a multiple of FileAlignment.
    pub SizeOfHeaders: u32,
    /// The image file checksum. The algorithm for computing the checksum is
    /// incorporated into IMAGHELP.DLL. The following are checked for validation at
    /// load time: all drivers, any DLL loaded at boot time, and any DLL that is
    /// loaded into a critical Windows process.
    pub CheckSum: u32,
    /// The subsystem that is required to run this image. For more information, see
    /// Windows Subsystem.
    pub Subsystem: u16,
    /// For more information, see DLL Characteristics later in this specification.
    pub DllCharacteristics: u16,
    /// The size of the stack to reserve. Only SizeOfStackCommit is committed; the
    /// rest is made available one page at a time until the reserve size is reached.
    pub SizeOfStackReserve: u64,
    /// The size of the stack to commit.
    pub SizeOfStackCommit: u64,
    /// The size of the local heap space to reserve.
    ///
    /// Only SizeOfHeapCommit is committed; the rest is made available one page at a
    /// time until the reserve size is reached.
    pub SizeOfHeapReserve: u64,
    /// The size of the local heap space to commit.
    pub SizeOfHeapCommit: u64,
    /// Reserved, must be zero.
    pub LoaderFlags: u32,
    /// The number of data-directory entries in the remainder of the optional header.
    /// Each describes a location and size.
    pub NumberOfRvaAndSizes: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMAGE_NT_HEADERS_32 {
    pub Signature: u32,
    pub FileHeader: IMAGE_FILE_HEADER,
    pub OptionalHeader: IMAGE_OPTIONAL_HEADER_32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMAGE_NT_HEADERS_64 {
    pub Signature: u32,
    pub FileHeader: IMAGE_FILE_HEADER,
    pub OptionalHeader: IMAGE_OPTIONAL_HEADER_64,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_SECTION_HEADER {
    pub Name: [u8; 8],
    /// Represents the [`IMAGE_SECTION_HEADER_u0`] union type.
    pub VirtualSize: u32,
    pub VirtualAddress: u32,
    pub SizeOfRawData: u32,
    pub PointerToRawData: u32,
    pub PointerToRelocations: u32,
    pub PointerToLinenumbers: u32,
    pub NumberOfRelocations: u16,
    pub NumberOfLinenumbers: u16,
    pub Characteristics: u32,
}
