# Rust Setup

Run the following commands to get your rust environment setup:

```bash
$ rustup target add thumbv6m-none-eabi
$ cargo install cargo-binutils
$ rustup component add llvm-tools-
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
$ elf2uf2-rs target/thumbv6m-none-eabi/release/blink
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
$ elf2uf2-rs -d target/thumbv6m-none-eabi/release/blink
```

## Two Pico Board Setup

If you haven’t setup your board for debugging yet, you can start
here: https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html#debugging-using-another-raspberry-pi-pico

Make sure to download the picoprobe UF2 file from the link above and flash one of the Picos with this file. This will
become your debug Pico.

Also, see Apendix A of https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf for a diagram of how to
wire your two Picos together for debugging. This is where you will be using the breadboard and wires.

With a picoprobe debugger, we can use the Rust default output format binary (.ELF). To flash the.elf file onto the Pico,
we will use OpenoCD.

Let’s install OpenoCD

If you have used the C/C++ Pico SDK, you probably already have this installed. You can skip this section if you do.

For MacOS:

```bash
$ brew install open-ocd
```

For Ubuntu:

```bash
$ sudo apt update
$ sudo apt -y install openocd
```

Let’s Install GDB

If you have used the C/C++ Pico SDK, you probably already have this installed. You can skip this section if you do.

For MacOS:

```bash
$ brew tap ArmMbed/homebrew-formulae
$ brew install arm-none-eabi-gcc
```

For Ubuntu:

```bash
$ sudo apt install git gdb-multiarch
```
