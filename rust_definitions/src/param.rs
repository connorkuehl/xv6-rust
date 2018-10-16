pub const NPROC: u32 = 64; // maximum number of processes
pub const KSTACKSIZE: u32 = 4096; // size of per-process kernel stack
pub const NCPU: u32 = 8; // maximum number of CPUs
pub const NOFILE: usize = 16; // open files per process
pub const NFILE: u32 = 100; // open files per system
pub const NINODE: u32 = 50; // maximum number of active i-nodes
pub const NDEV: u32 = 10; // maximum major device number
pub const ROOTDEV: u32 = 1; // device number of file system root disk
pub const MAXARG: u32 = 32; // max exec arguments
pub const MAXOPBLOCKS: u32 = 10; // max # of blocks any FS op writes
pub const LOGSIZE: u32 = MAXOPBLOCKS * 3; // max data blocks in on-disk log
pub const NBUF: u32 = MAXOPBLOCKS * 3; // size of disk block cache
#[cfg(feature = "PDX_XV6")]
pub const FSSIZE: u32 = 2000; // size of file system in blocks
#[cfg(not(feature = "PDX_XV6"))]
pub const FSSIZE: u32 = 1000; // size of file system in blocks

#[test]
fn test_param() {
    assert_eq!(NPROC, 64);
    assert_eq!(KSTACKSIZE, 4096);
    assert_eq!(NCPU, 8);
    assert_eq!(NOFILE, 16);
    assert_eq!(NFILE, 100);
    assert_eq!(NINODE, 50);
    assert_eq!(NDEV, 10);
    assert_eq!(ROOTDEV, 1);
    assert_eq!(MAXARG, 32);
    assert_eq!(MAXOPBLOCKS, 10);
    assert_eq!(LOGSIZE, MAXOPBLOCKS * 3);
    assert_eq!(NBUF, MAXOPBLOCKS * 3);
    #[cfg(feature = "PDX_XV6")]
    assert_eq!(FSSIZE, 2000);
    #[cfg(not(feature = "PDX_XV6"))]
    assert_eq!(FSSIZE, 1000);
}
