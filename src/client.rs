use notan::draw::{Draw, DrawImages};
use notan::graphics::Texture;

#[derive(Clone)]
pub enum ClassType {
    FIGHTER,
    CLERIC,
    MAGE,
}

#[derive(Clone)]
pub struct Client {
    class_type: ClassType,
    demand: i32,
    complete: bool,
    position: (f32, f32),
}

impl Client {
    pub fn new(class_type: ClassType) -> Self {
        Self {
            class_type,
            demand: 0,
            complete: false,
            position: (0.0, 0.0),
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

    pub fn draw(&mut self, texture: &Texture, gfx: &mut Draw) {
        gfx.image(texture).size(400.0, 400.0).position(self.position.0, self.position.1);

        if !self.is_complete() {
            if self.position.0 >= 200.0 {
                self.position.0 = 200.0;
            } else {
                self.position.0 += 5.0;
            }
        } else {
            if self.position.0 >= 700.0 {
                self.position.0 = 700.0
            } else {
                self.position.0 += 5.0;
            }
        }
    }

    pub fn get_class_type(&self) -> &ClassType {
        &self.class_type
    }
    pub fn get_demand(&self) -> i32 {
        self.demand
    }
    pub fn set_demand(&mut self, demand: i32) {
        self.demand = demand;
    }
    pub fn is_complete(&self) -> bool {
        self.complete
    }
    pub fn complete(&mut self) {
        self.complete = true;
    }
}
