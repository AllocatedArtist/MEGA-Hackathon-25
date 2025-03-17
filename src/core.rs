use core::f32;

use notan::draw::DrawImages;
use notan::draw::*;
use notan::prelude::*;
use notan_egui::*;

use crate::business::Business;
use crate::client::{ClassType, Client, Prioritization};

#[derive(AppState)]
pub struct Core {
    loaded_assets: AssetList,
    background_texture: Option<Texture>,
    foreground_texture: Option<Texture>,
    background_characters: Option<Texture>,

    credit_icons: [SizedTexture; 3],
    class_characters: [Option<Texture>; 3],

    window_states: [bool; 5],

    bg_music: AudioSource,
    bg_sound: Option<Sound>,
    volume: f32,

    start_game: bool,

    time: f32,

    production_time: f32,

    business: Business,
    client: Option<Client>,
    log: Vec<String>,

    mage_probability: f32,
    fighter_probability: f32,

    food_probability: f32,
    weapon_probability: f32,

    font: Font
}

impl Core {
    const FIGHTER_TEXTURE: usize = 0;
    const MAGE_TEXTURE: usize = 1;
    const CLERIC_TEXTURE: usize = 2;

    pub fn new(app: &mut App, assets: &mut Assets, graphics: &mut Graphics) -> Self {
        
        let programmer = graphics.create_texture().from_image(include_bytes!("../assets/lead_programmer.png")).with_premultiplied_alpha().build().unwrap();
        let artist = graphics.create_texture().from_image(include_bytes!("../assets/artist.png")).with_premultiplied_alpha().build().unwrap();
        let musician = graphics.create_texture().from_image(include_bytes!("../assets/musician.png")).with_premultiplied_alpha().build().unwrap();
        
        Self {
            font: graphics.create_font(include_bytes!("../assets/font/MajorMonoDisplay-Regular.ttf")).unwrap(),
            loaded_assets: assets
                .load_list(&[
                    "../assets/front.png",
                    "../assets/back.png",
                    "../assets/bg_characters.png",
                    "../assets/Cleric.png",
                    "../assets/fighter1.png",
                    "../assets/magegirl.png",
                    "../assets/lead_programmer.png",
                    "../assets/artist.png",
                    "../assets/musician.png"
                ])
                .unwrap(),
            background_texture: None,
            foreground_texture: None,
            background_characters: None,
            credit_icons: [graphics.egui_register_texture(&programmer), graphics.egui_register_texture(&artist), graphics.egui_register_texture(&musician)],
            
            time: 0.0,

            volume: 1.0,

            bg_music: app
                .audio
                .create_source(include_bytes!("../assets/5AM Jazz Mumbling.wav"))
                .unwrap(),

            bg_sound: None,

            window_states: [false, false, false, false, true],

            start_game: false,

            business: Business::new(5_000),
            client: None,
            class_characters: [None, None, None],

            production_time: 0.0,

            mage_probability: 0.3,
            fighter_probability: 0.3,
            food_probability: 0.3,
            weapon_probability: 0.3,
            log: Vec::new(),
        }
    }

    fn create_character(&mut self) {
        self.log.clear();
        let rng = rand::thread_rng().gen_range(0.0..1.0);

        let class = if rng < self.mage_probability {
            ClassType::MAGE
        } else if rng < self.mage_probability + self.fighter_probability {
            ClassType::FIGHTER
        } else {
            ClassType::CLERIC
        };

        let rng = rand::thread_rng().gen_range(0.0..1.0);

        let priority = if rng < self.food_probability {
            Prioritization::Food
        } else if rng < self.food_probability + self.weapon_probability {
            Prioritization::Weapon
        } else {
            Prioritization::Armor
        };

        let income = rand::thread_rng().gen_range(50..=100);

        self.client = Some(Client::new(class, 0.5, 0.4, priority, income));
    }

