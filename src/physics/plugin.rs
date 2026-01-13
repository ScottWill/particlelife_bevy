use bevy::prelude::*;



#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PhysicsSystems {
    Prepare,
    StepSimulation,
    Writeback,
}

