pub mod commands;
use clap::Parser;
pub use commands::CommandError;

mod exit_with_error;
pub use exit_with_error::exit_with_error;

#[cfg(feature = "unsafe-signal-handlers")]
mod unsafe_signal_handlers {
    use nix::libc;

    extern "C" fn handle_sigsegv(_signal: libc::c_int) {
        eprintln!("########### SIGSEGV #############");
        snarker::recorder::Recorder::graceful_shutdown();
        std::process::exit(1);
    }

    pub fn setup() {
        let stack_t = libc::stack_t {
            ss_sp: {
                let stack = Box::<[u8; libc::SIGSTKSZ]>::new([0; libc::SIGSTKSZ]);
                Box::into_raw(stack) as *mut _
            },
            ss_flags: 0,
            ss_size: libc::SIGSTKSZ,
        };

        let res = unsafe { libc::sigaltstack(&stack_t as *const _, std::ptr::null_mut()) };
        assert_eq!(res, 0);

        let action = libc::sigaction {
            sa_sigaction: handle_sigsegv as _,
            sa_mask: unsafe { std::mem::zeroed() },
            sa_flags: libc::SA_ONSTACK,
            sa_restorer: None,
        };
        let res = unsafe { libc::sigaction(libc::SIGSEGV, &action as _, std::ptr::null_mut()) };
        assert_eq!(res, 0);
    }
}

fn main() {
    #[cfg(feature = "unsafe-signal-handlers")]
    unsafe_signal_handlers::setup();

    match commands::OpenminaCli::parse().command.run() {
        Ok(_) => {}
        Err(err) => exit_with_error(err),
    }
}
