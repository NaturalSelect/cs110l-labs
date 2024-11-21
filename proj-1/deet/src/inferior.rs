use nix::sys::ptrace;
use nix::sys::signal;
use nix::sys::signal::Signal::SIGTRAP;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;
use std::io;
use std::os::unix::process::CommandExt;
use std::process::Child;
use std::process::Command;

use crate::dwarf_data::DwarfData;

pub enum Status {
    /// Indicates inferior stopped. Contains the signal that stopped the process, as well as the
    /// current instruction pointer that it is stopped at.
    Stopped(signal::Signal, usize),

    /// Indicates inferior exited normally. Contains the exit status code.
    Exited(i32),

    /// Indicates the inferior exited due to a signal. Contains the signal that killed the
    /// process.
    Signaled(signal::Signal),
}

/// This function calls ptrace with PTRACE_TRACEME to enable debugging on a process. You should use
/// pre_exec with Command to call this in the child process.
fn child_traceme() -> Result<(), std::io::Error> {
    ptrace::traceme().or(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "ptrace TRACEME failed",
    )))
}

#[derive(Clone)]
struct Breakpoint {
    addr: usize,
    orig_byte: u8,
}

pub struct Inferior {
    child: Child,
    breakpoint: Vec<Breakpoint>,
}

fn align_addr_to_word(addr: usize) -> usize {
    addr & (-(size_of::<usize>() as isize) as usize)
}

impl Inferior {
    /// Attempts to start a new inferior process. Returns Some(Inferior) if successful, or None if
    /// an error is encountered.
    pub fn new(target: &str, args: &Vec<String>, bps: &Vec<usize>) -> Option<Inferior> {
        let mut cmd = Command::new(target);
        cmd.args(args);
        unsafe {
            cmd.pre_exec(child_traceme);
        }
        let child = cmd.spawn();
        if let Err(err) = child {
            println!("Error: {:?}", err);
            return None;
        }

        let child = child.unwrap();
        let mut inf = Inferior {
            child: child,
            breakpoint: Vec::new(),
        };
        for i in bps {
            inf.add_bp(*i);
        }
        return Some(inf);
    }

    /// Returns the pid of this inferior.
    pub fn pid(&self) -> Pid {
        nix::unistd::Pid::from_raw(self.child.id() as i32)
    }

    /// Calls waitpid on this inferior and returns a Status to indicate the state of the process
    /// after the waitpid call.
    pub fn wait(&self, options: Option<WaitPidFlag>) -> Result<Status, nix::Error> {
        Ok(match waitpid(self.pid(), options)? {
            WaitStatus::Exited(_pid, exit_code) => Status::Exited(exit_code),
            WaitStatus::Signaled(_pid, signal, _core_dumped) => Status::Signaled(signal),
            WaitStatus::Stopped(_pid, signal) => {
                let regs = ptrace::getregs(self.pid())?;
                Status::Stopped(signal, regs.rip as usize)
            }
            other => panic!("waitpid returned unexpected status: {:?}", other),
        })
    }

    fn hint_breakpoint(&self, addr: usize) -> Option<Breakpoint> {
        for bp in &self.breakpoint {
            if bp.addr == addr {
                return Some(bp.clone());
            }
        }
        return None;
    }

    pub fn add_bp(&mut self, addr: usize) {
        let ori_byte = self.write_byte(addr, 0xcc);
        if let Err(err) = ori_byte {
            println!("Skip breakpoint error: {:?}", err);
            return;
        }
        let ori_byte = ori_byte.unwrap();
        self.breakpoint.push(Breakpoint {
            addr: addr,
            orig_byte: ori_byte,
        });
    }

