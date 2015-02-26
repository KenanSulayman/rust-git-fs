// Copyright (C) 2014  Josh Stone
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # git-fs: command-line tool to mount Git objects
//!
//! Usage: git-fs [GIT_DIR [MOUNTPOINT]]
//!
//! - GIT_DIR: The directory of a git repository.  A bare git directory is fine,
//! or if given as a working directory, it will automatically use the .git/
//! directory within.  Defaults to the current directory.
//!
//! - MOUNTPOINT: The target to mount the filesystem.  Defaults to GIT_DIR/fs.

#![feature(env)]
#![feature(os)]
#![feature(path)]
#![feature(std_misc)]

extern crate gitfs;

use std::ffi::{AsOsStr, OsStr, OsString};
use std::path::AsPath;

fn main() {
    let args: Vec<OsString> = std::env::args_os().collect();

    // If unspecified, source defaults to the current directory
    let source: &OsStr = if args.len() > 1 { &args[1] } else { ".".as_os_str() };

    match gitfs::GitFS::new(source.as_path()) {
        Ok(fs) => {
            // If unspecified, the target default to GIT_DIR/fs
            let target = if args.len() > 2 {
                args[2].as_path().to_path_buf()
            } else { 
                fs.git_dir().join("fs")
            };

            fs.mount(&target);
        },
        Err(e) => panic!("{}", e),
    };
}
