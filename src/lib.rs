// Copyright (c) 2020 Hilmar Gústafsson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! A simple git clone wrapper.
//! git-clown simply mimics the topology of the website it clones from.
//!
//! An example:
//! ```bash
//! $ git-clown https://github.com/lihram/git-clown-rs.git
//! Cloning https://github.com/lihram/git-clown-rs.git into /home/lihram/git/github.com/lihram/git-clown-rs
//! ```

use url::Url;

/// Transform the url into the folder structure.
/// This is used for the target folder of the git clone command.
pub fn to_filesystem_path(base: &str, url: &str) -> String {
    let url = trim(url);
    if let Ok(url) = Url::parse(url) {
        parse_https_url(url, base)
    } else {
        parse_ssh_url(url, base)
    }
}

fn trim(url: &str) -> &str {
    let url = url.trim();
    let url = url.strip_suffix(".git").unwrap_or_else(|| &url);
    url
}

fn parse_https_url(url: Url, base: &str) -> String {
    let path: String = url.path().split('/').collect::<Vec<_>>().join("/");
    format!("{}/{}{}", base, url.host_str().unwrap(), path)
}

fn parse_ssh_url(url: &str, base: &str) -> String {
    // FORGIVE ME: There is no ssh formatting validation
    let url = trim_ssh(url);

    // ["git@github.com", "lihram/git-clown-rs"]
    let ssh_parts = url.split(':').collect::<Vec<_>>();

    // ["git", "github.com"] -> "github.com"
    let domain = ssh_parts[0].split('@').collect::<Vec<_>>()[1];

    let path = ssh_parts[1];

    format!("{}/{}/{}", base, domain, path)
}

fn trim_ssh(url: &str) -> &str {
    url.strip_prefix("ssh://").unwrap_or_else(|| &url)
}

#[cfg(test)]
mod tests {
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
}
