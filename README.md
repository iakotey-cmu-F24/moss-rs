# Moss-rs

Moss-rs is an implementation of the MOSS client in pure Rust. Moss-rs currenly exports a library to provide reusable access to it's components, as well as a binary that acts as a client for the **Moss** service.

## About Moss

Moss is a code similarity analysis tool developed by Alex Aiken, Daniel S. Wilkerson and Saul Schleimer. It is useful as a tool for detecting similarity and ***possible*** plagiarism. **It is not, in itself, a plagiarism checker** as it cannot deterministically ascertain whether or not plagiarism occured.

Moss is provided as an internet service. This project is just an implementation of the reference client, found [here](http://moss.stanford.edu/general/scripts/mossnet).

## Installation

The current implementation is not yet on [crates.io](https://crates.io) or as binary executables, so manual building is required.

### Prerequisites

1. Rust Compiler. Install using [rustup](https://rustup.rs)
2. Git
3. Basic familiarity with your terminal

### Build steps

1. clone the repository

```bash
git clone https://github.com/ianakotey/moss-rs
```

2. Change directory (cd) into repository directory

```bash
cd MOSS-RS # folder name might not be uppercase
```

3. Build moss-rs

```bash
cargo build --release
```

4. The binary will be found at ***Project*/target/release/moss.exe**, where project is the project directory. It is recommended to copy/move the binary to a location of your choice an add that location to your system's PATH variable. See [this article](https://tadtadya.com/en/how-to-set-environment-variable-path/) for help if needed.

## Usage

### **Binary**

Most users will be interested in the binary executable. Usage instructions will be provided in more detail. Below is a summary of the moss CLI.

```bash
moss [OPTIONS] --comment <COMMENT> --user-id <USER_ID> [FILES]...
```

### **Library**

More documentation will be provided once the library is ready to publish.

## Future works

* A GUI desktop application
* Extra features like regex transforms for display names

## Contributing

    Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change. Please mark it with the *feature* label

    For bugs, documentation, and any questions, please create an issue with the appropriate label.

Please make sure to update any tests as appropriate.

## Credits

* [Moss](https://theory.stanford.edu/~aiken/moss/)
* Hjalti Magnussion, for [mossum](https://github.com/hjalti/mossum). Awesome projects like these inspire folks like me to make something that might help others too.
* Banahene Osei-Owusu, for ideas, support and project management.

## License

[GPL3](https://choosealicense.com/licenses/gpl-3.0/)
