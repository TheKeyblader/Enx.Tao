use interoptopus::{guard, inventory::RustInventory, pattern::guard::Version};

pub mod event;
pub mod keyboard;

pub fn api_guard() -> Version {
    crate::ffi_inventory().into()
}

pub fn ffi_inventory() -> RustInventory {
    RustInventory::new().register(guard!(api_guard)).validate()
}
