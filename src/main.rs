use redbpf::{Module, xdp};
use std::error::Error;
use tokio::runtime;
use tokio::signal::ctrl_c;

fn main() -> Result<(), Box<dyn Error>> {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let prog = include_bytes!("../target/bpf/programs/redirect/redirect.elf");
        let mut module = Module::parse(prog).expect("error parsing BPF code");

        for program in module.programs.iter_mut() {
            println!("Loading program: {}", program.name());
            program.load(module.version, module.license.clone()).unwrap()
        }

        for prog in module.xdps_mut() {
            println!("Attaching XDP: {}", prog.name());
            prog.attach_xdp("lo", xdp::Flags::default())
                .expect("Failed to attach program");
        }

        println!("Attached programs. Press Ctrl-C to exit.");
        let _ = ctrl_c().await;
        println!("Exiting...");

        Ok(())
    })
}
