use crate::debugger_command::DebuggerCommand;
use crate::dwarf_data::{DwarfData, Error as DwarfError};
use crate::inferior::{Inferior, Status};
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
    debug_data: DwarfData,
    breakpoints: Vec<usize>,
}

fn parse_address(addr: &str) -> Option<usize> {
    let addr_without_0x = if addr.to_lowercase().starts_with("0x") {
        &addr[2..]
    } else {
        &addr
    };
    usize::from_str_radix(addr_without_0x, 16).ok()
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        let debug_data = match DwarfData::from_file(target) {
            Ok(val) => val,
            Err(DwarfError::ErrorOpeningFile) => {
                println!("Could not open file {}", target);
                std::process::exit(1);
            }
            Err(DwarfError::DwarfFormatError(err)) => {
                println!("Could not debugging symbols from {}: {:?}", target, err);
                std::process::exit(1);
            }
        };
        debug_data.print();
        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        Debugger {
            target: target.to_string(),
            history_path,
            readline,
            inferior: None,
            debug_data: debug_data,
            breakpoints: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Run(args) => {
                    if let Some(inferior) = &mut self.inferior {
                        let r = inferior.kill();
                        if let Err(err) = r {
                            println!("Error killing child: {:?}", err);
                            return;
                        }
                        println!("Killing running inferior (pid {})", inferior.pid());
                        return;
                    }
                    if let Some(inferior) = Inferior::new(&self.target, &args, &self.breakpoints) {
                        // Create the inferior
                        self.inferior = Some(inferior);
                        // You may use self.inferior.as_mut().unwrap() to get a mutable reference
                        // to the Inferior object
                        let inf = self.inferior.as_mut().unwrap();
                        let status = inf.continue_exec();
                        if let Err(err) = status {
                            println!("Error continuing execution: {:?}", err);
                            return;
                        }
                        let status = status.unwrap();
                        if let Status::Exited(code) = status {
                            println!("Child exited (status {})", code);
                            self.inferior = None;
                        }
                        if let Status::Stopped(sign, rip) = status {
                            println!("Child stopped (signal {:?})", sign);
                            let line = self.debug_data.get_line_from_addr(rip);
                            if let Some(line) = line {
                                println!("Stopped at {}", line);
                            }
                        }
                    } else {
                        println!("Error starting subprocess");
                    }
                }
                DebuggerCommand::Cont => {
                    let inf = self.inferior.as_mut().unwrap();
                    let status = inf.continue_exec();
                    if let Err(err) = status {
                        println!("Error continuing execution: {:?}", err);
                        return;
                    }
                    let status = status.unwrap();
                    if let Status::Exited(code) = status {
                        println!("Child exited (status {})", code);
                        self.inferior = None;
                    }
                    if let Status::Stopped(sign, rip) = status {
                        println!("Child stopped (signal {:?})", sign);
                        let line = self.debug_data.get_line_from_addr(rip);
                        if let Some(line) = line {
                            println!("Stopped at {}", line);
                        }
                    }
                }
                DebuggerCommand::Quit => {
                    if let Some(inferior) = &mut self.inferior {
                        let r = inferior.kill();
                        if let Err(err) = r {
                            println!("Error killing child: {:?}", err);
                            return;
                        }
                        println!("Killing running inferior (pid {})", inferior.pid());
                    }
                    return;
                }
                DebuggerCommand::Kill => {
                    if let Some(inferior) = &mut self.inferior {
                        let r = inferior.kill();
                        if let Err(err) = r {
                            println!("Error killing child: {:?}", err);
                            return;
                        }
                        println!("Killing running inferior (pid {})", inferior.pid());
                    }
                    return;
                }
                DebuggerCommand::Backtrace => {
                    if let Some(inferior) = &mut self.inferior {
                        let r = inferior.print_backtrace(&self.debug_data);
                        if let Err(err) = r {
                            println!("Error printing backtrace: {:?}", err);
                            return;
                        }
                    }
                }
                DebuggerCommand::Breakpoint(bp) => {
                    if bp.starts_with("*") {
                        let addr = parse_address(&bp[1..]);
                        if let Some(addr) = addr {
                            if let Some(inferior) = &mut self.inferior {
                                inferior.add_bp(addr);
                            } else {
                                self.breakpoints.push(addr);
                            }
                        }
                        continue;
                    }
                    if let Ok(line) = bp.parse::<usize>() {
                        let addr = self.debug_data.get_addr_for_line(None, line);
                        if let Some(addr) = addr {
                            if let Some(inferior) = &mut self.inferior {
                                inferior.add_bp(addr);
                            } else {
                                self.breakpoints.push(addr);
                            }
                        }
                        continue;
                    }
                    let addr = self.debug_data.get_addr_for_function(None, &bp);
                    if let Some(addr) = addr {
                        if let Some(inferior) = &mut self.inferior {
                            inferior.add_bp(addr);
                        } else {
                            self.breakpoints.push(addr);
                        }
                    }
                }
            }
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str());
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }
}
