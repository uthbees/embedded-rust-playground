Requirements:

- Install gdb-multiarch
- Install OpenOCD

See https://docs.rust-embedded.org/cortex-m-quickstart/cortex_m_quickstart/ for details/troubleshooting.

To run, execute these commands in separate terminals:

```bash
openocd -f ./stlink.cfg
```

```bash
cargo run
```

The program can be reset by running `monitor reset` in the gdb console (the second terminal).
