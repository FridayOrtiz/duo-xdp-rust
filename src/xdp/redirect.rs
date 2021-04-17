#![no_std]
#![no_main]
program!(0xFFFFFFFE, "GPL");

use redbpf_probes::{bindings::udphdr, net::Transport, xdp::prelude::*};

#[xdp("redirect")]
pub fn redirect(ctx: XdpContext) -> XdpResult {
    if let Ok(Transport::UDP(udp)) = ctx.transport() {
        unsafe {
            if u16::from_be((*udp).dest) == 7999 {
                (*(udp as *mut udphdr)).dest = u16::to_be(7998);
            }
        }
    }

    Ok(XdpAction::Pass)
}
