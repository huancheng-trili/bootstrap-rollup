use tezos_smart_rollup::inbox::InboxMessage;
use tezos_smart_rollup::kernel_entry;
use tezos_smart_rollup::michelson::MichelsonBytes;
use tezos_smart_rollup::prelude::*;

kernel_entry!(hello_kernel);

fn handle_message(host: &mut impl Runtime, msg: impl AsRef<[u8]>) {
    if let Some((_, msg)) = InboxMessage::<MichelsonBytes>::parse(msg.as_ref()).ok() {
        debug_msg!(host, "Got message: {:?}\n", msg);
    }
}

pub fn hello_kernel(host: &mut impl Runtime) {
    debug_msg!(host, "Hello, kernel!\n");

    while let Some(msg) = host.read_input().unwrap() {
        handle_message(host, msg);
    }
}
