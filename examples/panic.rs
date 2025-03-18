#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use flywheel::{Context, ContextData};

fn main() {
    let _ = Context::new(&ContextData {
        name: "Game",
        version: "0.1.0",
        identifier: "com.example.game",
        creator: "Example Studios",
        copyright: "Copyright (C) 2025 Example Studios",
        url: "game.example.com",
        r#type: "game",
    })
    .unwrap();
    panic!("panic example");
}
