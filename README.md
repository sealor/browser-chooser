Browser Chooser
===============

This tool let's you choose the browser when opening a web link in a desktop application. A small pop-up window is shown where you can decide which browser to open - Mozilla Firefox or Google Chrome.

Implementation
--------------

The UI is implemented with <https://iced.rs>.

> A cross-platform GUI library for Rust focused on simplicity and type-safety.

The code is just a single [main.rs](src/main.rs). There is no configuration currently. PRs are welcome.

Build on Linux for Windows
--------------------------

```bash
# when cargo is installed via Nix package manager
export NIX_STORE=/nix/store
export PATH=$PATH:~/.cargo/bin

cargo install cross
cross build --target x86_64-pc-windows-gnu --release
```

License
-------

[MIT License](LICENSE)
