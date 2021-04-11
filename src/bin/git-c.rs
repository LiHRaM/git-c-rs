// Copyright (c) 2020 Hilmar Gústafsson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use git_c::cli::{GitConfig, GitExec};

#[doc(hidden)]
pub(crate) type Error = Box<dyn std::error::Error>;

#[doc(hidden)]
pub(crate) type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    GitExec::execute(GitConfig::get_args())?;
    Ok(())
}
