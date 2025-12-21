use arboard::Clipboard;
use bevy::prelude::Resource;
use bevy_egui::egui::{self, DragValue, Ui};
use rand::random;
use strum::{IntoEnumIterator, EnumIter};
use crate::config::{ConfigState, FormatableValue};

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum ForceMatrixType {
    Chains(ChainsForceMatrix),
    Random(RandomForceMatrix),
    Snakes(SnakeForceMatrix),
    // Symmetry(SymmetryForceMatrix),
    Zero(ZeroForceMatrix),
    One(IdentForceMatrix),
}

enum ForceShiftType {
    Column,
    Row,
}

#[derive(Clone, Default)]
struct ForceCell {
    // color: Color32,
    data: f64,
    // text: String,
}

impl ForceCell {
    fn new(data: f64) -> Self {
        // let mut cell = ForceCell::default();
        // cell.update(data);
        // cell
        Self { data }
    }

    fn force_cell_ui(&mut self, ui: &mut Ui) {
        // let desired_size = ui.spacing().interact_size.y * egui::vec2(1.0, 1.0);
        // let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
        // ui.label(&self.text);
        ui.add(DragValue::new(&mut self.data).speed(0.01));
    }

    fn abs(&mut self) {
        // self.update(self.data.abs());
        self.data = self.data.abs();
    }

    fn negate(&mut self) {
        // self.update(self.data * -1.0);
        self.data *= -1.0;
    }

    // fn update(&mut self, data: f64) {
    //     self.data = if data == -0.0 {
    //         0.0
    //     } else {
    //         data.clamp(-1.0, 1.0)
    //     };
    //     self.text = format!("{:.2}", self.data);
    // }

}

impl From<f64> for ForceCell {
    fn from(value: f64) -> Self {
        ForceCell::new(value)
    }
}

#[derive(Clone, Resource)]
pub struct ForceMatrix {
    data: Vec<ForceCell>,
    color_count: usize,
    matrix_type: ForceMatrixType,
}

impl ForceMatrix {

    pub fn new(color_count: usize, matrix_type: ForceMatrixType) -> Self {
        assert!(color_count > 0);
        let data = (0..color_count * color_count)
            .into_iter()
            .map(|i| {
                let x = i % color_count;
                let y = i / color_count;
                let f = match matrix_type {
                    ForceMatrixType::Chains(p) => p.force(x, y, color_count),
                    ForceMatrixType::Random(p) => p.force(x, y, color_count),
                    ForceMatrixType::Snakes(p) => p.force(x, y, color_count),
                    ForceMatrixType::Zero(p) => p.force(x, y, color_count),
                    ForceMatrixType::One(p) => p.force(x, y, color_count),
                };
                f.into()
            })
            .collect::<Vec<_>>();
        Self {
            data,
            color_count,
            matrix_type
        }
    }

    fn copy_to_clipboard(&self) {
        let output = self.data
            .chunks_exact(self.color_count)
            .map(|chunk| chunk
                .into_iter()
                .map(|f| f.data.to_string())
                .collect::<Vec<_>>()
                .join(",")
            )
            .collect::<Vec<_>>()
            .join("\n");
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(output).unwrap();
    }

    fn paste_from_clipboard(&mut self) {
        let mut clipboard = Clipboard::new().unwrap();
        if let Ok(contents) = clipboard.get_text() {
            let mut data: Vec<ForceCell> = Vec::with_capacity(self.color_count * self.color_count);
            for line in contents.lines() {
                let parts = line.split(',').collect::<Vec<_>>();
                for part in parts {
                    if let Ok(num) = part.trim().parse::<f64>() {
                        data.push(num.into());
                    } else {
                        break;
                    }
                }
            }
            if data.len() == self.data.len() {
                self.data = data;
            }
        }
    }

    fn data_ix(&self, x: usize, y: usize) -> usize {
        x + y * self.color_count
    }

    fn get_data(&self, x: usize, y: usize) -> Option<&ForceCell> {
        let ix = self.data_ix(x, y);
        self.data.get(ix)
    }

    pub fn get_force(&self, x: usize, y: usize) -> f64 {
        match self.get_data(x, y) {
            Some(force) => force.data,
            None => 0.0,
        }
    }

    fn abs(&mut self) {
        for cell in &mut self.data {
            cell.abs();
        }
    }

    fn negate(&mut self) {
        for cell in &mut self.data {
            cell.negate();
        }
    }

    pub fn expand(&mut self) {
        let new_size = self.color_count + 1;
        self.data = (0..new_size * new_size)
            .into_iter()
            .map(|i| {
                let x = i % new_size;
                let y = i / new_size;
                match self.get_data(x, y) {
                    Some(cell) => cell.clone(),
                    None => {
                        let f = match self.matrix_type {
                            ForceMatrixType::Chains(p) => p.force(x, y, new_size),
                            ForceMatrixType::Random(p) => p.force(x, y, new_size),
                            ForceMatrixType::Snakes(p) => p.force(x, y, new_size),
                            ForceMatrixType::Zero(p) => p.force(x, y, new_size),
                            ForceMatrixType::One(p) => p.force(x, y, new_size),
                        };
                        f.into()
                    },
                }
            })
            .collect::<Vec<_>>();
        self.color_count = new_size;
    }

