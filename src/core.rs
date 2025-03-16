use notan::draw::DrawImages;
use notan::draw::*;
use notan::prelude::*;
use notan_egui::*;

use crate::business::Business;
use crate::client::{ClassType, Client};

#[derive(AppState)]
pub struct Core {
    loaded_assets: AssetList,
    background_texture: Option<Texture>,
    foreground_texture: Option<Texture>,
    background_characters: Option<Texture>,

    class_characters: [Option<Texture>; 3],

    window_states: [bool; 4],

    bg_music: AudioSource,
    bg_sound: Option<Sound>,
    volume: f32,

    start_game: bool,

    time: f32,

    production_time: f32,

    business: Business,
    client: Client,
}

impl Core {
    const FIGHTER_TEXTURE: usize = 0;
    const MAGE_TEXTURE: usize = 1;
    const CLERIC_TEXTURE: usize = 2;

    pub fn new(app: &mut App, assets: &mut Assets) -> Self {
        Self {
            loaded_assets: assets
                .load_list(&[
                    "../assets/front.png",
                    "../assets/back.png",
                    "../assets/bg_characters.png",
                    "../assets/Cleric.png",
                    "../assets/fighter1.png",
                    "../assets/magegirl.png",
                ])
                .unwrap(),
            background_texture: None,
            foreground_texture: None,
            background_characters: None,
            time: 0.0,

            volume: 1.0,

            bg_music: app
                .audio
                .create_source(include_bytes!("../assets/5AM Jazz Mumbling.wav"))
                .unwrap(),

            bg_sound: None,

            window_states: [false, false, false, false],

            start_game: false,

            business: Business::new(100_000),
            client: Client::new(ClassType::CLERIC),
            class_characters: [None, None, None],

            production_time: 0.0,
        }
    }

    pub fn update(app: &mut App, state: &mut Core) {
        if !state.loaded_assets.is_loaded() {
            return;
        }

        if app.mouse.left_was_pressed() && !state.start_game {
            state.start_game = true;
        }

        if !state.start_game {
            return;
        }

        //Wait for user interaction before playing audio
        if state.start_game && state.bg_sound.is_none() && state.time > 1.0 {
            state.bg_sound = Some(app.audio.play_sound(&state.bg_music, 1.0, true));
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

        let fighter_empty = state.class_characters[Self::FIGHTER_TEXTURE].is_none();
        let mage_empty = state.class_characters[Self::MAGE_TEXTURE].is_none();
        let cleric_empty = state.class_characters[Self::CLERIC_TEXTURE].is_none();

        if bg_empty {
            state.background_texture = load_texture("../assets/back.png");
        }
        if fg_empty {
            state.foreground_texture = load_texture("../assets/front.png");
        }
        if bg_char_empty {
            state.background_characters = load_texture("../assets/bg_characters.png");
        }

        if fighter_empty {
            state.class_characters[Self::FIGHTER_TEXTURE] = load_texture("../assets/fighter1.png");
        }

        if mage_empty {
            state.class_characters[Self::MAGE_TEXTURE] = load_texture("../assets/magegirl.png");
        }

        if cleric_empty {
            state.class_characters[Self::CLERIC_TEXTURE] = load_texture("../assets/Cleric.png");
        }

        state.time += app.timer.delta_f32();
        if state.time >= 8.0 {
            state.time = 0.0;
        }

        state.production_time += app.timer.delta_f32();

        if state.production_time >= 5.0 {
            state.production_time = 0.0;
            state.business.update_quantities();
        }

        if let Some(sound) = &state.bg_sound {
            app.audio.set_volume(sound, state.volume);
        }
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

        let mut character = graphics.create_draw();

        match state.client.get_class_type() {
            ClassType::FIGHTER => {
                if let Some(fighter) = &state.class_characters[Self::FIGHTER_TEXTURE] {
                    state.client.draw(fighter, &mut character);
                }
            }
            ClassType::MAGE => {
                if let Some(mage) = &state.class_characters[Self::MAGE_TEXTURE] {
                    state.client.draw(mage, &mut character);
                }
            }
            ClassType::CLERIC => {
                if let Some(cleric) = &state.class_characters[Self::CLERIC_TEXTURE] {
                    state.client.draw(cleric, &mut character);
                }
            }
        };

        graphics.render(&character);

        let mut fg = graphics.create_draw();
        if let Some(fg_image) = &state.foreground_texture {
            fg.image(fg_image).size(800.0, 480.0);
        }

        graphics.render(&fg);

        let ui_output = plugins.egui(|ctx| {
            if !state.start_game {
                Window::new("Game")
                    .resizable(false)
                    .movable(false)
                    .pivot(Align2::CENTER_CENTER)
                    .fixed_pos(pos2(400.0, 240.0))
                    .title_bar(false)
                    .show(ctx, |ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label("Click anywhere to start.");
                        });
                    });
                return;
            }

            TopBottomPanel::bottom("bottom")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.columns(4, |uis| {
                        uis[0].toggle_value(&mut state.window_states[0], "Price Levels");
                        uis[1].toggle_value(&mut state.window_states[1], "Fund Allocation");
                        uis[2].toggle_value(&mut state.window_states[2], "Supply");
                        uis[3].toggle_value(&mut state.window_states[3], "Settings");
                    });
                });

            if state.window_states[0] {
                Window::new("Price Levels")
                    .resizable(false)
                    .collapsible(false)
                    .show(ctx, |ui| {
                        state.business.show_prices(ui);
                    });
            }

            if state.window_states[1] {
                Window::new("Fund Allocation")
                    .resizable(false)
                    .collapsible(false)
                    .show(ctx, |ui| {
                        state.business.show_allocation(ui);
                    });
            }

            if state.window_states[2] {
                Window::new("Supply")
                    .resizable(false)
                    .collapsible(false)
                    .show(ctx, |ui| {
                        state.business.show_supply(ui);
                    });
            }

            if state.window_states[3] {
                Window::new("Settings")
                    .resizable(false)
                    .collapsible(false)
                    .show(ctx, |ui| {
                        ui.add(
                            Slider::new(&mut state.volume, 0.0..=1.0)
                                .text("Volume")
                                .show_value(false),
                        )
                        .on_hover_ui(|ui| {
                            ui.label("Our musician stayed up until 5 am to finish this.");
                        });
                    });
            }
        });

        graphics.render(&ui_output);
    }
}