    pub fn update(app: &mut App, state: &mut Core) {
        if !state.loaded_assets.is_loaded() {
            return;
        }

        if app.keyboard.was_pressed(KeyCode::Space) && !state.start_game {
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

        // let programmer_empty = state.credit_icons[0].is_none();
        // let artist_empty = state.credit_icons[1].is_none();
        // let musician_empty = state.credit_icons[2].is_none();

        if bg_empty {
            state.background_texture = load_texture("../assets/back.png");
        }
        if fg_empty {
            state.foreground_texture = load_texture("../assets/front.png");
        }
        if bg_char_empty {
            state.background_characters = load_texture("../assets/bg_characters.png");
        }

        // if programmer_empty {
        //     state.credit_icons[0] = load_texture("../assets/lead_programmer.png");
        // }

        // if artist_empty {
        //     state.credit_icons[1] = load_texture("../assets/artist.png");
        // }

        // if musician_empty {
        //     state.credit_icons[2] = load_texture("../assets/lead_programmer.png");
        // }

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

        'client_buy: {
            if let Some(client) = &mut state.client {
                if client.is_complete() {
                    state.create_character();
                } else if !client.is_purchase_complete() && client.center() {
                    let income = client.income();
                    let want = client.want();

                    state.business.update_demand(
                        client.price_factor(),
                        client.quality_factor(),
                        &mut state.log,
                    );

                    let price = state.business.get_price(&want);
                    let demand = state.business.get_demand(&want);

                    let stock = state.business.get_quantity(&want);

                    state.log.push(format!(
                        "Client [{}] appears.",
                        client.get_class_type_name()
                    ));

                    state.log.push(format!("Client desires [{}].", want));

                    if price == 0 {
                        state
                            .log
                            .push(format!("[{}] is not for sale. (Price is set to 0)", want));
                        client.complete_purchase();
                        break 'client_buy;
                    }

                    //If stock is negative, there's a problem.
                    if stock <= 0 {
                        state
                            .log
                            .push(format!("You do not have any [{}] available.", want));
                        client.complete_purchase();
                        break 'client_buy;
                    }

                    let quantity = (demand as f32 / 2.0) * (income as f32 / price as f32);
                    let quantity_multiplier = 1.0 + rand::thread_rng().gen_range(-0.5..=0.75);
                    let final_quantity = quantity * quantity_multiplier;
                    let final_quantity = final_quantity as i32;
                    notan::log::debug!("{demand}");
                    let final_quantity = final_quantity.min(demand);

                    if final_quantity == 0 {
                        state.log.push(
                            "Client does not desire anything from your business.".to_string(),
                        );
                        state.log.push(format!(
                            "Either there is not enough demand for [{}], or the price is too high.",
                            want
                        ));
                        client.complete_purchase();
                        break 'client_buy;
                    }

                    if price > income {
                        state.log.push(format!(
                            "Price of [{}] is more than client is willing to spend.",
                            want
                        ));
                    }

                    state.log.push(format!(
                        "Client seeks to buy {} unit(s) of [{}].",
                        final_quantity, want
                    ));

                    if final_quantity > stock {
                        state.log.push("Client desires more than is currently available. Client will buy what is remaining.".to_string());
                    }

                    let e = f32::consts::E;
                    let purchase_probability =
                        demand as f32 / (demand as f32 + e.powi((price - income).max(-5)));

                    let rng = rand::thread_rng().gen_range(0.0..=1.0);
                    if rng < purchase_probability {
                        state.log.push(format!("Client has made purchase!"));
                        state
                            .business
                            .purchase(&want, final_quantity, &mut state.log);
                    } else {
                        state.log.push(format!("Client did not make purchase."));
                    }

                    client.complete_purchase();
                }
            }
        }

        if state.client.is_none() {
            state.create_character();
        }
    }

