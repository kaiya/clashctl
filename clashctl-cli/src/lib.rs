pub use clap;
pub(crate) use clashctl_interactive::clashctl::{self, model};

use clap::Parser;
use clashctl::mod_use;
use log::{debug, LevelFilter};

use crate::{init_logger, Cmd, Opts};

mod_use![command, proxy_render, utils, error];

pub fn run() {
    let opts = Opts::parse();

    #[cfg(feature = "tui")]
    if let Cmd::Tui(opt) = opts.cmd {
        if let Err(e) = clashctl_tui::main_loop(opt, opts.flag) {
            eprintln!("{:?}", e);
        }
        return;
    }

    init_logger(match opts.flag.verbose {
        0 => Some(LevelFilter::Info),
        1 => Some(LevelFilter::Debug),
        2 => Some(LevelFilter::Trace),
        _ => None,
    });

    debug!("Opts: {:#?}", opts);

    let res = match opts.cmd {
        #[cfg(feature = "tui")]
        Cmd::Tui(_) => unreachable!(),
        Cmd::Proxy(sub) => sub.handle(&opts.flag),
        Cmd::Server(sub) => sub.handle(&opts.flag),
        Cmd::Completion(arg) => arg.handle(),
    };
    if let Err(e) = res {
        eprintln!("{:?}", e)
    }
}
