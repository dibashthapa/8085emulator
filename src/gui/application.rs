use logos::Logos;
use std::fs;
use std::time::Duration;

use eframe::egui::{include_image, CentralPanel, Context, Image, TextEdit, TextStyle, Vec2};

use crate::gui::{memory::render_memory, registers::render_registers};

use crate::core::{assembler::assemble, cpu::Cpu, parser::parse, token::Token};
use crate::syntax_highlighting;

pub struct Application {
    pub source: String,
    pub address: Vec<(String, String)>,
    pub cpu: Cpu,
    assembled_instructions: Vec<u8>,
}

impl Application {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            address: vec![(String::new(), String::new()); 0xFFFF],
            cpu: Cpu::new(),
            assembled_instructions: vec![],
        }
    }

    pub fn repaint_max_timeout() -> Duration {
        Duration::from_secs(1)
    }

    fn reset(&mut self) {
        self.cpu.reset_memory();
        self.cpu.reset_registers();
        self.cpu.reset_pc();
        self.cpu.reset_flags();
        self.address = vec![(String::new(), String::new()); 0xFFFF];
        self.assembled_instructions = vec![];
    }

    fn evaluate(&mut self) {
        let assembled_count = self.assembled_instructions.iter().len();
        for (address, value) in self.address.iter() {
            if let (Ok(address), Ok(value)) = (
                u16::from_str_radix(address, 16),
                u8::from_str_radix(value, 16),
            ) {
                self.cpu.write_memory(address as usize, value);
            }
        }

        while let Some(pc) = self.cpu.eval() {
            if pc as usize >= assembled_count {
                break;
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
        let lexer = Token::lexer(&self.source);
        let tokens: Vec<_> = lexer.filter_map(|token| token.ok()).collect();
        let instructions = parse(tokens);
        self.assembled_instructions = assemble(&instructions);

        for (i, instruction) in self.assembled_instructions.iter().enumerate() {
            self.address[i].0 = format!("{:04X}", i);
            self.address[i].1 = format!("{:02X}", instruction);
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        use eframe::egui::{menu, TopBottomPanel};

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open File").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.source = fs::read_to_string(path).unwrap();
                        }
                    }

                    if ui.button("Save").clicked() {}
                });
                ui.menu_button("Edit", |ui| {});
                ui.menu_button("Debug", |ui| {});
            });
        });
        // set height and width
        let debug_icon = Image::new(include_image!("../icons/bug-play.svg"))
            .fit_to_exact_size(Vec2::new(40.0, 40.0));

        let play_icon = Image::new(include_image!("../icons/play.svg"))
            .fit_to_exact_size(Vec2::new(40.0, 40.0));

        TopBottomPanel::top("secondary_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                let response = ui.menu_image_button(play_icon, |ui| {});
                if response.response.clicked() {
                    self.assemble();
                    self.evaluate();
                }

                ui.menu_image_button(debug_icon, |ui| {})
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            let mut layouter = |ui: &eframe::egui::Ui, text: &str, wrap_width: f32| {
                let mut layout_job = syntax_highlighting::highlight(&text);
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(layout_job))
            };

            ui.add(
                TextEdit::multiline(&mut self.source)
                    .font(TextStyle::Monospace)
                    .code_editor()
                    .desired_rows(20)
                    .desired_width(500.)
                    .layouter(&mut layouter),
            );
        });
        render_registers(ctx, self);
        render_memory(ctx, self);

        ctx.request_repaint_after(Self::repaint_max_timeout());
    }
}
