use notan::draw::DrawImages;
use notan::draw::*;
use notan::prelude::*;
use notan_egui::*;

use crate::business::Business;

#[derive(AppState)]
pub struct Core {
    loaded_assets: AssetList,
    background_texture: Option<Texture>,
    foreground_texture: Option<Texture>,
    background_characters: Option<Texture>,
    time: f32,

    business: Business,
}

impl Core {
    pub fn new(assets: &mut Assets) -> Self {
        Self {
            loaded_assets: assets
                .load_list(&[
                    "../assets/front.png",
                    "../assets/back.png",
                    "../assets/bg_characters.png",
                ])
                .unwrap(),
            background_texture: None,
            foreground_texture: None,
            background_characters: None,
            time: 0.0,

            business: Business::new(),
        }
    }

    pub fn update(app: &mut App, state: &mut Core) {
        if !state.loaded_assets.is_loaded() {
            return;
        }

        let mut load_texture = |path| {
            return match state.loaded_assets.take::<Texture>(path) {
                Ok(asset) => Some(asset.try_unwrap().unwrap()),
                Err(err) => {
                    notan::log::error!("Failed to load image! {}", err);
                    None
                }
            };
        };

        let bg_empty = state.background_texture.is_none();
        let fg_empty = state.foreground_texture.is_none();
        let bg_char_empty = state.background_characters.is_none();

        if bg_empty {
            state.background_texture = load_texture("../assets/back.png");
        }
        if fg_empty {
            state.foreground_texture = load_texture("../assets/front.png");
        }
        if bg_char_empty {
            state.background_characters = load_texture("../assets/bg_characters.png");
        }

        state.time += app.timer.delta_f32();
    }

    pub fn draw(graphics: &mut Graphics, plugins: &mut Plugins, state: &mut Core) {
        let mut bg = graphics.create_draw();
        bg.clear(Color::from_rgb(0.4, 0.6, 0.3));

        if state.loaded_assets.progress() != 1.0 {
            bg.clear(Color::from_rgb(0.56, 0.4, 0.3));
            graphics.render(&bg);
            return;
        }

        if let Some(bg_image) = &state.background_texture {
            bg.image(bg_image).size(800.0, 480.0);
        }

        if let Some(bg_characters) = &state.background_characters {
            let x_scale = 800.0 / 1920.0;
            let y_scale = 480.0 / 1080.0;

            //TODO: FIX ANIMATION CODE
            bg.animation_grid(bg_characters, 10, 8)
                .time(state.time / 6.0)
                .size(666.0 * x_scale, 418.0 * y_scale)
                .position(50.0, 50.0);
        }

        graphics.render(&bg);

        let mut fg = graphics.create_draw();
        if let Some(fg_image) = &state.foreground_texture {
            fg.image(fg_image).size(800.0, 480.0);
        }

        graphics.render(&fg);

        let ui_output = plugins.egui(|ctx| {
            Window::new("Price Levels")
                .resizable(false)
                .show(ctx, |ui| {
                    state.business.price_label(ui, "Food");

                    ui.separator();

                    state.business.price_label(ui, "Fighter Armor");
                    state.business.price_label(ui, "Fighter Weapons");

                    ui.separator();

                    state.business.price_label(ui, "Cleric Armor");
                    state.business.price_label(ui, "Cleric Weapons");

                    ui.separator();

                    state.business.price_label(ui, "Mage Armor");
                    state.business.price_label(ui, "Mage Weapons");
                });

            Window::new("Fund Allocation")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(format!("Available Funds: ${}", state.business.funds()));

                    state.business.allocation_label(ui, "Food");

                    ui.separator();

                    state.business.allocation_label(ui, "Fighter Armor");
                    state.business.allocation_label(ui, "Fighter Weapons");

                    ui.separator();

                    state.business.allocation_label(ui, "Cleric Armor");
                    state.business.allocation_label(ui, "Cleric Weapons");

                    ui.separator();

                    state.business.allocation_label(ui, "Mage Armor");
                    state.business.allocation_label(ui, "Mage Weapons");

                    ui.separator();
                    state
                        .business
                        .allocation_label(ui, "Research & Development");

                    ui.separator();
                    state.business.allocation_label(ui, "Marketing");
                });
        });

        graphics.render(&ui_output);
    }
}
