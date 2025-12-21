// use crate::{
//     config::ConfigState,
//     physics::{bodies::PointBody, forces::ForceMatrix}
// };

// pub struct SaveState {
//     bodies: Vec<PointBody>,
//     config: ConfigState,
//     forces: ForceMatrix,
// }

// impl SaveState {
//     pub fn new(bodies: &[&PointBody], config: &ConfigState, forces: &ForceMatrix) -> Self {
//         Self {
//             bodies: bodies.iter().map(|c| **c).collect(),
//             config: config.clone(),
//             forces: forces.clone(),
//         }
//     }
// }

// fn _ui() {

// }