use std::collections::HashMap;

use notan_egui::*;
use rand::Rng;

#[derive(Clone)]
enum Quality {
    Basic,
    Good,
    Exceptional,
}

#[derive(Clone)]
struct Product {
    quantity: i32,
    quality: Quality,
    production_cost: i32,

    baseline_demand: i32,
    final_demand: i32,
}

impl Product {
    fn base(cost: i32, baseline_demand: i32) -> Self {
        Self {
            quantity: 0,
            quality: Quality::Basic,
            production_cost: cost,
            baseline_demand,
            final_demand: 0,
        }
    }
}

pub struct Business {
    funds: i32,
    prices: HashMap<String, i32>,
    allocation: HashMap<String, i32>,
    quantities: HashMap<String, Product>,
    accumulated_rnd: i32,
    accumulated_marketing: i32,
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
        quantities.insert(String::from("Food"), Product::base(200, 5));

        quantities.insert(String::from("Fighter Armor"), Product::base(300, 20));
        quantities.insert(String::from("Fighter Weapons"), Product::base(275, 20));

        quantities.insert(String::from("Cleric Armor"), Product::base(300, 20));
        quantities.insert(String::from("Cleric Weapons"), Product::base(275, 20));

        quantities.insert(String::from("Mage Armor"), Product::base(300, 20));
        quantities.insert(String::from("Mage Weapons"), Product::base(275, 20));

