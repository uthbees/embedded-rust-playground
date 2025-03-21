Embedded Rust stuff for the NUCLEO-L476RG board.

### Setting up

Everything is standard git/Rust, except that the git hooks are stored in the `.githooks` directory. Run
`git config --local core.hooksPath .githooks` when setting up a new environment.

### Requirements

- Install gdb-multiarch
- Install OpenOCD

See https://docs.rust-embedded.org/cortex-m-quickstart/cortex_m_quickstart/ for details/troubleshooting.

### Running

The best way to run this is to use the debug option with CLion's OpenOCD configuration. However, if you need to run it
manually, you can do so by executing these commands in separate terminals:

```bash
openocd -f ./stlink.cfg
```

```bash
cargo run
```

When running manually, the program can be reset by running `monitor reset` in the gdb console (the second terminal).
