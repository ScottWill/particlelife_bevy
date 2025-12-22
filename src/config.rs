use bevy::prelude::*;
use crate::{
    physics::forces::{ForceMatrixType, RandomForceMatrix},
    providers::positioners::PositionerType
};

// defaults
const BODIES: usize = 20_000;
const COLORS: usize = 7;

pub trait FormatableValue<T> {
    fn get_str(&self) -> &str;
    fn get_value(&self) -> T;
    fn set_value(&mut self, value: T);
}

#[derive(Clone, Debug)]
pub struct FormattedNumber {
    value: usize,
    string: String,
}

impl FormatableValue<usize> for FormattedNumber {
    fn set_value(&mut self, value: usize) {
        self.value = value;
        self.string = value.to_string();
    }
    fn get_value(&self) -> usize {
        self.value
    }
    fn get_str(&self) -> &str {
        &self.string
    }
}

impl FormattedNumber {
    fn new(value: usize) -> Self {
        Self {
            value,
            string: value.to_string()
        }
    }
}

#[derive(Clone, Debug, Resource)]
pub struct ConfigState {
    pub bodies_count: FormattedNumber,
    pub colors_count: FormattedNumber,
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
            bodies_count: FormattedNumber::new(BODIES),
            colors_count: FormattedNumber::new(COLORS),
            force_matrix_option: ForceMatrixType::Random(RandomForceMatrix),
            half_side: 0.0,
            body_mesh: None,
            panel_width: 200.0,
            position_option: PositionerType::Uniform,
            reset_bodies: true,
        }
    }
}
