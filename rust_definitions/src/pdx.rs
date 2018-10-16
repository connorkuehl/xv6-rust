/*
 * This file contains types and definitions for Portland State University.
 * The contents are intended to be visible in both user and kernel space.
 */

pub const TRUE: u32 = 1;
pub const FALSE: u32 = 0;
pub const RETURN_SUCCESS: i32 = 0;
pub const RETURN_FAILURE: i32 = -1;

pub const NUL: u32 = 0;
pub const NULL: u32 = NUL;

pub const TPS: u32 = 1000; // ticks-per-second
pub const SCHED_INTERVAL: u32 = (TPS / 100); // see trap.c

pub const NPROC: u32 = 64; // maximum number of processes -- normally in param.h

#[cfg(feature = "CS333_P2")]
pub const DEFAULT_UID: u32 = 0;
#[cfg(feature = "CS333_P2")]
pub const DEFAULT_GID: u32 = 0;

#[cfg(feature = "CS333_P3P4")]
pub const DEFAULT_BUDGET: u32 = (3 * TPS);
#[cfg(feature = "CS333_P3P4")]
pub const DEFAULT_PRIORITY: u32 = 0;
#[cfg(feature = "CS333_P3P4")]
pub const TICKS_TO_PROMOTE: u32 = (20 * TPS);
#[cfg(feature = "CS333_P3P4")]
pub const MAXPRIO: u32 = 7; /* max prio. 0 <= prio <= MAXPRIO */

#[cfg(feature = "CS333_P5")]
pub const DEFAULT_MODE: u32 = 0755;

#[test]
fn test_pdx() {
    assert_eq!(TRUE, 1);
    assert_eq!(FALSE, 0);
    assert_eq!(RETURN_SUCCESS, 0);
    assert_eq!(RETURN_FAILURE, -1);
    assert_eq!(RETURN_FAILURE, -1);
    assert_eq!(NUL, 0);
    assert_eq!(NULL, 0);
    assert_eq!(TPS, 1000);
    assert_eq!(SCHED_INTERVAL, TPS / 100);
    assert_eq!(NPROC, 64);
}

#[test]
#[cfg(feature = "CS333_P2")]
fn test_pdx_cs333_p2() {
    assert_eq!(DEFAULT_UID, 0);
    assert_eq!(DEFAULT_GID, 0);
}

#[test]
#[cfg(feature = "CS333_P3P4")]
fn test_pdx_cs333_p3p4() {
    assert_eq!(DEFAULT_BUDGET, 3 * TPS);
    assert_eq!(DEFAULT_PRIORITY, 0);
    assert_eq!(TICKS_TO_PROMOTE, 20 * TPS);
    assert_eq!(MAXPRIO, 7);
}

#[test]
#[cfg(feature = "CS333_P5")]
fn test_pdx_cs333_p5() {
    assert_eq!(DEFAULT_MODE, 0755);
}
