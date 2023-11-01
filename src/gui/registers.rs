use eframe::egui::{Context, TopBottomPanel};
use egui_extras::{Column, TableBuilder};

use crate::Application;

const HEIGHT: f32 = 400.;

pub fn render_registers(ctx: &Context, state: &Application) {
    TopBottomPanel::bottom("registers").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.set_height(HEIGHT);

            TableBuilder::new(ui)
                .column(Column::auto().resizable(false))
                .column(Column::remainder())
                .header(20., |mut header| {
                    header.col(|ui| {
                        ui.label("Register");
                    });
                    header.col(|ui| {
                        ui.label("Value");
                    });
                })
                .body(|mut body| {
                    body.row(20., |mut row| {
                        row.col(|ui| {
                            ui.label("A");
                        });
                        row.col(|ui| {
                            ui.label(format!("{:02X}", state.cpu.accumulator));
                        });
                    });
                    body.row(20., |mut row| {
                        row.col(|ui| {
                            ui.label("B");
                        });
                        row.col(|ui| {
                            ui.label(format!("{:02X}", state.cpu.b));
                        });
                    });
                    body.row(20., |mut row| {
                        row.col(|ui| {
                            ui.label("C");
                        });
                        row.col(|ui| {
                            ui.label(format!("{:02X}", state.cpu.c));
                        });
                    });
                    body.row(20., |mut row| {
                        row.col(|ui| {
                            ui.label("D");
                        });
                        row.col(|ui| {
                            ui.label(format!("{:02X}", state.cpu.d));
                        });
                    });
                    body.row(20., |mut row| {
                        row.col(|ui| {
                            ui.label("E");
                        });
                        row.col(|ui| {
                            ui.label(format!("{:02X}", state.cpu.e));
                        });
                    });
                    body.row(20., |mut row| {
                        row.col(|ui| {
                            ui.label("H");
                        });
                        row.col(|ui| {
                            ui.label(format!("{:02X}", state.cpu.h));
                        });
                    });
                    body.row(20., |mut row| {
                        row.col(|ui| {
                            ui.label("L");
                        });
                        row.col(|ui| {
                            ui.label(format!("{:02X}", state.cpu.l));
                        });
                    });
                });
        })
    });
}
