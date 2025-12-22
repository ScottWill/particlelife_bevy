use glam::DVec2;
use super::bodies::PointBody;

const NEIGHBORS: [[isize; 2]; 9] = [
    [-1, -1], [0, -1], [1, -1],
    [-1,  0], [0,  0], [1,  0],
    [-1,  1], [0,  1], [1,  1],
];

#[derive(Default)]
pub struct IslandManager {
    // islands: Vec<Island>,
    islands: Vec<Vec<usize>>, // islands and their per step computed body indicies
    neighbor_ixs: Vec<Vec<usize>>, // pre-cached neighbor indices
    side_f64: f64,
    side: usize,
}

impl IslandManager {
    pub fn new(max_radius: f64) -> Self {
        let side = max_radius.recip().floor() as usize;
        let size = side * side;
        // build and return self
        let mut this = Self {
            islands: vec![vec![]; size],
            neighbor_ixs: Vec::with_capacity(size),
            side_f64: side as f64,
            side,
        };
        this.setup_neighbors();
        this
    }

    // cache the computed indices of each island's group
    fn setup_neighbors(&mut self) {
        let side = self.side as isize;
        self.neighbor_ixs.clear();
        // for each island
        for i in 0..side * side {
            let x = i % side;
            let y = i / side;
            let mut neighborhood = Vec::with_capacity(NEIGHBORS.len());
            // find the index of each surrounding island
            for n in &NEIGHBORS {
                let u = x + n[0];
                let v = y + n[1];
                let j = u.rem_euclid(side) + v.rem_euclid(side) * side;
                neighborhood.push(j as usize);
            }
            self.neighbor_ixs.push(neighborhood);
        }
    }

    // add each body's vec index into the appropriate island
    pub fn index_positions(&mut self, bodies: &[&PointBody]) {
        // clear all the islands w/o reallocating memory
        for island in &mut self.islands {
            island.clear();
        }
        // for each body, add its index to the appropriate island
        // based on its current position
        for (bx, body) in bodies.iter().enumerate() {
            let ix = self.get_local_island_ix(&body.position);
            if let Some(island) = self.islands.get_mut(ix) {
                island.push(bx);
            }
        }
    }

    #[inline]
    pub fn get_neighboring_ixs(&self, pos: &DVec2) -> Vec<usize> {
        let ix = self.get_local_island_ix(pos);
        self.get_local_body_ixs(ix)
    }

    #[inline]
    fn get_local_island_ix(&self, pos: &DVec2) -> usize {
        let x = (pos.x * self.side_f64) as usize;
        let y = (pos.y * self.side_f64) as usize;
        x + y * self.side
    }

    #[inline]
    fn get_local_body_ixs(&self, i: usize) -> Vec<usize> {
        let mut ixs = Vec::new();
        if let Some(nixs) = self.neighbor_ixs.get(i) {
            for nix in nixs {
                ixs.extend_from_slice(&self.islands[*nix]);
            }
        }
        ixs
    }

}
