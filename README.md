# Inspiration

This project was inspired by
[duo labs blog post](https://duo.com/labs/tech-notes/writing-an-xdp-network-filter-with-ebpf).
I was able to get the redirection filter working pretty cleanly in C, but I wanted to replicate
the functionality in Rust with [RedBPF](https://github.com/foniod/redbpf). I ended up basing the project
structure on [aquarhead/protect-the-rabbit](https://github.com/aquarhead/protect-the-rabbit), with some
modifications per [this blog](https://kbknapp.dev/ebpf-part-iii/).

# Requirements

*  [Cargo BPF](https://ingraind.org/api/cargo_bpf/)
*  [Cargo Make](https://github.com/sagiegurari/cargo-make)
*  [RedBPF Dependencies](https://github.com/foniod/redbpf#installing-dependencies-on-debian-based-distributions)

# Building

Simply run `cargo make release` to build the program and loader. If you don't have `cargo make`, you can open
the `Makefile.toml` file and perform each step manually.

# Running

You can run the program with `sudo ./target/release/duo-xdp-rust`. Press `Ctrl-C` to exit the program.

While running, you can test it by opening three terminals before running. In the first terminal
run `nc -kul 127.0.0.1 7998`; in the second terminal run `nc -kul 127.0.0.1 7999`; and in the third
terminal run `nc -u 127.0.0.1 7999`. This will open two loopback UDP listeners on ports `7998` and `7999`,
and one loopback sender to UDP port `7999`. Type some text into terminal three and hit enter, the text should
appear in terminal two. Now in a fourth terminal, run the program with `sudo ./target/release/duo-xdp-rust`. Once
it's running, type some new text into terminal three and hit enter. It should now appear in terminal one, with
the destination port being overwritten from `7999` to `7998`. In terminal four, hit `Ctrl-C` to stop the program
and enter some text back in terminal three. The new text should appear back in terminal two again, with no
redirection.

# Manual Loading

The eBPF program can also be manually loaded. Add the following snippet to your `Cargo.toml` file 
([source](https://github.com/neallred/redbpf-postprocessing/blob/master/user.sh#L20)).

```toml
[tasks.postprocessing_hack]
command = "llvm-objcopy"
args = ["--rename-section", "xdp/redirect=prog", "target/bpf/programs/redirect/redirect.elf"]
dependencies = ["bpf"]
```

Then comment out the `[tasks.release]` section. Run `cargo make postprocessing_hack` and load the program
as follows:

```
$ sudo ip link set dev lo xdp obj target/bpf/programs/redirect/redirect.elf
```

You can remove it with:

```
$ sudo ip link set dev lo xdp off
```