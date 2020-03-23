//! A simple git clone wrapper.
//! git-clown simply mimics the topology of the website it clones from.
//!
//! An example:
//! ```
//! $ git-clown https://github.com/lihram/git-clown
//! Cloning https://github.com/lihram/aau into /home/lihram/git/github.com/lihram/aau
//! ```

use url::Url;
use argh::FromArgs;
use std::process::Command;

fn main() {
    let args: Args = argh::from_env();
    let home_dir = dirs::home_dir().expect("Could not find home dir!").to_str().unwrap().to_string();
    let prefix = args.prefix.or_else(|| Some(home_dir + "/git")).unwrap();

    let url = {
        if let Some(url) = args.url {
            url.to_string()
        } else {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            buffer
        }
    };
    let url = &url;
    let into_dir = organize(&prefix, &url);
    println!("Cloning {} into {}", url, into_dir);
    Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(into_dir)
        .output()
        .expect("Calling git failed");
}

#[derive(FromArgs)]
/// Let's parse some git repository urls.
struct Args {
    #[argh(positional)]
    url: Option<String>,

    #[argh(option)]
    /// optional git prefix
    prefix: Option<String>,
}

/// Transform the url into the folder structure.
/// This is used for the target folder of the git clone command.
fn organize(prefix: &str, url: &str) -> String {
    let url = url.trim();
    // Remove trailing .git
    let url = {
        if url.ends_with(".git") {
            &url[..url.len()-4]
        } else {
            url
        }
    };
    if let Ok(url) = Url::parse(url) {
        // HTTPS protocol
        format!("{}/{}{}", prefix, url.host_str().unwrap(), url.path())
    } else {
        // FORGIVE ME: There is no ssh formatting validation
        let domain = url.split(':').collect::<Vec<_>>()[0].split("@").collect::<Vec<_>>()[1];
        let path = {
            if let Some(loc) = url.find(':') {
                &url[loc+1..]
            } else {
                ""
            }
        };
        format!("{}/{}/{}", prefix, domain, path)
    }
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
}