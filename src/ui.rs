use bevy::{prelude::*, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}};
use bevy_egui::*;
use strum::IntoEnumIterator;
use crate::{
    AppState, ConfigState, ShowUi, config::FormatableValue, physics::forces::ForceMatrix, providers::positioners::PositionerType
};

const LEFT_PANEL: &'static str = "CONFIG";

pub fn ui_system(
    mut config: ResMut<ConfigState>,
    mut force_matrix: ResMut<ForceMatrix>,
    mut gui: EguiContexts,
    mut vis_state: ResMut<NextState<ShowUi>>,
    diagnostics: Res<DiagnosticsStore>,
    key_state: Res<ButtonInput<KeyCode>>,
) {
    let Ok(ctx) = gui.ctx_mut() else { return };

    let inc_amt = match key_state.pressed(KeyCode::ShiftLeft) {
        true => match key_state.pressed(KeyCode::SuperLeft) {
            true => 10000,
            false => 1000,
        },
        false => 100,
    };

    egui::SidePanel::left(LEFT_PANEL)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Hide").clicked() {
                    vis_state.set(ShowUi::No);
                }
                if let Some(value) = diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FPS)
                    .and_then(|fps| fps.smoothed())
                {
                    ui.label(format!("{value:.0} fps"));
                }
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                // load/save
                ui.horizontal(|ui| {
                    if ui.button(" Load ").clicked() {

                    }
                    if ui.button(" Save ").clicked() {

                    }
                });
                // particle count
                ui.label("Particle Count:");
                ui.horizontal(|ui| {
                    if ui.button(" - ").clicked() {
                        let bodies_count = config.bodies_count.get_value();
                        config.bodies_count.set_value(safe_inc(bodies_count, -inc_amt));
                    }
                    if ui.button(" + ").clicked() {
                        let bodies_count = config.bodies_count.get_value();
                        config.bodies_count.set_value(safe_inc(bodies_count, inc_amt));
                    }
                    ui.label(config.bodies_count.get_str());
                });
                // color count
                ui.label("Color Type Count:");
                ui.horizontal(|ui| {
                    if ui.button(" - ").clicked() {
                        let colors_count = config.colors_count.get_value();
                        config.colors_count.set_value(safe_inc(colors_count, -1).max(1));
                        force_matrix.shrink();
                    }
                    if ui.button(" + ").clicked() {
                        let colors_count = config.colors_count.get_value();
                        config.colors_count.set_value(safe_inc(colors_count, 1));
                        force_matrix.expand();
                    }
                    ui.label(config.colors_count.get_str());
                });

                force_matrix.force_matrix_ui(ui, &mut config);

                ui.horizontal(|ui| {
                    if ui.button(" Update ").clicked() {
                        config.reset_bodies = true;
                    }
                    egui::ComboBox::from_label("Positions")
                        .selected_text(format!("{:?}", config.position_option))
                        .show_ui(ui, |ui| {
                            // ui.style_mut().wrap = Some(false);
                            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                            ui.set_min_width(60.0);
                            for f in PositionerType::iter() {
                                ui.selectable_value(&mut config.position_option, f, format!("{:?}", f));
                            }
                        });
                    ui.end_row();
                });
            });

        });

}

fn safe_inc(value: usize, amount: isize) -> usize {
    ((value as isize) + amount).max(0) as usize
}

pub fn toggle_running(
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
) {
    match state.get() {
        AppState::Running => next_state.set(AppState::Paused),
        AppState::Paused => next_state.set(AppState::Running),
    }
}

pub fn toggle_visible(
    mut next_state: ResMut<NextState<ShowUi>>,
    state: Res<State<ShowUi>>,
) {
    match state.get() {
        ShowUi::Yes => next_state.set(ShowUi::No),
        ShowUi::No => next_state.set(ShowUi::Yes),
    }
}

pub fn negate_forces(
    mut force_matrix: ResMut<ForceMatrix>,
) {
    force_matrix.negate();
}