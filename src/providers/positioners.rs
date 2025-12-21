use glam::DVec2;
use rand::{Rng as _, rng};
use strum::EnumIter;
use std::f64::consts::TAU;

pub fn get_position(pos_type: &PositionerType) -> DVec2 {
    match pos_type {
        PositionerType::BigBang => BigBangPositioner::get_pos(),
        PositionerType::Circle => CirclePositioner::get_pos(),
        PositionerType::Line => LinePositioner::get_pos(),
        PositionerType::SRing => SRingPositioner::get_pos(),
        PositionerType::MRing => MRingPositioner::get_pos(),
        PositionerType::LRing => LRingPositioner::get_pos(),
        PositionerType::Spiral => SpiralPositioner::get_pos(),
        PositionerType::Uniform => UniformPositioner::get_pos(),
        PositionerType::UniformCircle => UniformCirclePositioner::get_pos(),
    }
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum PositionerType {
    BigBang,
    Circle,
    Line,
    SRing,
    MRing,
    LRing,
    Spiral,
    Uniform,
    UniformCircle,
}

pub trait Positioner {
    fn get_pos() -> DVec2;
}

pub struct BigBangPositioner;
impl Positioner for BigBangPositioner {
    fn get_pos() -> DVec2 {
        let mut rng = rng();
        let radius = rng.random::<f64>() * 0.01;
        let theta = rng.random::<f64>() * TAU;
        DVec2 {
            x: theta.cos() * radius + 0.5,
            y: theta.sin() * radius + 0.5
        }
    }
}

pub struct CirclePositioner;
impl Positioner for CirclePositioner {
    fn get_pos() -> DVec2 {
        let mut rng = rng();
        let radius = rng.random::<f64>() * 0.5;
        let theta = rng.random::<f64>() * TAU;
        DVec2 {
            x: theta.cos() * radius + 0.5,
            y: theta.sin() * radius + 0.5
        }
    }
}

// pub struct ExactPositioner(DVec2);
// impl Positioner for ExactPositioner {
//     fn get_pos(&self) -> DVec2 {
//         self.0
//     }
// }

pub struct LinePositioner;
impl Positioner for LinePositioner {
    fn get_pos() -> DVec2 {
        let mut rng = rng();
        DVec2 {
            x: rng.random::<f64>(),
            y: rng.random::<f64>() * 0.125 + 0.5 - 0.0625
        }
    }
}

pub struct SRingPositioner;
impl Positioner for SRingPositioner {
    fn get_pos() -> DVec2 {
        let mut rng = rng();
        let radius = rng.random::<f64>() * 0.1 + 0.125;
        let theta = rng.random::<f64>() * TAU;
        DVec2 {
            x: theta.cos() * radius + 0.5,
            y: theta.sin() * radius + 0.5
        }
    }
}

pub struct MRingPositioner;
impl Positioner for MRingPositioner {
    fn get_pos() -> DVec2 {
        let mut rng = rng();
        let radius = rng.random::<f64>() * 0.1 + 0.25;
        let theta = rng.random::<f64>() * TAU;
        DVec2 {
            x: theta.cos() * radius + 0.5,
            y: theta.sin() * radius + 0.5
        }
    }
}

pub struct LRingPositioner;
impl Positioner for LRingPositioner {
    fn get_pos() -> DVec2 {
        let mut rng = rng();
        let radius = rng.random::<f64>() * 0.1 + 1.0 / 3.0;
        let theta = rng.random::<f64>() * TAU;
        DVec2 {
            x: theta.cos() * radius + 0.5,
            y: theta.sin() * radius + 0.5
        }
    }
}

pub struct SpiralPositioner;
impl Positioner for SpiralPositioner {
    fn get_pos() -> DVec2 {
        // let mut rng = Random::default();
        let mut rng = rng();
        let max_rotations = 2.0;
        let f = rng.random::<f64>();
        let angle = max_rotations * TAU * f;
        let spread = 0.5 * f.min(0.2);
        let radius = (0.9 * f + spread * spread * rng.random::<f64>()) * 0.5;
        DVec2 {
            x: radius * angle.cos() + 0.5,
            y: radius * angle.sin() + 0.5,
        }
    }
}

pub struct UniformPositioner;
impl Positioner for UniformPositioner {
    fn get_pos() -> DVec2 {
        let mut rng = rng();
        DVec2 {
            x: rng.random::<f64>(),
            y: rng.random::<f64>()
        }
    }
}

pub struct UniformCirclePositioner;
impl Positioner for UniformCirclePositioner {
    fn get_pos() -> DVec2 {
        let mut rng = rng();
        let radius = rng.random::<f64>().sqrt() * 0.5;
        let theta = rng.random::<f64>() * TAU;
        DVec2 {
            x: theta.cos() * radius + 0.5,
            y: theta.sin() * radius + 0.5
        }
    }
}
