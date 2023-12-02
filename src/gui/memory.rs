use eframe::egui::{Context, Direction, Layout, SidePanel};
use egui_extras::{Column, TableBuilder};

use super::application::Application;
pub fn render_memory(ctx: &Context, state: &mut Application) {
    SidePanel::right("right memory").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Memory Editor");
        });
        ui.separator();
        ui.vertical_centered_justified(|ui| {
            let table = TableBuilder::new(ui)
                .striped(false)
                .resizable(false)
                .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
                .column(Column::auto())
                .column(Column::initial(100.0).range(40.0..=300.0))
                .column(Column::initial(100.0).at_least(40.0).clip(true))
                .column(Column::remainder())
                .min_scrolled_height(0.0);

            table
                .header(20., |mut header| {
                    header.col(|ui| {
                        ui.label("Address");
                    });
                    header.col(|ui| {
                        ui.label("Value");
                    });
                })
                .body(|body| {
                    body.rows(20., state.address.len(), |index, mut row| {
                        row.col(|ui| {
                            ui.text_edit_singleline(&mut state.address[index].0);
                        });
                        row.col(|ui| {
                            ui.text_edit_singleline(&mut state.address[index].1);
                        });
                    });
                });
        });
    });
}
