use std::{io, thread, time::Duration};
use console::{style, Term, Color, Key};
use notify_rust::Notification;


pub struct Timer {
    timer_term_id: String,
    timer_id: String,
    timer_duration: u32,
    timer_color: Color,
}


impl Timer {
    pub fn cycle (&self, notification: bool) -> io::Result<()> {
        // #FIXME: make this less cancer.
        let mut time_left = self.timer_duration;
        let term = Term::stdout();
        term.set_title(self.timer_term_id.clone());
        term.write_line(&format!("Timer {} running...", style(self.timer_id.clone()).fg(self.timer_color)))?;
        term.write_line(&format!("Press q to quit."))?;
        term.hide_cursor()?;
        term.write_line("")?;
        while time_left != 0 {
            let minutes = time_left / 60;
            let seconds = time_left % 60;
            term.move_cursor_up(1)?;
            term.write_line(&format!(
                "🍅 Timer: {:02}:{:02}",
                style(minutes).fg(self.timer_color), 
                style(seconds).fg(self.timer_color),
            ))?;
            time_left = time_left - 1;
            thread::sleep(Duration::from_secs(1));
        }
        term.show_cursor()?;
        term.clear_last_lines(1)?;
        term.write_line(&format!("{} 😩💦", style("Timer finished!").green()))?;
        let body = format!("Timer: {} finished! 😩💦", self.timer_id.clone());
        if notification {
        Notification::new()
        .summary("Timer ended.")
        .body(&body.as_str())
        .show()
        .unwrap();
        }

        Ok(())
    }

    pub fn timer_cycle (&self,notification: bool){
        self.cycle(notification).unwrap()
    }

}

pub fn timer_constructor (term_name: String,name: String, duration: u32, color: Color) -> Timer {
    Timer {
        timer_term_id: term_name,
        timer_id: name, 
        timer_duration: duration, 
        timer_color: color,
    }
}


pub struct Pomodoro{
    work_duration: u32,
    short_break_duration: u32,
    long_break_duration: u32,
    number_of_cycles: u32,
}


impl Pomodoro{
    pub fn pomodoro_contents(&self) -> (u32, u32, u32, u32){
    (self.work_duration, self.short_break_duration, self.long_break_duration, self.number_of_cycles)
    }

    pub fn pomodoro_cycle (&self, notification: bool){
        let pomodoro_string = String::from("Pomodoro");
        let mut cycles_left = self.number_of_cycles;
        let work_timer = timer_constructor(pomodoro_string.clone(), "Work".to_string(), self.work_duration, Color::Red);
        let short_break_timer = timer_constructor(pomodoro_string.clone(),"Short Break".to_string(), self.short_break_duration, Color::Blue);
        let long_break_timer = timer_constructor(pomodoro_string,"Long Break".to_string(), self.long_break_duration, Color::Yellow);
        
        while cycles_left != 0 {
            work_timer.cycle(notification).unwrap();
            short_break_timer.cycle(notification).unwrap();
            cycles_left = cycles_left - 1;
        }
        long_break_timer.cycle(notification).unwrap();
    }
}

pub fn pomodoro_constructor (work_time: u32, short_break_time: u32, long_break_time: u32, cycle_number: u32) -> Pomodoro {
    Pomodoro{
        work_duration: work_time,
        short_break_duration: short_break_time,
        long_break_duration: long_break_time,
        number_of_cycles: cycle_number
    }
}