        Self {
            funds: initial_fund,
            prices,
            allocation,
            quantities,
            accumulated_rnd: 0,
            accumulated_marketing: 0,
        }
    }

    //price_factor: how much demand goes down the higher the price
    //quality_factor: how much demand goes up when quality is higher
    pub fn update_demand(&mut self, price_factor: f32, quality_factor: f32, log: &mut Vec<String>) {
        let rnd_funds = self.allocation["Research & Development"] as f32 * 0.01 * self.funds as f32;
        let marketing_funds = self.allocation["Marketing"] as f32 * 0.01 * self.funds as f32;

        self.funds -= rnd_funds as i32;
        self.funds -= marketing_funds as i32;
        self.funds = self.funds.max(0);

        let mut rnd_demand_adder = 0;
        let mut quality_percentage = 0;
        let mut production_cost_deduction = 0;

        self.accumulated_rnd += rnd_funds as i32;
        self.accumulated_marketing += marketing_funds as i32;

        let base = match self.funds {
            0..10_000 => 20_000,
            10_000..50_000 => 100_000,
            _ => 200_000,
        };

        let rnd_rng = rand::thread_rng().gen_range(0..100);
        'rng: {
            match rnd_rng {
                0..=50 => (),
                51..=80 => {
                    if self.accumulated_rnd > base / 2 {
                        self.accumulated_rnd = 0;
                    } else {
                        break 'rng;
                    }
                    log.push(String::from("Research & Development leads to a slight increase in product quality, raising demand."));
                    rnd_demand_adder += 5;
                    log.push(String::from(
                        "Research also leads to a small cut in production costs.",
                    ));
                    production_cost_deduction = rand::thread_rng().gen_range(1..=5);
                    quality_percentage = 10;
                }
                81..=90 => {
                    if self.accumulated_rnd > base {
                        self.accumulated_rnd = 0;
                    } else {
                        break 'rng;
                    }
                    log.push(String::from("Research & Development leads to a fair increase in product quality, raising demand."));
                    rnd_demand_adder += 10;
                    log.push(String::from(
                        "Research also leads to a decent cut in production costs.",
                    ));
                    production_cost_deduction = rand::thread_rng().gen_range(5..=10);
                    quality_percentage = 30;
                }
                91..100 => {
                    if self.accumulated_rnd > base * 2 {
                        self.accumulated_rnd = 0;
                    } else {
                        break 'rng;
                    }
                    log.push(String::from("Research & Development leads to an exceptional increase in product quality, raising demand greatly."));
                    rnd_demand_adder += 50;
                    log.push(String::from("Research also greatly cuts production costs."));
                    production_cost_deduction = rand::thread_rng().gen_range(20..=30);
                    quality_percentage = 100;
                }
                _ => (),
            };
        }

        let rnd_rng = rand::thread_rng().gen_range(0..100);
        'rng: {
            match rnd_rng {
                0..=50 => (),
                51..=80 => {
                    if self.accumulated_marketing > base / 2 {
                        self.accumulated_marketing = 0;
                    } else {
                        break 'rng;
                    }
                    log.push(String::from(
                        "Marketing leads to a slight increase in consumer demand.",
                    ));
                    rnd_demand_adder += 5;
                }
                81..=90 => {
                    if self.accumulated_marketing > base {
                        self.accumulated_marketing = 0;
                    } else {
                        break 'rng;
                    }
                    log.push(String::from(
                        "Marketing leads to a fair increase in consumer demand.",
                    ));
                    rnd_demand_adder += 20;
                }
                91..100 => {
                    if self.accumulated_marketing > base * 2 {
                        self.accumulated_marketing = 0;
                    } else {
                        break 'rng;
                    }
                    log.push(String::from(
                        "Marketing leads to a boom in consumer demand.",
                    ));
                    rnd_demand_adder += 40;
                }
                _ => (),
            };
        }

        for (name, product) in self.quantities.iter_mut() {
            let quality_value = match product.quality {
                Quality::Basic => 0f32,
                Quality::Good => 5f32,
                Quality::Exceptional => 15f32,
            };

            product.baseline_demand += rnd_demand_adder;
            if rand::thread_rng().gen_range(0..100) < quality_percentage {
                match product.quality.clone() {
                    Quality::Basic => {
                        product.quality = Quality::Good;
                        product.production_cost += 50;
                    }
                    Quality::Good => {
                        product.quality = Quality::Exceptional;
                        product.production_cost += 100;
                    }
                    _ => (),
                }
            }
            product.production_cost -= production_cost_deduction;
            product.production_cost = production_cost_deduction.max(quality_value as i32 + 20);

            let baseline_demand = product.baseline_demand as f32;
            let price = self.prices[name] as f32;
            product.final_demand = (baseline_demand - (price_factor * price)
                + (quality_factor * quality_value)) as i32;
            if product.final_demand < 0 {
                product.final_demand = 0;
            }
        }
    }

    pub fn get_price(&self, name: &str) -> i32 {
        self.prices[name]
    }

    pub fn get_demand(&self, name: &str) -> i32 {
        self.quantities[name].final_demand
    }

    pub fn get_quantity(&self, name: &str) -> i32 {
        self.quantities[name].quantity
    }

    pub fn purchase(&mut self, name: &str, quantity_sold: i32, log: &mut Vec<String>) {
        let mut supply = self.quantities[name].quantity;
        let mut supply_sold = quantity_sold;
        if supply < quantity_sold {
            supply_sold = supply;
        }

        supply -= supply_sold;
        self.funds += supply_sold * self.prices[name];

        log.push(format!("+${} made!", supply_sold * self.prices[name]));

        self.quantities.get_mut(name).unwrap().quantity = supply;
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
        )
        .on_hover_ui(|ui| {
            ui.label("Percentage of current funds allocated.");
            ui.label(
                "Research & Development (R&D) can result in the quality/demand increase of a product.",
            );
            ui.label("Improvements in technology may also reduce production costs.");
            ui.label("Marketing can result in the demand increase of a product.");
        });
    }

    pub fn update_quantities(&mut self) {
        for (k, v) in self.quantities.iter_mut() {
            let fund_percentage = self.allocation.get(k).unwrap().clone();
            let fund_percentage = fund_percentage as f32 * 0.01;
            let fund = (self.funds as f32 * fund_percentage) as i32;

            let unit_production = fund / v.production_cost;

            v.quantity += unit_production;
            self.funds -= fund;
            self.funds = self.funds.max(0);
        }
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

    fn supply_label(&self, ui: &mut Ui, name: &str) {
        let product = self.quantities[name].clone();

        let quality = match product.quality {
            Quality::Basic => "Basic",
            Quality::Good => "Good",
            Quality::Exceptional => "Exceptional",
        };

        let tooltip_format = format!(
            "Quality: {}\nProduction Cost ($): {}",
            quality, product.production_cost
        );

        ui.label(format!(
            "{} (Units): {}",
            name, self.quantities[name].quantity
        ))
        .on_hover_text(tooltip_format);
    }

    pub fn show_supply(&self, ui: &mut Ui) {
        self.supply_label(ui, "Food");

        ui.separator();

        self.supply_label(ui, "Fighter Armor");
        self.supply_label(ui, "Fighter Weapons");

        ui.separator();

        self.supply_label(ui, "Cleric Armor");
        self.supply_label(ui, "Cleric Weapons");

        ui.separator();

        self.supply_label(ui, "Mage Armor");
        self.supply_label(ui, "Mage Weapons");
    }
}
