# Rust Setup

Run the following commands to get your rust environment setup:

```bash
$ rustup target add thumbv6m-none-eabi
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
```

Install flip-link to ensure memory safety. `flip-link` implements stack overflow solution. Linking your program with
flip-link produces the flipped memory layout, which is memory safe in presence of stack overflows.

```bash
$ cargo install flip-link
```

## One Pico Board Setup

If you want to work with one Pico microcontroller, then you will need to convert the cargo build output binary (.ELF)
into the.UF2 binary format. You can do this with a tool called elf2uf2. In your terminal, run the following in the
project’s directory:

```bash
$ cargo install elf2uf2-rs
```

After the tool is installed, you can use the tool after a build of your project to produce a.uf2 file.

```bash
$ cargo build --release
$ elf2uf2-rs target/thumbv6m-none-eabi/release/display_i2c
```

If you want to automatically run elf2uf2 when you type cargo run or cargo build - in the .cargo/config.toml, you need to
set your runner to elf2uf2-rs:

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "elf2uf2-rs"
```

And for further automation, if you want to automatically flash your Pico board with the new uf2 binary, put your Pico in
transfer mode, connect it to the usb, and add the -d option like so:

```bash
$ cargo build --release
$ elf2uf2-rs -d target/thumbv6m-none-eabi/release/display_i2c
```