    pub fn continue_exec(&mut self) -> Result<Status, nix::Error> {
        let regs = ptrace::getregs(self.pid());
        if let Err(err) = regs {
            return Err(err);
        }
        let mut regs = regs.unwrap();
        // NOTE: if we hint breakpoint, we should restore the original byte
        // and set it's next instruction to INT
        let bp = self.hint_breakpoint(regs.rip as usize);
        let copy_bp = bp.clone();
        let mut restore_byte: Option<u8> = None;
        if let Some(bp) = bp {
            // NOTE: restore the original byte
            let ori_byte = self.write_byte(bp.addr, bp.orig_byte);
            if let Err(err) = ori_byte {
                return Err(err);
            }
            regs.rip = bp.addr as u64;
            let r = ptrace::setregs(self.pid(), regs);
            if let Err(err) = r {
                return Err(err);
            }

            // NOTE: move to next step
            let r = ptrace::step(self.pid(), SIGTRAP);
            if let Err(err) = r {
                return Err(err);
            }
            let r = self.wait(None);
            if let Err(err) = r {
                return Err(err);
            }

            // NOTE: set the next instruction to INT
            let regs = ptrace::getregs(self.pid());
            if let Err(err) = regs {
                return Err(err);
            }
            let regs = regs.unwrap();
            let ori_byte = self.write_byte(regs.rip as usize, 0xcc);
            if let Err(err) = ori_byte {
                return Err(err);
            }
            restore_byte = Some(ori_byte.unwrap());
        }

        let r = ptrace::cont(self.pid(), None);
        if let Err(err) = r {
            return Err(err);
        }

        // NOTE: if we hint bp
        if let Some(restore_byte) = restore_byte {
            let r = self.wait(None);
            if let Err(err) = r {
                return Err(err);
            }
            // NOTE: reset breakpoint
            let bp = copy_bp.unwrap();
            let ori_byte = self.write_byte(bp.addr, 0xcc);
            if let Err(err) = ori_byte {
                return Err(err);
            }
            // NOTE: restore the original byte
            let ori_byte = self.write_byte(bp.addr, restore_byte);
            if let Err(err) = ori_byte {
                return Err(err);
            }
            // NOTE: continue
            let r = ptrace::cont(self.pid(), None);
            if let Err(err) = r {
                return Err(err);
            }
        }
        return self.wait(None);
    }

    // pub fn on_stop(&mut self) {
    //     let regs = ptrace::getregs(self.pid()).unwrap();
    //     let bp = self.hint_breakpoint(regs.rip as usize);
    //     if let Some(bp) = bp {
    //         let ori_byte = self.write_byte(bp.addr, 0xcc);
    //         if let Err(err) = ori_byte {
    //             println!("Error: {:?}", err);
    //             return;
    //         }
    //         println!("Breakpoint at {:x}", bp.addr);
    //     }
    // }

    pub fn kill(&mut self) -> io::Result<()> {
        return self.child.kill();
    }

    fn write_byte(&mut self, addr: usize, val: u8) -> Result<u8, nix::Error> {
        let aligned_addr = align_addr_to_word(addr);
        let byte_offset = addr - aligned_addr;
        let word = ptrace::read(self.pid(), aligned_addr as ptrace::AddressType)? as u64;
        let orig_byte = (word >> 8 * byte_offset) & 0xff;
        let masked_word = word & !(0xff << 8 * byte_offset);
        let updated_word = masked_word | ((val as u64) << 8 * byte_offset);
        ptrace::write(
            self.pid(),
            aligned_addr as ptrace::AddressType,
            updated_word as *mut std::ffi::c_void,
        )?;
        Ok(orig_byte as u8)
    }

    pub fn print_backtrace(&self, dbg_data: &DwarfData) -> Result<(), nix::Error> {
        let regs = ptrace::getregs(self.pid());
        if let Err(err) = regs {
            return Err(err);
        }
        let regs = regs.unwrap();
        let mut rip = regs.rip as usize;
        let mut rbp = regs.rbp as usize;
        loop {
            let func = dbg_data.get_function_from_addr(rip).unwrap();
            let line = dbg_data.get_line_from_addr(rip).unwrap();
            println!("{} ({})", func, line);
            if func == "main" {
                break;
            }
            rip = ptrace::read(self.pid(), (rbp + 8) as ptrace::AddressType).unwrap() as usize;
            rbp = ptrace::read(self.pid(), rbp as ptrace::AddressType).unwrap() as usize;
        }
        return Ok(());
    }
}
