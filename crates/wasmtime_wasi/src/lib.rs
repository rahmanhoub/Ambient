//! Custom synchronous implementation of wasmtime_wasi. Disabling Clippy because this isn't really
//! our code, and this is a short-to-medium term solution anyway.
#![allow(clippy::all)]
#![allow(dead_code, unused_imports, unused_variables)]

mod clocks;
mod env;
mod exit;
mod filesystem;
mod io;
mod ip_name_lookup;
mod network;
mod poll;
mod random;
mod stderr;
mod tcp;
mod udp;
pub use wasi_cap_std_sync::WasiCtxBuilder;
pub use wasi_common::{table::Table, WasiCtx};

type HostResult<T, E> = anyhow::Result<Result<T, E>>;

pub mod wasi {
    wasmtime::component::bindgen!({
        path: "wit",
        world: "cli-reactor"
    });
}

pub fn add_to_linker<T: Send>(
    l: &mut wasmtime::component::Linker<T>,
    f: impl (Fn(&mut T) -> &mut WasiCtx) + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    wasi::wall_clock::add_to_linker(l, f)?;
    wasi::monotonic_clock::add_to_linker(l, f)?;
    wasi::timezone::add_to_linker(l, f)?;
    wasi::instance_monotonic_clock::add_to_linker(l, f)?;
    wasi::instance_wall_clock::add_to_linker(l, f)?;
    wasi::filesystem::add_to_linker(l, f)?;
    wasi::stderr::add_to_linker(l, f)?;
    wasi::poll::add_to_linker(l, f)?;
    wasi::streams::add_to_linker(l, f)?;
    wasi::random::add_to_linker(l, f)?;
    wasi::tcp::add_to_linker(l, f)?;
    wasi::udp::add_to_linker(l, f)?;
    wasi::ip_name_lookup::add_to_linker(l, f)?;
    wasi::instance_network::add_to_linker(l, f)?;
    wasi::network::add_to_linker(l, f)?;
    wasi::exit::add_to_linker(l, f)?;
    wasi::environment::add_to_linker(l, f)?;
    wasi::environment_preopens::add_to_linker(l, f)?;
    Ok(())
}
