use std::{io, thread, time::Duration};
use console::{style, Term};
use clap::{Parser, Subcommand};



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Timer {
        #[arg(short, long)]
        timer_name: String,
    
        #[arg(short, long)]
        duration: u32,
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
    },

}

struct Timer {    
    timer_id: String,
    timer_duration: u32,
}


impl Timer {
    fn cycle (&self) -> io::Result<()> {
        let mut time_left = self.timer_duration;
        let term = Term::stdout();
        term.set_title("🍅 Red Tomatoes!");
        term.write_line(&format!(
            "{} {} {}",
            style("Timer").cyan(),
            style(self.timer_id.clone()).red(),
            style("running...").cyan(),
            ))?;
        term.hide_cursor()?;
        term.write_line("")?;
        while time_left != 0 {
            let minutes = time_left / 60;
            let seconds = time_left % 60;
            term.move_cursor_up(1)?;
            term.write_line(&format!(
                "🍅 {}: {:02}:{:02}",
                style("Timer").cyan(),
                style(minutes).red(), 
                style(seconds).red()
            ))?;


            thread::sleep(Duration::from_secs(1));
            time_left = time_left - 1;
        }
        term.show_cursor()?;
        term.clear_last_lines(1)?;
        term.write_line(&format!("{} 😩💦", style("Timer finished!").green()))?;
        Ok(())
    }
}

fn timer_constructor (name: String, duration: u32) -> Timer {
    Timer { timer_id: name, timer_duration: duration }
}


struct Pomodoro{
    work_duration: u32,
    short_break_duration: u32,
    long_break_duration: u32,
    number_of_cycles: u32,
}

impl Pomodoro{
    fn pomodoro_cycle (&self){
        let mut cycles_left = self.number_of_cycles;
        let work_timer = timer_constructor("Work".to_string(), self.work_duration);
        let short_break_timer = timer_constructor("Short Break".to_string(), self.short_break_duration);
        let long_break_timer = timer_constructor("Long Break".to_string(), self.long_break_duration);
        while cycles_left != 0 {
            work_timer.cycle().unwrap();
            short_break_timer.cycle().unwrap();
            cycles_left = cycles_left - 1;
        }
        long_break_timer.cycle().unwrap();
    }
}




fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Pomodoro { work_duration, short_break_duration, long_break_duration, number_of_cycles } => {
            let my_pomodoro = Pomodoro{work_duration, short_break_duration, long_break_duration, number_of_cycles};
            my_pomodoro.pomodoro_cycle();
        },
        Commands::Timer { timer_name, duration } => {
            let my_timer = timer_constructor(timer_name, duration);
            my_timer.cycle().unwrap();
        },
    }
}