use std::time::Duration;

use eframe::egui::{CentralPanel, Context, TextEdit};

use crate::gui::{memory::render_memory, registers::render_registers};

use super::{cpu::Cpu, parser};

pub struct Application {
    pub source: String,
    pub address: Vec<(String, String)>,
    pub cpu: Cpu,
}

impl Application {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            address: vec![(String::new(), String::new()); 0xFFFF],
            cpu: Cpu::new(),
        }
    }

    pub fn repaint_max_timeout() -> Duration {
        Duration::from_secs(1)
    }

    fn reset_cpu(&mut self) {
        self.cpu.reset_memory();
        self.cpu.reset_registers();
    }

    fn evaluate(&mut self) {
        self.reset_cpu();
        let instructions = parser::parse_instructions(&self.source);
        for (address, value) in self.address.iter() {
            if let (Ok(address), Ok(value)) = (
                u16::from_str_radix(address, 16),
                u8::from_str_radix(value, 16),
            ) {
                self.cpu.write_memory(address as usize, value);
            }
        }

        self.address = vec![(String::new(), String::new()); 0xFFFF];

        for (i, instruction) in instructions.iter().enumerate() {
            self.cpu.write_memory(i, *instruction);
            self.address[i].0 = format!("{:04X}", i);
            self.address[i].1 = format!("{:02X}", *instruction);
        }

        let memory_count = instructions.iter().len();

        loop {
            match self.cpu.eval() {
                Some(pc) => {
                    if pc as usize >= memory_count {
                        break;
                    }
                }
                None => break,
            }
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add(
                TextEdit::multiline(&mut self.source)
                    .code_editor()
                    .desired_rows(20)
                    .desired_width(500.),
            );
            if ui.button("Run").clicked() {
                self.evaluate();
            };
        });
        render_registers(&ctx, &self);
        render_memory(&ctx, self);

        ctx.request_repaint_after(Self::repaint_max_timeout());
    }
}
