// Copyright (c) 2020 Hilmar Gústafsson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use argh::FromArgs;
use git_c;
use std::process::Command;

fn main() {
    let args: Args = argh::from_env();
    let base_dir = get_base_dir();
    let prefix = args.prefix.or_else(|| Some(base_dir)).unwrap();

    let url = {
        if let Some(url) = args.url {
            url
        } else {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            buffer
        }
    };
    let into_dir = git_c::to_filesystem_path(&prefix, &url);
    Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(into_dir)
        .spawn()
        .expect("starting git clone failed")
        .wait()
        .expect("git clone was unsuccessful");
}

fn get_base_dir() -> String {
    std::env::var("REPOS_DIR").unwrap_or_else(|_| get_home_dir() + "/git")
}

#[track_caller]
fn get_home_dir() -> String {
    dirs::home_dir()
        .expect("Failed to find home directory.")
        .to_string_lossy()
        .to_string()
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