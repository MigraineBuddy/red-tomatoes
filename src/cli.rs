use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(
    author = "Noelle Brooks", 
    version = "v0.1", 
    about = "🍅🦀 A yassified CLI Pomodoro timer written in pure rust for all your procrastination needs.", 
    long_about = None
)]
#[command(propagate_version = true)]


pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Timer {
        #[arg(short, long)]
        timer_name: String,
    
        #[arg(short, long)]
        duration: u32,

        #[arg(long, default_value_t = true)]
        notification: bool,

    },
    Pomodoro {
        #[arg(short, long, default_value_t = 25*60)]
        work_duration: u32,

        #[arg(short, long, default_value_t = 5*60)]
        short_break_duration: u32,

        #[arg(short, long, default_value_t = 15*60)]
        long_break_duration: u32,

        #[arg(short, long, default_value_t = 2)]
        number_of_cycles: u32,

        #[arg(long, default_value_t = true)]
        notification: bool,
    },

}
