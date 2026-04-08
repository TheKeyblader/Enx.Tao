use interoptopus::inventory::RustInventory;

pub fn ffi_inventory() -> RustInventory {
    RustInventory::new().validate()
}
