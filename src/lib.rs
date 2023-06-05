use tezos_smart_rollup::kernel_entry;
use tezos_smart_rollup::prelude::*;

kernel_entry!(hello_kernel);

pub fn hello_kernel(host: &mut impl Runtime) {
    debug_msg!(host, "Hello, kernel!\n");
}
