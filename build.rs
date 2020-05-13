//
// build.rs
// Copyright (C) 2020 gmg137 <gmg137@live.com>
// Distributed under terms of the MIT license.
//

fn main() {
    cc::Build::new()
        .file("minilzo/minilzo.c")
        .warnings(false)
        .extra_warnings(false)
        .compile("minilzo.a");
}
