#![no_std]
#![no_main]

use core::mem;

use demo_common::BackendPorts;

use aya_bpf::{bindings::xdp_action, macros::xdp, maps::HashMap, programs::XdpContext};
use aya_log_ebpf::info;

mod bindings;
use bindings::{ethhdr, iphdr, udphdr};

const IPPROTO_UDP: u8 = 0x0011;
const ETH_P_IP: u16 = 0x0800;
const ETH_HDR_LEN: usize = mem::size_of::<ethhdr>();
const IP_HDR_LEN: usize = mem::size_of::<iphdr>();

#[inline(always)]
fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Option<*const T> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return None;
    }

    Some((start + offset) as *const T)
}

#[inline(always)]
fn ptr_at_mut<T>(ctx: &XdpContext, offset: usize) -> Option<*mut T> {
    let ptr = ptr_at::<T>(ctx, offset)?;
    Some(ptr as *mut T)
}

#[xdp(name = "demo")]
pub fn demo(ctx: XdpContext) -> u32 {
    match try_demo(ctx) {
        Ok(ret) => ret,
        Err(action) => action,
    }
}

static mut BACKEND_PORTS: HashMap<u16, BackendPorts> = HashMap::with_max_entries(10, 0);

fn try_demo(ctx: XdpContext) -> Result<u32, u32> {
    info!(&ctx, "received a packet");
    let eth = ptr_at::<ethhdr>(&ctx, 0).ok_or(xdp_action::XDP_PASS)?;

    if unsafe { u16::from_be((*eth).h_proto) } != ETH_P_IP {
        return Ok(xdp_action::XDP_PASS);
    }

    let ip = ptr_at::<iphdr>(&ctx, ETH_HDR_LEN).ok_or(xdp_action::XDP_PASS)?;

    if unsafe { (*ip).protocol } != IPPROTO_UDP {
        return Ok(xdp_action::XDP_PASS);
    }

    info!(&ctx, "recieved a UDP packet");

    let udp = ptr_at::<udphdr>(&ctx, ETH_HDR_LEN + IP_HDR_LEN).ok_or(xdp_action::XDP_PASS)?;

    let destination_port = unsafe { u16::from_be((*udp).dest) };
    if destination_port == 9875 {
        info!(&ctx, "recieved UDP on port 9875");
    }

    Ok(xdp_action::XDP_PASS)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
