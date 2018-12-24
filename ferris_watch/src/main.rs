use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use log::debug;
use clap::{App, Arg, value_t};

fn main() -> Result<(), failure::Error>{
    env_logger::init();
    debug!("ferris_watch starting...");

    let matches = App::new("ferris_watch")
        .version("0.1.0")
        .author("Suguru Araki sgr.araki@gmail.com")
        .about("cute watch command")
        .arg(
            Arg::with_name("command")
                .required(true)
                .multiple(true)
                .help("The command to run periodically"),
        )
        .arg(
            Arg::with_name("interval")
                .long("interval")
                .short("n")
                .takes_value(true)
                .default_value("2.0")
                .help("The period to run a command"),
        )
        .get_matches();

    let command =
        matches.values_of("command").unwrap().collect::<Vec<_>>();
    let interval = value_t!(matches, "interval", f64)?;
    debug!("command = {:?}", command);
    debug!("interval = {:?}", interval);
    let interval10 = (interval * 10.0) as u32;

    let interrupted = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGINT, interrupted.clone())?;
    let interrupted = || interrupted.load(Ordering::SeqCst);

    let window = ncurses::initscr();
    struct EndWin;
    impl Drop for EndWin {
        fn drop(&mut self) {
            ncurses::endwin();
        }
    }
    let _endwin = EndWin;

    /*
    let frame_out = {
        let (h, w) = window.get_max_yx();
        window.subwin(h - 5, w, 0, 0).unwrap();
    };
    let frame_in = {
        let (height, width) = frame_out.get_max_yx();
        window.subwin(height - 2, width - 2, 1, 1).unwrap();
    };
    */

    'outer: loop {
        let output = Command::new(command[0]).args(&command[1..]).output()?;
        debug!("output = {:?}", output);
        let stdout = String::from_utf8_lossy(&output.stdout);
        ncurses::clear();
        ncurses::printw(&stdout.to_string());
        ncurses::printw("       \\\n");
        ncurses::printw("         _~^~^~_\n");
        ncurses::printw("     \\) /  o o  \\ (/\n");
        ncurses::printw("       '_   -   _'\n");
        ncurses::printw("       / '-----' \\\n");
        ncurses::refresh();

        for _ in 0..interval10 {
            sleep(Duration::from_millis(100));
            if interrupted() {
                break 'outer;
            }
        }
        if interrupted() {
            break 'outer;
        }
    }

    log::debug!("end");
    Ok(())
}
