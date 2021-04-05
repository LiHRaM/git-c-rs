// Copyright (c) 2021 Hilmar Gústafsson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use url::Url;

pub(crate) fn parse_url(url: Url, base: &str) -> String {
    let path: String = url.path().split('/').collect::<Vec<_>>().join("/");
    format!("{}/{}{}", base, url.host_str().unwrap(), path)
}
