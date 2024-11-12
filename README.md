<div align="center">
<h1>rldr</h1>
<p><em>[ ree-loh-der ]</em></p>
<p>A simple command line utility to run and manage commands with keypress controls.</p>
</div>

## Features

- **Run any command**: Start any command (e.g., `python3 main.py`, `ping google.com`, etc.).
- **Real-time output**: Captures both `stdout` and `stderr` and displays them as they are generated.
- **Interactive controls**:
  - Press `r` to restart the command.
  - Press `c` to close the command.
  - Press `q` to quit the reloader.

## Installation
In order to install `rldr`, you need to have `cargo` installed. If you don't have Rust or `cargo` installed, you can find instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### Cargo
```
cargo install rldr
rldr --help
```

### Source
```bash
git clone https://github.com/ammar-ahmed22/rldr.git
cd rldr
cargo install --path .
```

## Usage
To use `rldr`, you simply pass in the command you want to run. `rldr` will start the command and provide interactive controls.
```bash
rldr ping google.com
```

### Example Session
Upon running `rldr` it will display:
```bash
[rldr] Enter `r` to reload, `c` to close, `q` to quit
```
You can then enter:
- `r` to restart the command, which terminates the current instance and starts it again.
- `c` to close the command, stopping it's execution (but still listening for input)
- `q` to quit out of `rldr`

## Limitations and Known Issues
- *Platform Compatibility*: `rldr` relies on `bash` on Unix systems and `cmd` on Windows. Ensure these shells are available on your system
- *Non-blocking I/O*: While `rldr` works well with real-time commands, some programs with heavy buffering may still delay output display. Commands that require user input will also not work due to the input handling from `rldr`
- *Output Thread Management*: Each input creates new threads to handle input, so frequent restarts may consume system resources over time.

## Contributing
Contributions, issues, and feature requests are welcome! Feel free to open an issue or pull request to help improve `rldr`.

## License
This project is licensed under the [MIT License](./LICENSE).