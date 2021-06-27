#![no_std]
#![no_main]
program!(0xFFFFFFFE, "GPL");

use redbpf_probes::{net::Transport, xdp::prelude::*};

#[xdp("drop_isn")]
pub fn drop_isn(ctx: XdpContext) -> XdpResult {
    if let Ok(Transport::TCP(tcp)) = ctx.transport() {
        unsafe {
            let sn = u32::from_be((*tcp).seq);
            if (sn & 0x00FFFFFF) == 0 {
                Ok(XdpAction::Drop)
            } else {
                Ok(XdpAction::Pass)
            }
        }
    } else {
        Ok(XdpAction::Pass)
    }
}
