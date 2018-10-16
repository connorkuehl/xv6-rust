#![feature(const_fn)]
#![allow(dead_code)]
#![no_std]

//! This crate is used to provide a identical view of constants and structs
//! that are shared between different pieces of xv6: the kernel, userprograms,
//! and utilities.

pub mod pdx;

pub mod param;

pub mod fs;

pub mod stat;

pub mod types;
