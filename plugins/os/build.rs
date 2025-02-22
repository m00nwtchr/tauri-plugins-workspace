// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

const COMMANDS: &[&str] = &[
    "platform",
    "version",
    "os_type",
    "family",
    "arch",
    "exe_extension",
    "locale",
    "hostname",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
