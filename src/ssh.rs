// Copyright (c) 2021 Hilmar Gústafsson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use pom::{
    char_class::multispace,
    parser::{none_of, not_a, sym, Parser},
};

pub(crate) fn parse_url<'a>(url: &'a str, base: &str) -> String {
    let (server, project) = match url.strip_prefix("ssh://") {
        Some(url) => ssh().parse(url.as_bytes()),
        None => scp().parse(url.as_bytes()),
    }
    .expect("Unable to parse ssh url.");
    format!("{}/{}/{}", base, server, project)
}

fn scp<'a>() -> Parser<'a, u8, (String, String)> {
    parts(b':')
}

fn ssh<'a>() -> Parser<'a, u8, (String, String)> {
    parts(b'/')
}

fn parts<'a>(separator: u8) -> Parser<'a, u8, (String, String)> {
    (user() - sym(b'@')).opt() * server() - sym(separator) + project()
}

fn user<'a>() -> Parser<'a, u8, Vec<u8>> {
    none_of(b"@").repeat(1..).name("User")
}

fn server<'a>() -> Parser<'a, u8, String> {
    none_of(b":/")
        .repeat(1..)
        .convert(String::from_utf8)
        .name("Server")
}

fn project<'a>() -> Parser<'a, u8, String> {
    not_a(multispace)
        .repeat(1..)
        .convert(String::from_utf8)
        .name("Project")
}

#[test]
fn parses_scp() {
    let (server, project) = parts(b':').parse(b"user@server:project").unwrap();

    assert_eq!(&server, "server");
    assert_eq!(project, "project");
}

#[test]
fn parses_url() {
    let (server, project) = parts(b'/').parse(b"user@server/project").unwrap();

    assert_eq!(&server, "server");
    assert_eq!(&project, "project");
}

#[test]
fn parses_edge_cases() {
    let urls = [(
        b"User@Server:Project.Api.Dto".to_vec(),
        ("Server", "Project.Api.Dto"),
        b"User@Server:Project-Api-Dto".to_vec(),
        ("Server", "Project-Api-Dto"),
    )];

    for url in &urls {
        let (actual_server, actual_project) = scp().parse(&url.0).unwrap();
        let (expected_server, expected_project) = url.1;
        assert_eq!(expected_server, &actual_server);
        assert_eq!(expected_project, &actual_project);
    }
}
