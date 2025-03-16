mod business;
mod core;

use notan::draw::DrawConfig;
use notan::prelude::*;
use notan_egui::EguiConfig;
use wasm_bindgen::prelude::*;

#[notan_main]
#[wasm_bindgen(start)]
fn main() -> Result<(), String> {
    notan::init_with(core::Core::new)
        .add_config(WindowConfig {
            position: Some((0, 0)),
            fullscreen: false,
            width: 800,
            height: 480,
            ..Default::default()
        })
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .add_config(notan::log::LogConfig::debug())
        .update(core::Core::update)
        .draw(core::Core::draw)
        .build()
}
