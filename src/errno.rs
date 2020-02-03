// Copyright 2019 Intel Corporation. All Rights Reserved.
//
// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
//
// Portions Copyright 2017 The Chromium OS Authors. All rights reserved.
//
// SPDX-License-Identifier: (Apache-2.0 AND BSD-3-Clause)

//! Structures, helpers, and type definitions for working with
//! [`errno`](http://man7.org/linux/man-pages/man3/errno.3.html).

use std::io;

/// Wrapper over [`errno`](http://man7.org/linux/man-pages/man3/errno.3.html).
///
/// The error number is an integer number set by system calls and some libc
/// functions in case of error.
pub type Error = io::Error;

/// A specialized [Result](https://doc.rust-lang.org/std/result/enum.Result.html) type
/// for operations that can return `errno`.
///
/// This typedef is generally used to avoid writing out `errno::Error` directly and is
/// otherwise a direct mapping to `Result`.
pub type Result<T> = io::Result<T>;

/// Returns the last `errno` as a [`Result`] that is always an error.
///
/// [`Result`]: type.Result.html
pub fn errno_result<T>() -> Result<T> {
    Err(Error::last_os_error())
}
