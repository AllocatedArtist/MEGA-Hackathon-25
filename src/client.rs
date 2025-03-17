use notan::draw::{Draw, DrawImages};
use notan::graphics::Texture;

#[derive(Clone)]
pub enum ClassType {
    FIGHTER,
    CLERIC,
    MAGE,
}

#[derive(Clone)]
pub enum Prioritization {
    Food,
    Armor,
    Weapon,
}

#[derive(Clone)]
pub struct Client {
    class_type: ClassType,
    price_factor: f32,   //demand goes down the higher this is
    quality_factor: f32, //demand goes up the higher this is
    prioritization: Prioritization,
    income: i32,
    exit: bool,
    complete: bool,
    position: (f32, f32),
    purchase_complete: bool,
    reached_center: bool,
}

impl Client {
    pub fn new(
        class_type: ClassType,
        price_factor: f32,
        quality_factor: f32,
        prioritization: Prioritization,
        income: i32,
    ) -> Self {
        Self {
            reached_center: false,
            class_type,
            quality_factor,
            price_factor,
            complete: false,
            exit: false,
            position: (-200.0, -40.0),
            income,
            prioritization,
            purchase_complete: false,
        }
    }

    pub fn x(&self) -> f32 {
        self.position.0
    }

    pub fn y(&self) -> f32 {
        self.position.1
    }

    pub fn set_x(&mut self, val: f32) {
        self.position.0 = val;
    }

    pub fn set_y(&mut self, val: f32) {
        self.position.1 = val;
    }

    pub fn want(&self) -> String {
        let mage = ["Mage Armor", "Mage Weapons"];
        let fighter = ["Fighter Armor", "Fighter Weapons"];
        let cleric = ["Cleric Armor", "Cleric Weapons"];

        let index: Option<usize> = match self.prioritization {
            Prioritization::Food => None,
            Prioritization::Armor => Some(0),
            Prioritization::Weapon => Some(1),
        };

        if let Some(index) = index {
            match self.class_type {
                ClassType::FIGHTER => fighter[index].to_string(),
                ClassType::CLERIC => cleric[index].to_string(),
                ClassType::MAGE => mage[index].to_string(),
            }
        } else {
            String::from("Food")
        }
    }

    pub fn draw(&mut self, texture: &Texture, gfx: &mut Draw) {
        gfx.image(texture)
            .size(315.0, 315.0)
            .position(self.position.0, self.position.1);

        if !self.exit {
            if self.position.0 >= 200.0 {
                self.position.0 = 200.0;
                self.reached_center = true;
            } else {
                self.position.0 += 5.0;
            }
        } else {
            if self.position.0 >= 800.0 {
                self.position.0 = 800.0;
                self.complete = true;
            } else {
                self.position.0 += 5.0;
            }
        }
    }

    pub fn center(&self) -> bool {
        self.reached_center
    }

    pub fn get_class_type(&self) -> &ClassType {
        &self.class_type
    }

    pub fn get_class_type_name(&self) -> &str {
        match self.class_type {
            ClassType::FIGHTER => "Fighter",
            ClassType::MAGE => "Mage",
            ClassType::CLERIC => "Cleric",
        }
    }

    pub fn price_factor(&self) -> f32 {
        self.price_factor
    }

    pub fn quality_factor(&self) -> f32 {
        self.quality_factor
    }

    pub fn set_price_factor(&mut self, value: f32) {
        self.price_factor = value;
    }

    pub fn set_quality_factor(&mut self, value: f32) {
        self.quality_factor = value;
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }
    pub fn complete(&mut self) {
        self.exit = true;
    }

    pub fn exit(&self) -> bool {
        self.exit
    }

    pub fn income(&self) -> i32 {
        self.income
    }

    pub fn complete_purchase(&mut self) {
        self.purchase_complete = true;
    }

    pub fn is_purchase_complete(&self) -> bool {
        self.purchase_complete
    }
}
