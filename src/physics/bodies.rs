use bevy::prelude::Component;
use glam::DVec2;

const DRAG_HALFLIFE: f64 = 23.255813953488374;

#[derive(Clone, Copy, Component, Debug, Default)]
pub struct PointBody {
    pub color: usize,
    pub position: DVec2,
    pub velocity: DVec2,
}

impl PointBody {

    pub fn new(color: usize, position: DVec2) -> Self {
        Self {
            color,
            position,
            velocity: DVec2::ZERO,
        }
    }

    #[inline]
    pub fn step(&mut self, force: DVec2, dt: f64) {
        // degrade velocity before adding force
        self.velocity *= 0.5f64.powf(DRAG_HALFLIFE * dt);
        self.velocity += force * dt;
        // update position and wrap in space
        self.position += self.velocity * dt;
        self.position = self.position.rem_euclid(DVec2::ONE);
    }

}
