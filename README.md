# ⏲️🍅 Red-Tomatoes 😩💅
## About 📝
Red-Tomatoes is a command line timer app mainly built on the [Console](https://github.com/console-rs/console) and [Clap](https://github.com/clap-rs/clap) crates. Designed while I was procrastinating to avoid procrastinating. (I still ended up procrastinating btw 😭)
## Installation 🛠️
### Through Cargo 📦
Requires a [rust installation](https://www.rust-lang.org/tools/install) to build.
```sh
# Clone repository to any given folder.
git clone https://github.com/MigraineBuddy/red-tomatoes.git

# Change directory to repo clone
cd ./red-tomatoes/

# Cargo will do the rest of the heavy lifting.
cargo build -r && cargo install --path .
```

### Manual install 💪
*Note: Only available on Windows for now.*
Download the [latest release](https://github.com/MigraineBuddy/red-tomatoes/releases) and run directly. A rust install shouldn't be necessary.

```sh
./red-tomatoes.exe --help
```

### Uninstall 🗑️
You will be missed 😔 <3

```sh
cargo uninstall red-tomatoes

# To delete the cloned repo from the command line.
rm -rf ./red-tomatoes/
```
## Usage 🤓☝️
The `red-tomatoes` command comprises two subcommands that you need to know about: `pomodoro`, which is a standard pomodoro timer with default work and break time durations, and `timer`, which you can use as a standard timer.
```sh
# Basic timer usage. (duration is in seconds)
red-tomatoes timer --timer-name "Boiling an egg." --duration 900

# Pomodoro timer usage. it'll run the default Pomodoro timer if ran without any arguments.
# Work (25 mins) -> Short Break (5 mins) -> Work -> Short Break -> Work -> Long Break (15 mins)
red-tomatoes pomodoro

# For more info.
red-tomatoes --help 
```

## Under Construction 🚧:
- Add pause and stop buttons.
- Refactor the code to make it neater.
- Add a progress bar for the timer and Pomodoro.
- ~Incorporate color configuration for timers into the structs.~
- ~Divide the code into multiple different files.~
- Add cutesy animations and other tweaks to enhance the user experience.
- ~Add desktop notifications.~