    pub fn shrink(&mut self) {
        if self.color_count > 1 {
            let new_len = self.data.len() - self.color_count;
            self.color_count -= 1;
            self.data = self.data[0..new_len]
                .chunks_exact(self.color_count + 1)
                .into_iter()
                .flat_map(|chunks|
                    chunks
                        .iter()
                        .take(self.color_count)
                        .map(|cell| cell.clone())
                )
                .collect::<Vec<_>>();
        }
    }

    fn shift_matrix(&mut self, shift_type: ForceShiftType, amount: isize) {
        self.data = self.data
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let (x, y) = match shift_type {
                    ForceShiftType::Column => (
                        (((i % self.color_count) as isize) + amount).rem_euclid(self.color_count as isize) as usize,
                        i / self.color_count
                    ),
                    ForceShiftType::Row => (
                        i % self.color_count,
                        (((i / self.color_count) as isize) + amount).rem_euclid(self.color_count as isize) as usize
                    ),
                };
                ForceCell::new(self.get_force(x, y))
            })
            .collect();
    }

    pub fn force_matrix_ui(&mut self, ui: &mut Ui, config: &mut ConfigState) {
        ui.horizontal(|ui| {
            if ui.button(" < ").clicked() {
                self.shift_matrix(ForceShiftType::Column, 1);
            }
            if ui.button(" > ").clicked() {
                self.shift_matrix(ForceShiftType::Column, -1);
            }
            if ui.button(" ⬆ ").clicked() {
                self.shift_matrix(ForceShiftType::Row, 1);
            }
            if ui.button(" ⬇ ").clicked() {
                self.shift_matrix(ForceShiftType::Row, -1);
            }
            if ui.button(" Abs ").clicked() {
                self.abs();
            }
            if ui.button(" Neg ").clicked() {
                self.negate();
            }
            if ui.button(" Copy ").clicked() {
                self.copy_to_clipboard();
            }
            if ui.button(" Paste ").clicked() {
                self.paste_from_clipboard();
            }
        });
        // todo: force matric color boxes
        egui::ScrollArea::both()
            .max_height(300.0)
            .show(ui, |ui| {
                egui::Grid::new("force_matrix")
                    .spacing([1.0, 1.0])
                    .striped(true)
                    .show(ui, |ui| {
                        for y in 0..self.color_count {
                            let y = y * self.color_count;
                            for x in 0..self.color_count {
                                if let Some(cell) = self.data.get_mut(x + y) {
                                    cell.force_cell_ui(ui);
                                }
                            }
                            ui.end_row();
                        }
                    });
            });

        // forces select
        ui.horizontal(|ui| {
            if ui.button(" Update ").clicked() {
                // *self = config.new_force_matrix();
                let count = config.colors_count.get_value();
                *self = ForceMatrix::new(count, config.force_matrix_option);
            }
            egui::ComboBox::from_label("Matrix")
                .selected_text(format!("{:?}", config.force_matrix_option))
                .show_ui(ui, |ui| {
                    // ui.style_mut().wrap = Some(false);
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                    ui.set_min_width(60.0);
                    for f in ForceMatrixType::iter() {
                        ui.selectable_value(&mut config.force_matrix_option, f, format!("{:?}", f));
                    }
                });
            ui.end_row();
        });

    }

}



trait MatrixProvider {
    fn force(self, x: usize, y: usize, w: usize) -> f64;
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChainsForceMatrix;
impl MatrixProvider for ChainsForceMatrix {
    fn force(self, x: usize, y: usize, w: usize) -> f64 {
        let amt = 1.0;
        match (y, x) {
            (y, x) if y == x => amt,
            (y, x) if y == (x + 1) % w => amt,
            (y, x) if y == (x + w - 1) % w => amt,
            _ => 0.0
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RandomForceMatrix;
impl MatrixProvider for RandomForceMatrix {
    fn force(self, _: usize, _: usize, _: usize) -> f64 {
        random::<f64>() * 2.0 - 1.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SnakeForceMatrix;
impl MatrixProvider for SnakeForceMatrix {
    fn force(self, x: usize, y: usize, w: usize) -> f64 {
        match (y, x) {
            (y, x) if y == x => 1.0,
            (y, x) if y == (x + 1) % w => 0.2,
            _ => 0.0,
        }
    }
}

// #[derive(Clone, Copy, Default, Debug, PartialEq)]
// pub struct SymmetryForceMatrix(pub usize);
// impl MatrixProvider for SymmetryForceMatrix {
//     fn force(self, _x: usize, _y: usize, _w: usize) -> f64 {
//         todo!()
//     }
// }

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct ZeroForceMatrix;
impl MatrixProvider for ZeroForceMatrix {
    fn force(self, _: usize, _: usize, _: usize) -> f64 {
        0.0
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct IdentForceMatrix;
impl MatrixProvider for IdentForceMatrix {
    fn force(self, _: usize, _: usize, _: usize) -> f64 {
        1.0
    }
}
