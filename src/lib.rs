// Copyright (c) 2020 Hilmar Gústafsson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! A simple git clone wrapper.
//! git-c simply mimics the topology of the website it clones from.
//!
//! An example:
//! ```bash
//! $ git-c https://github.com/lihram/git-c-rs.git
//! Cloning https://github.com/lihram/git-c-rs.git into /home/lihram/git/github.com/lihram/git-c-rs
//! ```

mod https;
mod ssh;

use url::Url;

/// Transform the url into the folder structure.
/// This is used for the target folder of the git clone command.
pub fn to_filesystem_path<'a>(base: &str, url: &'a str) -> String {
    let url = trim(url);
    if let Ok(url) = Url::parse(url) {
        https::parse_url(url, base)
    } else {
        ssh::parse_url(url, base)
    }
}

fn trim(url: &str) -> &str {
    let url = url.trim();
    let url = url.strip_suffix(".git").unwrap_or_else(|| &url);
    url
}

#[test]
fn github_https() {
    let url = "https://github.com/lihram/rust-clone-organized.git";
    let expected = "git/github.com/lihram/rust-clone-organized";
    let actual = to_filesystem_path("git", url);

    assert_eq!(actual, expected);
}

#[test]
fn github_ssh() {
    let url = "git@github.com:lihram/rust-clone-organized.git";
    let expected = "git/github.com/lihram/rust-clone-organized";
    let actual = to_filesystem_path("git", url);

    assert_eq!(actual, expected);
}
