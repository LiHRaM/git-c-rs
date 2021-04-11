#![cfg(test)]

use super::*;

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
