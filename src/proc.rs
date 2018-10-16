use file::{File, Inode};
use rust_definitions::param::NOFILE;
use rust_definitions::types::Pde;

use core::ffi::c_void;

extern "C" {
    pub fn myproc() -> *const Proc;
    pub fn growproc(n: i32) -> i32;
    pub fn kill(pid: i32) -> i32;
    pub fn exit();
    pub fn fork() -> i32;
    pub fn sleep(chan: *const c_void, lk: *const c_void);
    pub fn wait() -> i32;
}

#[repr(C)]
pub struct Context {
    edi: u32,
    esi: u32,
    ebx: u32,
    ebp: u32,
    eip: u32,
}

#[repr(C)]
pub struct Proc {
    pub sz: u32,
    pub pgdir: *const Pde,
    pub kstack: *const u8,
    pub procstate: u32, // Should be enum
    pub pid: u32,
    pub parent: *const Proc,
    pub context: *const Context,
    pub chan: *const c_void,
    pub killed: i32,
    pub ofile: [*const File; NOFILE],
    pub cwd: *const Inode,
    pub name: [u8; 16],
}
