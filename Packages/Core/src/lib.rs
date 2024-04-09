use neon::prelude::*;

mod buffers;
mod client;
mod filters;
mod generators;
mod init;
mod overlays;
mod utils;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("init", init::init)?;
    Ok(())
}
