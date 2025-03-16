
#[derive(Clone)]
pub enum ClassType{
    FIGHTER,
    CLERIC,
    MAGE
}

#[derive(Clone)]
pub struct Client {
    class_type : ClassType,
    demand : i32,
    position : (f32, f32),
}

impl Client {

    pub fn new(class_type: ClassType) -> Self {

        Self {
            class_type,
            demand : 0,
            position : (0.0, 0.0),
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
}