    pub fn draw(graphics: &mut Graphics, plugins: &mut Plugins, state: &mut Core) {
        let mut bg = graphics.create_draw();
        bg.clear(Color::from_rgb(0.1, 0.1, 0.1));

        if state.loaded_assets.progress() < 1.0 {
            bg.clear(Color::from_rgb(0.56, 0.4, 0.3));
            bg.text(&state.font, "loading assets").position(400.0, 240.0).h_align_center().size(60.0);
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

        if let Some(client) = &mut state.client {
            match client.get_class_type() {
                ClassType::FIGHTER => {
                    if let Some(fighter) = &state.class_characters[Self::FIGHTER_TEXTURE] {
                        client.draw(fighter, &mut character);
                    }
                }
                ClassType::MAGE => {
                    if let Some(mage) = &state.class_characters[Self::MAGE_TEXTURE] {
                        client.draw(mage, &mut character);
                    }
                }
                ClassType::CLERIC => {
                    if let Some(cleric) = &state.class_characters[Self::CLERIC_TEXTURE] {
                        client.draw(cleric, &mut character);
                    }
                }
            };

            graphics.render(&character);
        }

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
                        ui.vertical_centered(|ui| { ui.heading("Dungeon and Finances"); });
                        ui.separator();
                        ui.columns(3, |uis| {
                            if uis[0].button("Easy Mode").on_hover_ui(|ui| { ui.label("Start off with $200,000. Recommended to learn since I had no time to balance anything."); }).clicked() {
                                state.business.set_funds(200_000);    
                            }
                            if uis[1].button("Medium Mode").on_hover_ui(|ui| { ui.label("Start off with $10,000. You should be better now."); }).clicked() {
                                state.business.set_funds(10_000);    
                            }
                            if uis[2].button("Hard Mode").on_hover_ui(|ui| { ui.label("Start off with $5,000. Is this even feasible?"); }).clicked() {
                                state.business.set_funds(5_000);    
                            }
                        });

                        ui.separator();

                        ui.vertical_centered(|ui| { ui.heading("Credits"); });
                        
                        ui.columns(6, |uis| {
                           uis[0].image(state.credit_icons[0]);
                           uis[1].label("AllocatedArtist").on_hover_ui(|ui| { ui.label("Programmer"); });
                           uis[2].image(state.credit_icons[1]);
                           uis[3].label("MizutamiBazza").on_hover_ui(|ui| { ui.label("Illustrator"); });
                           uis[4].image(state.credit_icons[2]);
                           uis[5].label("Mr.Pigon").on_hover_ui(|ui| { ui.label("Musician");});
                        });

                        ui.separator();

                        ui.columns(2, |uis| {
                           uis[0].vertical_centered(|ui| {
                               ui.label(RichText::new("Fotohh").size(16.0)).on_hover_ui(|ui| { ui.label("Programmer"); }); });
                           uis[1].vertical_centered(|ui| {
                               ui.label(RichText::new("Tategami99").size(16.0)).on_hover_ui(|ui| { ui.label("Programmer--Too busy to participate this weekend but was with us in spirit."); }); });
                        });

                        ui.separator();
                        
                        
                        ui.centered_and_justified(|ui| {
                            ui.label(RichText::new("Press Space to Begin").size(32.0));
                        });
                    });
                return;
            }

            TopBottomPanel::bottom("bottom")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.columns(5, |uis| {
                        uis[0].toggle_value(&mut state.window_states[0], "Price Levels");
                        uis[1].toggle_value(&mut state.window_states[1], "Fund Allocation");
                        uis[2].toggle_value(&mut state.window_states[2], "Supply");
                        uis[3].toggle_value(&mut state.window_states[3], "Settings");
                        uis[4].toggle_value(&mut state.window_states[4], "Client");
                    });
                });

            if state.window_states[4] {
                if let Some(client) = &mut state.client {
                    Window::new("Client")
                        .resizable(false)
                        .collapsible(false)
                        .show(ctx, |ui| {
                            ScrollArea::vertical().show(ui, |ui| {
                                for entry in state.log.iter() {
                                    ui.label(entry);
                                }
                            });
                            if ui.button("Complete Order").clicked() {
                                client.complete();
                            }
                        });
                }
            }

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
