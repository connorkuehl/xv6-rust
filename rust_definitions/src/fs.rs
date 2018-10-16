use core::mem::size_of;

// On-disk file system format.
// Both the kernel and user programs use this header file.

pub const ROOTINO: u32 = 1; // root i-number
pub const BSIZE: u32 = 512; // block size

// Disk layout:
// [ boot block | super block | log | inode blocks |
//                                          free bit map | data blocks]
//
// mkfs computes the super block and builds an initial file system. The
// super block describes the disk layout:
pub struct SuperBlock {
    size: u32,       // Size of file system image (blocks)
    nblocks: u32,    // Number of data blocks
    ninodes: u32,    // Number of inodes.
    nlog: u32,       // Number of log blocks
    logstart: u32,   // Block number of first log block
    inodestart: u32, // Block number of first inode block
    bmapstart: u32,  // Block number of first free map block
}

pub const NDIRECT: usize = 12;
pub const NINDIRECT: usize = BSIZE as usize / size_of::<u32>();
pub const MAXFILE: usize = NDIRECT + NINDIRECT;

// On-disk inode structure
pub struct Dinode {
    f_type: i16,               // File type
    major: i16,                // Major device number (T_DEV only)
    minor: i16,                // Minor device number (T_DEV only)
    nlink: i16,                // Number of links to inode in file system
    size: u32,                 // Size of file (bytes)
    addrs: [u32; NDIRECT + 1], // Data block addresses
}

// Inodes per block.
pub const IPB: u32 = BSIZE / size_of::<Dinode>() as u32;

// Block containing inode i
macro_rules! IBLOCK {
    ($i:ident, $sb:ident) => {
        $i / IPB + $sb.inodestart
    };
}

// Bitmap bits per block
pub const BPB: u32 = BSIZE * 8;

// Block of free map containing bit for block b
macro_rules! BBLOCK {
    ($b:ident, $sb:ident) => {
        $b / BPB + $sb.bmapstart
    };
}

// Directory is a file containing a sequence of dirent structures.
pub const DIRSIZ: u32 = 14;

pub struct Dirent {
    inum: u16,
    name: [u8; DIRSIZ as usize],
}
