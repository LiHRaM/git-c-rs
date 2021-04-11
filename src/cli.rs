// Copyright (c) 2021 Hilmar Gústafsson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{
    io,
    process::{Command, ExitStatus},
};

pub struct GitExec;

impl GitExec {
    pub fn execute(args: Vec<String>) -> io::Result<ExitStatus> {
        Command::new("git").arg("clone").args(args).spawn()?.wait()
    }
}

const CLONE_FLAGS: &[&str] = &[
    "-l",
    "--local",
    "--no-hardlinks",
    "-s",
    "--shared",
    "--dissociate",
    "-q",
    "--quiet",
    "-v",
    "--verbose",
    "--progress",
    "-n",
    "--no-checkout",
    "--bare",
    "--sparse",
    "--mirror",
    "--single-branch",
    "--no-single-branch",
    "--no-tags",
    "--shallow-submodules",
    "--no-shallow-submodules",
    "--remote-submodules",
    "--no-remote-submodules",
];

const CLONE_ARGS: &[&str] = &[
    "--template",
    "-o",
    "--origin",
    "-b",
    "--branch",
    "-u",
    "--upload-pack",
    "--reference",
    "--reference-if-able",
    "--separate-git-dir",
    "--depth",
    "--recurse-submodules",
    "--filter",
    "--server-option",
    "-c",
    "--config",
    "--shallow-since",
    "--shallow-exclude",
    "--separate-git-dir",
    "-j",
    "--jobs",
];

pub struct GitConfig;

impl GitConfig {
    fn get_base_dir() -> String {
        std::env::var("REPOS_DIR").unwrap_or_else(|_| Self::get_home_dir() + "/git")
    }

    #[track_caller]
    fn get_home_dir() -> String {
        dirs::home_dir()
            .expect("Failed to find home directory.")
            .to_string_lossy()
            .to_string()
    }

    pub fn get_args() -> Vec<String> {
        let mut git_args: Vec<String> = vec![];
        let mut base = None;

        let mut is_arg = false;
        let mut url = None;
        for arg in std::env::args().skip(1) {
            if is_arg {
                git_args.push(arg);
                is_arg = false;
            } else {
                match arg.as_str() {
                    arg if CLONE_FLAGS.contains(&arg) => {
                        git_args.push(arg.into());
                    }
                    arg if CLONE_ARGS.contains(&arg) => {
                        git_args.push(arg.into());
                        is_arg = true;
                    }
                    arg if arg.starts_with('-') => {
                        println!("Argument {} not supported.", arg);
                        std::process::exit(1);
                    }
                    arg if url.is_none() => {
                        url = Some(arg.to_owned());
                        git_args.push(arg.into());
                    }
                    arg if url.is_some() => {
                        base = Some(arg.into());
                    }
                    arg => {
                        println!("Too many arguments - {} is excessive", arg);
                        std::process::exit(1);
                    }
                }
            }
        }

        let url = &url.unwrap_or_else(|| {
            println!("The url argument is missing");
            std::process::exit(1);
        });

        let base = &base.unwrap_or_else(Self::get_base_dir);
        let directory = crate::to_filesystem_path(base, url);
        git_args.push(directory);

        git_args
    }
}
