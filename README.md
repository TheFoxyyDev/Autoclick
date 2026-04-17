#  Rust Autoclicker

A lightweight, high-performance cross-platform autoclicker built with Rust. It features a command-line interface for configuration and a global hotkey to toggle clicking while the program is running in the background.

## Features

-   **Global Hotkey:** Toggle the clicking state  using the `F8` key.
-   **Adjustable Speed:** Custom delay (in milliseconds) between clicks.
-   **CLI Controls:** Simple terminal commands to manage settings.

## Installation

### Prerequisites

Ensure you have the [Rust toolchain](https://rustup.rs/) installed.

### Build from source

1.  Clone the repository:
    ```bash
    git clone https://github.com/TheFoxyyDev/Autoclick.git
    cd autoclicker
    ```

2.  Build the project:
    ```bash
    cargo build --release
    ```

3.  Run the application:
    ```bash
    ./target/release/autoclicker
    ```

## Usage

Once the application is running, it listens for both keyboard commands in the terminal and global hotkeys.

### Global Hotkeys
-   **`F8`**: Toggle the Autoclicker **ON** or **OFF**.

### CLI Commands
Type these commands into the terminal and press `Enter`:

| Command | Action |
| :--- | :--- |
| `s` | **Set Speed:** Prompts you to enter a delay in milliseconds. |
| `h` | **Help:** Displays the command list. |
| `q` | **Quit:** Safely exits the application. |

> **Performance Note:** Setting a speed below **25ms** may result in high CPU usage or cause your operating system to become unresponsive. Use low values with caution.

## Dependencies

This project leverages the following crates:
-   [`enigo`](https://crates.io/crates/enigo): For simulating mouse input.
-   [`device_query`](https://crates.io/crates/device_query): For global keyboard state detection.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---
*Created with ❤️ using Rust.*