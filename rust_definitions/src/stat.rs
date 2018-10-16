pub const T_DIR: u16 = 1; // Directory
pub const T_FILE: u16 = 2; // File
pub const T_DEV: u16 = 3; // Device

pub struct Stat {
    f_type: i16, // Type of file
    dev: i32,    // File system's disk device
    ino: u32,    // Inode number
    nlink: i16,  // Number of links to file
    size: u32,   // Size of file in bytes
}
