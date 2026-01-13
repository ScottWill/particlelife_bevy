use bevy::prelude::Resource;
use glam::DVec2;
use rayon::prelude::*;
use super::{islands::IslandManager, forces::ForceMatrix, bodies::PointBody};

const MAX_DIST: f64 = 0.02; // The maximum distance that a particle can interact with another
const MAX_DIST_RECIP: f64 = 1.0 / MAX_DIST;
const MAX_DIST_SQRD: f64 = MAX_DIST * MAX_DIST;
const MIN_REL_DIST: f64 = 0.3; // The minimum relative distance that two particles can interact with
const MIN_DIST_RECIP: f64 = 1.0 / MIN_REL_DIST;
const INV_MIN_DIST_RECIP: f64 = 1.0 / (1.0 - MIN_REL_DIST);

#[derive(Resource)]
pub struct ParticlePhysics {
    forces: Vec<DVec2>,
    islands: IslandManager,
}

impl Default for ParticlePhysics {
    fn default() -> Self {
        Self {
            forces: Vec::new(),
            islands: IslandManager::new(MAX_DIST),
        }
    }
}

impl ParticlePhysics {

    pub fn get_forces(&mut self, bodies: &[&PointBody], force_matrix: &ForceMatrix) -> &[DVec2] {
        // bucket bodies, (broad phase?)
        self.islands.index_positions(&bodies);
        // aggregate forces
        bodies
            .par_iter()
            .enumerate()
            .map(|(ix, body0)| {
                let mut total_force = DVec2::ZERO;
                for jx in self.islands.get_neighboring_ixs(&body0.position) {
                    if ix == jx { continue }
                    total_force += get_force(body0, &bodies[jx], force_matrix);
                }
                total_force
            })
            .collect_into_vec(&mut self.forces);
        &self.forces
    }

}

fn get_force(body0: &PointBody, body1: &PointBody, forces: &ForceMatrix) -> DVec2 {
    // shortest distance in wrapped toroidal space
    let min_pos = (body1.position - body0.position + 0.5).rem_euclid(DVec2::ONE) - 0.5;
    if min_pos.length_squared() > MAX_DIST_SQRD {
        return DVec2::ZERO;
    }

    let pos = min_pos * MAX_DIST_RECIP;
    let dist = pos.length();

    let force;
    if dist <= MIN_REL_DIST {
        force = dist * MIN_DIST_RECIP - 1.0;
    } else {
        let f = forces.get_force(body0.color, body1.color);
        if f == 0.0 {
            return DVec2::ZERO;
        }
        force = f * (1.0 - (1.0 + MIN_REL_DIST - 2.0 * dist) * INV_MIN_DIST_RECIP);
    };

    force / dist * MAX_DIST * pos
}
