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

const FILTER: [&str; 1] = ["git"];

/// Transform the url into the folder structure.
/// This is used for the target folder of the git clone command.
pub fn organize(prefix: &str, url: &str) -> String {
    let url = url.trim();
    // Remove trailing .git
    let url = {
        if url.ends_with(".git") {
            &url[..url.len() - 4]
        } else {
            url
        }
    };
    if let Ok(url) = Url::parse(url) {
        // HTTPS protocol
        let path: String = url
            .path()
            .split('/')
            .filter(|el| !FILTER.contains(el))
            .collect::<Vec<_>>()
            .join("/");
        format!("{}/{}{}", prefix, url.host_str().unwrap(), path)
    } else {
        // FORGIVE ME: There is no ssh formatting validation
        // ["git@github.com", "lihram/git-clown-rs"]
        let ssh_parts = url.split(':').collect::<Vec<_>>();

        // ["git", "github.com"] -> "github.com"
        let domain = ssh_parts[0].split('@').collect::<Vec<_>>()[1];

        let path: String = path_filtered(ssh_parts[1]);

        format!("{}/{}/{}", prefix, domain, path)
    }
}

/// Remove parts of the path which are found in FILTER.
fn path_filtered(path: &str) -> String {
    path.split('/')
        .filter(|el| !FILTER.contains(el))
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn github_https() {
        let url = "https://github.com/lihram/rust-clone-organized.git";
        let expected = "git/github.com/lihram/rust-clone-organized";
        let actual = organize("git", url);

        assert_eq!(actual, expected);
    }

    #[test]
    fn github_ssh() {
        let url = "git@github.com:lihram/rust-clone-organized.git";
        let expected = "git/github.com/lihram/rust-clone-organized";
        let actual = organize("git", url);

        assert_eq!(actual, expected);
    }

    #[test]
    fn filter_git() {
        let path = path_filtered("/hello/git/world");
        assert_eq!("/hello/world", &path);
    }
}
