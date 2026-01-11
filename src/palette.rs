use bevy::prelude::*;
use rand::random;
use crate::{physics::bodies::PointBody, config::ConfigState};

#[derive(Resource)]
pub struct Palette {
    data: Vec<Handle<ColorMaterial>>,
    size: usize,
    white: Handle<ColorMaterial>,
}

impl Palette {
    pub fn new(
        materials: &mut Assets<ColorMaterial>,
        size: usize,
    ) -> Self {
        Self {
            data: (0..size)
                .into_iter()
                .map(|i| {
                    let hue = (i as f32 / size as f32) * 360.0;
                    materials.add(ColorMaterial::from(Color::hsl(hue, 1.0, 0.5)))
                })
                .collect(),
            white: materials.add(ColorMaterial::from(Color::WHITE)),
            size,
        }
    }

    pub fn get(&self, i: usize) -> &Handle<ColorMaterial> {
        &self.data[i]
    }

    pub fn random_ix(&self) -> usize {
        random::<u64>() as usize % self.size
    }

    pub fn white(&self) -> &Handle<ColorMaterial> {
        &self.white
    }
}

pub fn update_palette(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut palette: ResMut<Palette>,
    mut query: Query<(&mut MeshMaterial2d<ColorMaterial>, &mut PointBody)>,
    config: Res<ConfigState>,
) {
    let size = config.colors_count as usize;
    if size != palette.size {
        // re-init palette
        *palette = Palette::new(&mut materials, size);
        // reassign colors
        for (mut hndl, mut body) in query.iter_mut() {
            match body.color.cmp(&size) {
                std::cmp::Ordering::Less => {
                    **hndl = palette.get(body.color).clone();
                },
                _ => {
                    let color = palette.random_ix();
                    body.color = color;
                    **hndl = palette.get(color).clone();
                },
            }
        }
    }
}