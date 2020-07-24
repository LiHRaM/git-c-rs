// Copyright (c) 2020 Hilmar Gústafsson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use argh::FromArgs;
use std::process::Command;

mod lib;

fn main() {
    let args: Args = argh::from_env();
    let home_dir = dirs::home_dir()
        .expect("Could not find home dir!")
        .to_str()
        .unwrap()
        .to_string();
    let prefix = args.prefix.or_else(|| Some(home_dir + "/git")).unwrap();

    let url = {
        if let Some(url) = args.url {
            url
        } else {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            buffer
        }
    };
    let into_dir = lib::organize(&prefix, &url.to_lowercase());
    Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(into_dir)
        .spawn()
        .expect("starting git clone failed")
        .wait()
        .expect("git clone was unsuccessful");
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
