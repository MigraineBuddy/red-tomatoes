# 🍅 Red-Tomatoes 😩💅
## Installation
### Through Cargo
Requires a rust installation to build.
```sh
# clone repository to any given folder.
git clone https://github.com/MigraineBuddy/red-tomatoes.git
# change directory to repo clone
cd ./red-tomatoes/
# cargo will do the rest of the heavy lifting.
cargo build -R && cargo install --path .
```

### Manual install
*Note: only available on windows for now.*
Download the latest release and run directly. A rust install shouldn't be necessary.

```sh
./red-tomatoes.exe --help
```

## Roadmap
- add pause and stop buttons
- refactor to make it neater.
- add progress bar for timer and pomodoro.
- add color configuration for timers (into the structs).
- spread out to multiple different files.
- sprinkle cutesy animation and tweaks around.
