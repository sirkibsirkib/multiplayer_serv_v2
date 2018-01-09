use identity::*;

pub struct DiscretePoint2 {
    x: u16,
    y: u16,
}

pub  struct ContinuousPoint2 {
    x: f32,
    y: f32,
}

pub enum Point2 {
    D(DiscretePoint2),
    C(ContinuousPoint2),
}

pub struct Location {

}

impl Location {
    pub fn new_from(primitive: LocationPrimitive) -> Self {
        Location {}
    }
}


pub struct LocationPrimitive {
    
}