# 🍅 Red-Tomatoes 😩💅
## About
Red-Tomatoes is a command line timer app mainly built on the [Console](https://github.com/console-rs/console) and [Clap](https://github.com/clap-rs/clap) crates. Designed while I was procrastinating to avoid procrastinating. (I still ended up procrastinating btw ;-;)
## Installation
### Through Cargo
Requires a [rust installation](https://www.rust-lang.org/tools/install) to build.
```sh
# Clone repository to any given folder.
git clone https://github.com/MigraineBuddy/red-tomatoes.git
# change directory to repo clone
cd ./red-tomatoes/
# Cargo will do the rest of the heavy lifting.
cargo build -r && cargo install --path .
```

### Manual install
*Note: only available on Windows for now.*
Download the [latest release](https://github.com/MigraineBuddy/red-tomatoes/releases) and run directly. A rust install shouldn't be necessary.

```sh
./red-tomatoes.exe --help
```

## Usage
The `red-tomatoes` command comprises two subcommands that you need to know about: `pomodoro`, which is a standard pomodoro timer with default work and break time durations, and `timer`, which you can use as a standard timer.
```sh
# For more info
red-tomatoes --help 
```

Please find the revised text below with spelling, grammar, and punctuation errors corrected:

## Roadmap:
- Add pause and stop buttons.
- Refactor the code to make it neater.
- Add a progress bar for the timer and Pomodoro.
- ~Incorporate color configuration for timers into the structs.~
- ~Divide the code into multiple different files.~
- Add cutesy animations and other tweaks to enhance the user experience.
- ~Add desktop notifications.~
