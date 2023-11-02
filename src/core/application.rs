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

    fn reset(&mut self) {
        self.cpu.reset_memory();
        self.cpu.reset_registers();
        self.address = vec![(String::new(), String::new()); 0xFFFF];
    }

    fn evaluate(&mut self) {
        let instructions = parser::parse_instructions(&self.source);
        let instructions_count = instructions.iter().len();
        for (address, value) in self.address.iter() {
            if let (Ok(address), Ok(value)) = (
                u16::from_str_radix(address, 16),
                u8::from_str_radix(value, 16),
            ) {
                self.cpu.write_memory(address as usize, value);
            }
        }

        loop {
            match self.cpu.eval() {
                Some(pc) => {
                    if pc as usize >= instructions_count {
                        break;
                    }
                }
                None => break,
            }
        }
        let non_zero_entries: Vec<(usize, &u8)> = self
            .cpu
            .memory
            .iter()
            .enumerate()
            .filter(|(_, &value)| value != 0)
            .collect();

        for (index, (address, value)) in non_zero_entries.iter().enumerate() {
            self.address[index].0 = format!("{:04X}", address);
            self.address[index].1 = format!("{:02X}", value);
        }
        self.cpu.print_memory();
    }

    fn assemble(&mut self) {
        self.reset();
        let instructions = parser::parse_instructions(&self.source);

        for (i, instruction) in instructions.iter().enumerate() {
            self.cpu.write_memory(i, *instruction);
            self.address[i].0 = format!("{:04X}", i);
            self.address[i].1 = format!("{:02X}", instruction);
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
            if ui.button("Assemble").clicked() {
                self.assemble();
            };

            if ui.button("Run").clicked() {
                self.evaluate();
            };
        });
        render_registers(&ctx, &self);
        render_memory(&ctx, self);

        ctx.request_repaint_after(Self::repaint_max_timeout());
    }
}
