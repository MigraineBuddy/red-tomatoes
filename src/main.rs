
/*
#TODO:

-> add pause and stop buttons.
-> refactor to make it neater.
-> add progress bar for timer and pomodoro.
-> sprinkle cutesy animation and tweaks around.
-> Enable Desktop notifications
 
 */

pub mod cli;
pub mod timers;

use clap::Parser;
use cli::*;
use console::Color;
use timers::*;



fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Pomodoro { work_duration, short_break_duration, long_break_duration, number_of_cycles, notification } => {
            let my_pomodoro = pomodoro_constructor(
                work_duration, 
                short_break_duration, 
                long_break_duration, 
                number_of_cycles
            );
            my_pomodoro.pomodoro_cycle(notification);
        },
        Commands::Timer { timer_name, duration, notification} => {
            let my_timer = timer_constructor("Red-Tomatoes".to_string(),timer_name, duration, Color::Red);
            my_timer.timer_cycle(notification);
        },
    }
}