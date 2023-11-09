mod core;
mod gui;

use core::{application::Application, token::Token};

use eframe::NativeOptions;

use crate::core::{assembler::assemble, cpu::Cpu, parser::parse};

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--help" => {
                println!("Usage: 8085-emulator [file]");
                return Ok(());
            }
            "--gui" => {
                env_logger::init();
                let native_options = NativeOptions {
                    initial_window_size: Some(eframe::egui::vec2(800., 800.)),
                    ..Default::default()
                };
                eframe::run_native(
                    "8085 Emulator",
                    native_options,
                    Box::new(|_| Box::new(Application::new())),
                )?;
            }
            _ => {
                use logos::Logos;
                let source = std::fs::read_to_string(&args[1]).expect("Couldn;t read file");
                let lexer = Token::lexer(&source);
                let tokens: Vec<_> = lexer.filter_map(|token| token.ok()).collect();
                let instructions = parse(tokens);
                let assembled_instructions = assemble(&instructions);
                let mut cpu = Cpu::new();
                for (index, inst) in assembled_instructions.iter().enumerate() {
                    cpu.write_memory(index, *inst);
                }
                let assembled_count = assembled_instructions.iter().len();
                cpu.print_memory();
                loop {
                    match cpu.eval() {
                        Some(pc) => {
                            if pc as usize >= assembled_count {
                                break;
                            }
                        }
                        None => break,
                    }
                }
                cpu.print_memory();
                cpu.print();
            }
        }
    }
    Ok(())
}
