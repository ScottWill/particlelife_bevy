use bevy::prelude::*;
use crate::{
    physics::forces::{ForceMatrixType, RandomForceMatrix},
    providers::positioners::PositionerType
};

// defaults
const BODIES: u16 = 20_000;
const COLORS: u8 = 7;

#[derive(Clone, Debug, Resource)]
pub struct ConfigState {
    pub bodies_count: u16,
    pub colors_count: u8,
    pub force_matrix_option: ForceMatrixType,
    pub half_side: f32,
    pub body_mesh: Option<Handle<Mesh>>,
    pub panel_width: f32,
    pub position_option: PositionerType,
    pub reset_bodies: bool,
}

impl Default for ConfigState {
    fn default() -> Self {
        Self {
            bodies_count: BODIES,
            colors_count: COLORS,
            force_matrix_option: ForceMatrixType::Random(RandomForceMatrix),
            half_side: 0.0,
            body_mesh: None,
            panel_width: 200.0,
            position_option: PositionerType::Uniform,
            reset_bodies: true,
        }
    }
}
