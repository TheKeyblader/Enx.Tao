use enx_tao_ffi::ffi_inventory;
use interoptopus_csharp::config::*;
use interoptopus_csharp::lang::meta::FileEmission;
use interoptopus_csharp::output::Target;
use interoptopus_csharp::{RustLibrary, dispatch::Dispatch};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    RustLibrary::builder(ffi_inventory())
        .dll_name("tao")
        .dispatch(Dispatch::custom(|item, _| match item.emission {
            FileEmission::Common => Target::new("Interop.Common.cs", "Enx.Tao.Internal"),
            FileEmission::Default => Target::new("Interop.cs", "Enx.Tao.Internal"),
            FileEmission::CustomModule(_) => Target::new("Interop.cs", "Enx.Tao.Internal"),
        }))
        .header_config(HeaderConfig { emit_version: true })
        .build()
        .process()?
        .write_buffers_to("bindings/")?;

    Ok(())
}
