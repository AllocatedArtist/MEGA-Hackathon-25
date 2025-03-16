use std::collections::HashMap;

use notan_egui::*;

enum Quality {
    Basic,
    Good,
    Exceptional,
}

struct Product {
    quantity: i32,
    quality: Quality,
    production_cost: i32,
}

pub struct Business {
    funds: i32,
    prices: HashMap<String, i32>,
    allocation: HashMap<String, i32>,
    quantities: HashMap<String, i32>,
}

impl Business {
    pub fn new(initial_fund: i32) -> Self {
        let mut prices = HashMap::new();
        prices.insert(String::from("Food"), 0);

        prices.insert(String::from("Fighter Armor"), 0);
        prices.insert(String::from("Fighter Weapons"), 0);

        prices.insert(String::from("Cleric Armor"), 0);
        prices.insert(String::from("Cleric Weapons"), 0);

        prices.insert(String::from("Mage Armor"), 0);
        prices.insert(String::from("Mage Weapons"), 0);

        let mut allocation = HashMap::new();
        allocation.insert(String::from("Food"), 0);

        allocation.insert(String::from("Fighter Armor"), 0);
        allocation.insert(String::from("Fighter Weapons"), 0);

        allocation.insert(String::from("Cleric Armor"), 0);
        allocation.insert(String::from("Cleric Weapons"), 0);

        allocation.insert(String::from("Mage Armor"), 0);
        allocation.insert(String::from("Mage Weapons"), 0);

        allocation.insert(String::from("Research & Development"), 0);
        allocation.insert(String::from("Marketing"), 0);

        let mut quantities = HashMap::new();
        quantities.insert(String::from("Food"), 0);

        quantities.insert(String::from("Fighter Armor"), 0);
        quantities.insert(String::from("Fighter Weapons"), 0);

        quantities.insert(String::from("Cleric Armor"), 0);
        quantities.insert(String::from("Cleric Weapons"), 0);

        quantities.insert(String::from("Mage Armor"), 0);
        quantities.insert(String::from("Mage Weapons"), 0);

        Self {
            funds: initial_fund,
            prices,
            allocation,
            quantities,
        }
    }

    fn get_price_mut(&mut self, name: &str) -> &mut i32 {
        self.prices.get_mut(name).unwrap()
    }

    fn get_allocation_percentage_mut(&mut self, name: &str) -> &mut i32 {
        self.allocation.get_mut(name).unwrap()
    }

    pub fn funds(&self) -> i32 {
        self.funds
    }

    pub fn set_funds(&mut self, funds: i32) {
        self.funds = funds;
    }

    pub fn add_funds(&mut self, amount: i32) {
        self.funds += amount;
        self.funds = self.funds.max(0);
    }

    pub fn price_label(&mut self, ui: &mut Ui, name: &str) {
        const MAX_PRICE: i32 = 1_000_000;
        ui.columns(2, |uis| {
            uis[0].label(format!("{}:", name));
            let value = self.get_price_mut(name);
            uis[1]
                .add(DragValue::new(value).clamp_range(0..=MAX_PRICE).prefix("$"))
                .on_hover_ui(|ui| {
                    ui.label("Current price level.\nHigher price levels may lower demand while lower price levels may not make as much profit.\nA price level of $0 means the good is not for sale.");
                });
        });
    }

    pub fn show_prices(&mut self, ui: &mut Ui) {
        self.price_label(ui, "Food");

        ui.separator();

        self.price_label(ui, "Fighter Armor");
        self.price_label(ui, "Fighter Weapons");

        ui.separator();

        self.price_label(ui, "Cleric Armor");
        self.price_label(ui, "Cleric Weapons");

        ui.separator();

        self.price_label(ui, "Mage Armor");
        self.price_label(ui, "Mage Weapons");
    }

    pub fn allocation_label(&mut self, ui: &mut Ui, name: &str) {
        let total_percentage_used = self
            .allocation
            .values()
            .fold(0, |accumulator, &element| accumulator + element);

        let value = self.get_allocation_percentage_mut(name);

        let total_percentage_used = total_percentage_used - *value;

        let max_percentage = (100 - total_percentage_used).max(0);

        ui.add(
            Slider::new(value, 0..=max_percentage)
                .suffix("%")
                .text(name),
        );
    }

    pub fn show_allocation(&mut self, ui: &mut Ui) {
        ui.label(format!("Available Funds: ${}", self.funds()));

        self.allocation_label(ui, "Food");

        ui.separator();

        self.allocation_label(ui, "Fighter Armor");
        self.allocation_label(ui, "Fighter Weapons");

        ui.separator();

        self.allocation_label(ui, "Cleric Armor");
        self.allocation_label(ui, "Cleric Weapons");

        ui.separator();

        self.allocation_label(ui, "Mage Armor");
        self.allocation_label(ui, "Mage Weapons");

        ui.separator();
        self.allocation_label(ui, "Research & Development");

        ui.separator();
        self.allocation_label(ui, "Marketing");
    }
}
