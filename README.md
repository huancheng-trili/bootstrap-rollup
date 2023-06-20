# Smart Rollups Onboarding Tutorial

- `src/lib.rs` -- contains the `Rust` code for our "Hello, World" kernel
- `Cargo.toml` -- has the necessary dependencies for the building process
- `empty_input.json` -- empty example of an input (for debugging purposes)
- `sandbox_node.sh` -- script for setting up the sandboxed mode binaries
- `two_inputs.json` -- example of an input with two messages (for debugging purposes)

This tutorial will explain in detail the necessary steps for setting up a **smart rollup** on a test network for the Tezos blockchain. A valuable resource for learning about this exciting new feature can be found by following the [GitLab documentation](https://tezos.gitlab.io/alpha/smart_rollups.html).

This is an attempt to collect information from multiple resources (provided at the end of this tutorial) to ensure a smooth onboarding experience. However, a basic familiarity with blockchain terms and practices is assumed.

## 1. Introduction to Smart Rollups

**Smart rollups** are an elegant solution for **horizontally scaling** the Tezos blockchain, which involves distributing the workload of the main layer (Layer 1) to external layers that perform their tasks "off-chain". In comparison, **vertical scaling** focuses on optimizing the main layer itself but is less scalable than the former.

Let us use an analogy: think of a company with N employees who receive increasing amounts of work every day. Eventually, the team becomes overwhelmed. While hiring more people for the team is an option, it becomes challenging for them to coordinate, which ultimately reduces their productivity.

**Vertical scaling** in this scenario means providing better working equipment to increase productivity. On the other hand, **horizontal scaling** involves creating external teams that work on specific portions of the workload, reducing the need for extensive interaction with the initial team. The latter option is more scalable because one can continuously improve the equipment up to a certain point, while creating external teams can happen at any time and will always be beneficial. In our case, these external teams are the **smart rollups**.

## 2. The Kernel

### 2.1. Definition

The core component of any smart rollup is the **kernel**. A kernel is a 32-bit `WebAssembly` (`WASM`) program responsible for managing input messages, updating the state of the rollup, and determining when to output messages to Layer 1. To continue with the analogy, the kernel represents the work ethic of the "external team".

### 2.2. `Rust`

In this tutorial, `Rust` is used as the programming language for the kernel due to its excellent support for `WASM`. However, any programming language that has `WASM` compilation support could be used.

Prerequisites for developing kernels are `cargo` and a `Rust` compiler with `WebAssembly` support (e.g. `wasm32-unknown-unknown` target).

We propose using `rustup` for this purpose, by following this [installation tutorial](https://www.rust-lang.org/tools/install).

:information_source: In case you have an outdated version, you might encounter issues. In this case, we encourage you to do:

```bash!
rustup override set 1.66
```

With `rustup`, one can enable `WASM` as a compilation target using the following:

```bash!
rustup target add wasm32-unknown-unknown
```

### 2.3. `Clang` + `LLVM`

We need `Clang` for compilation to `WebAssembly`. At minimum version `11` is required. Here are some suggested ways to achieve that, depending on your OS:

```bash!
# With Homebrew
$ brew install llvm
$ export CC="$(brew --prefix llvm)/bin/clang"

# On Ubuntu
$ sudo apt-get install clang-11
$ export CC=clang-11

# On Fedora
$ dnf install clang
$ export CC=clang

# On Arch Linux
$ pacman -S clang
$ export CC=clang
```

We do the `export CC` because there are systems, such as various Linux distributions, that don't ship with `Clang` as their default `C/C++` compiler.

Check that at least version `11` is installed with `$CC --version`.

Also, ensure that the `clang` you've installed supports the `wasm32` target with:

```bash!
$ $CC -print-targets | grep WebAssembly
#     wasm32      - WebAssembly 32-bit
#     wasm64      - WebAssembly 64-bit
```

#### `AR` on macOS

To compile to `WebAssembly` on macOS, you need to use the `LLVM` archiver. If you've used `Homebrew` to install `LLVM`, you can configure it with the following:

```bash!
export AR="$(brew --prefix llvm)/bin/llvm-ar"
```

### 2.4. WebAssembly Toolkit

During development, having the [`WebAssembly Toolkit` (`wabt`)](https://github.com/WebAssembly/wabt) available is useful. It provides tooling for stripping `WebAssembly` binaries (`wasm-strip`) and conversion utilities between the textual and binary representation of `WebAssembly` (`wat2wasm`, `wasm2wat`).

Most distributions ship a `wabt` package which you can install using:

```bash!
# With Homebrew
$ brew install wabt

# On Ubuntu
$ sudo apt install wabt

# On Fedora
$ dnf install wabt

# On Arch Linux
$ pacman -S wabt
```

Then, check that the `wasm-strip` version is at least `1.0.31`. If not, you can download it directly from [here](https://github.com/WebAssembly/wabt/releases/tag/1.0.31), extract files, and then whenever you have to use `wasm-strip`, you can use `.<path_to_wabt_1.0.31>/bin/wasm-strip`, instead.

### 2.5. "Hello, World!" Kernel

To get started, we've prepared a [repository](https://gitlab.com/trili/hello-world-kernel) that helps you get started with kernel development quickly.

You can clone the repository as such:

```bash!
git clone https://gitlab.com/trili/hello-world-kernel.git
cd hello-world-kernel/
```

You can immediately build using:

```bash!
cargo build --target wasm32-unknown-unknown
```

After building it, you should be able to inspect the produced artifacts.

```bash!
$ ls -1 target/wasm32-unknown-unknown/debug
# build
# deps
# examples
# hello_world_kernel.d
# hello_world_kernel.wasm
# incremental
# libhello_world_kernel.d
# libhello_world_kernel.rlib
```

The most important item is the `hello_world_kernel.wasm` which is our readily compiled kernel.

## 3. Getting `Octez`

You need the `Octez` binaries to test locally and deploy a Smart Rollup kernel.

`Octez` is distributed in multiple ways. Most convenient to you may be these:

- Container Images ([`Docker`](https://hub.docker.com/r/tezos/tezos/) or [`Podman`](https://podman.io/))
- `OPAM` (`opam install octez`)
- Nix Shell

### 3.1. Container Images

You have the option to install one of the popular tools for interacting with container images:

- [Docker](https://www.docker.com/)
- [Podman](https://podman.io/)

For this tutorial, we assume you have installed `Docker`. However, you can easily adapt the instructions by replacing `docker` with `podman`.

The [Octez container images](https://hub.docker.com/r/tezos/tezos/) are automatically generated from the [Tezos GitLab repository](https://gitlab.com/tezos/tezos), ensuring that you can always access the latest version of the `Octez` binaries.

To obtain the most recent image from our repository, execute the following command:

```bash!
docker pull tezos/tezos:master
```

Now, you can initiate an interactive (`-it`) session with `Docker` based on that image, which allows access to the kernel files created as part of this tutorial. To achieve this, you must mount the current directory within the container using the [`--volume`](https://docs.docker.com/storage/bind-mounts/) argument. Run the following command within the <kbd>"Hello, World!" kernel</kbd> directory:

```bash!
docker run -it --volume $(pwd):/home/tezos/hello-world-kernel --entrypoint /bin/sh --name octez-container tezos/tezos:master
```

At this point, you should observe that the <kbd>"Hello, World!" kernel</kbd> directory is accessible and contains the kernel files previously created.

```bash!
$ ls -1 hello-world-kernel
# same contents as in the repository
```

At this stage, you can verify that the container image includes all the required executables:

```bash!
$ octez-node --version
# 6fb8d651 (2023-06-05 12:05:17 +0000) (0.0+dev)
$ octez-smart-rollup-wasm-debugger --version
# 6fb8d651 (2023-06-05 12:05:17 +0000) (0.0+dev)
$ octez-smart-rollup-node-alpha --version
# 6fb8d651 (2023-06-05 12:05:17 +0000) (0.0+dev)
$ octez-client --version
# 6fb8d651 (2023-06-05 12:05:17 +0000) (0.0+dev)
```

Please note that the version number mentioned may not precisely match the version you have locally, as the container images are periodically updated.

### 3.2. OPAM

:warning: We suggest utilizing this approach only if you are already familiar with `OPAM`.

If you're acquainted with the `opam` tool, you might prefer to install `Octez` into your `OPAM` switch:

```bash!
opam install octez
```

For a more comprehensive installation guide via `OPAM`, please refer to this [guide](https://tezos.gitlab.io/introduction/howtoget.html#building-from-sources-via-opam).

### 3.3. Nix Shell

Another way to bring in the `Octez` binaries is through a Nix shell.

The Nix website provides some information on how to install the Nix package manager [here](https://nixos.org/download.html).

Once you have Nix installed, you can simply drop into a Nix shell like so:

```bash!
$ nix-shell -p 'import (builtins.fetchTarball "https://gitlab.com/tezos/tezos/-/archive/master/tezos-master.zip")'
[nix-shell:~]$ # You're in a Nix shell! 
```

For `x86_64-linux` systems, there is a binary cache available. Add this to your `nix.conf` to activate it:

```bash!
extra-trusted-public-keys = nix.cache.hwlium.com:M57rk9haJRNFiNUA+6sF6ogbIVg4k8XrKpf5QSohBEA= nix.cache.hwlium.com-2:mFFtk/Pvh/mrCJ7DHOY9mf769A/Nth97WFXMPMy6BGw=
extra-substituters = https://nix.cache.hwlium.com
```

## 4. Processing the Kernel

### 4.1. Debugging the Kernel

Before originating a rollup, it can be helpful to observe the behavior of its kernel. To facilitate this, there is a dedicated `Octez` binary called `octez-smart-rollup-wasm-debugger`.
However, before using it, it is important to understand how the rollup receives its inputs. Each block at every level of the blockchain has a specific section dedicated to the (shared and unique) **smart rollup inbox**. Consequently, the inputs of a rollup can be seen as a list of inboxes for each level, or more precisely, a list of lists.
Let us start from a trivial inbox, which is stored in the `empty_input.json` file. We can debug the <kbd>"Hello, World!" kernel</kbd> with:

```bash!
cd hello-world-kernel

octez-smart-rollup-wasm-debugger target/wasm32-unknown-unknown/debug/hello_world_kernel.wasm --inputs empty_input.json
```

Now you are in **debugging** mode, which is very well documented and explained in the [documentation](https://tezos.gitlab.io/alpha/smart_rollups.html#testing-your-kernel). Similar to how the rollup awaits internal messages from Layer 1 or external sources, the debugger also waits for inputs.

Once we're in the debugger REPL (read–eval–print loop), you can run the kernel for one level using the `step inbox` command:

```bash!
> step inbox
# Loaded 0 inputs at level 0
# Hello, kernel!
# Got message: Internal(StartOfLevel)!
# Got message: Internal(InfoPerLevel(InfoPerLevel { predecessor_timestamp: 1970-01-01T00:00:00Z, predecessor: BlockHash("BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M") }))!
# Got message: Internal(EndOfLevel)!
# Evaluation took 11000000000 ticks so far
# Status: Waiting for input
# Internal_status: Collect
```

Let us explain what our kernel is supposed to do:

- whenever it receives an input, it prints the `"Hello, kernel!"` message.
- whenever there is a message in the input, it prints it, because of the `handle_message` function.

It is important to understand that the **shared rollup inbox** has at each level at least the following **internal** messages:

- <kbd>StartOfLevel</kbd> -- marks the beginning of the inbox level, does not have any payload.
- <kbd>InfoPerLevel</kbd> -- provides the timestamp and block hash of the predecessor of the current Tezos block as payload.
- <kbd>EndOfLevel</kbd> -- pushed after the application of the operations of the Tezos block, does not have any payload.

You will notice that the behavior aligns with the expectations. You can also experiment with a non-empty input, such as `two_inputs.json`:

```bash!
$ octez-smart-rollup-wasm-debugger target/wasm32-unknown-unknown/debug/hello_world_kernel.wasm --inputs two_inputs.json
> step inbox
# Loaded 2 inputs at level 0
# Hello, kernel!
# Got message: Internal(StartOfLevel)
# Got message: Internal(InfoPerLevel(InfoPerLevel { predecessor_timestamp: 1970-01-01T00:00:00Z, predecessor: BlockHash("BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M") }))
# Got message: External([26, 84, 104, 105, 115, 32, 109, 101, 115, 115, 97, 103, 101, 32, 105, 115, 32, 102, 111, 114, 32, 109, 101])
# Got message: External([5, 84, 104, 105, 115, 32, 111, 110, 101, 32, 105, 115, 110, 39, 116])
# Got message: Internal(EndOfLevel)
# Evaluation took 11000000000 ticks so far
# Status: Waiting for input
# Internal_status: Collect
```

As expected, the two messages from the input are also displayed as debug messages.
Feel free to explore additional examples from the dedicated [kernel gallery](https://gitlab.com/tezos/kernel-gallery/-/tree/main/) or create your own!

### 4.2. Reducing the Size of the Kernel

The origination process is similar to that of smart contracts. To originate a smart rollup, we have to consider the size of the kernel that shall be deployed. The size of the kernel needs to be smaller than the manager operation size limit.

Regrettably, the size of the `.wasm` file is currently too large:

```bash!
$ du -h target/wasm32-unknown-unknown/debug/hello_world_kernel.wasm 
# 17.3M target/wasm32-unknown-unknown/debug/hello_world_kernel.wasm
```

To address this, we can use `wasm-strip`, a tool designed to reduce the size of kernels. It accomplishes this by removing unused parts of the `WebAssembly` module (e.g. dead code), which are not required for the execution of the rollups.

```bash!
$ wasm-strip target/wasm32-unknown-unknown/debug/hello_world_kernel.wasm 

$ du -h target/wasm32-unknown-unknown/debug/hello_world_kernel.wasm
# 532.0K target/wasm32-unknown-unknown/debug/hello_world_kernel.wasm
```

:information_source: If you did not install `wabt` in your `Docker` session, you can open another terminal session, navigate to the <kbd>"Hello, World!" kernel </kbd> directory, and do this there. The modifications will get propagated to the interactive `Docker` session thanks to the `--volume` command option.

Undoubtedly, this process has effectively reduced the size of the kernel. However, there is still additional work required to ensure compliance with the manager operation size limit.

### 4.3. The Installer Kernel

Instead of using a kernel file for origination in the aforementioned format, an alternative approach is to utilize the **installer** version of the kernel. This **installer kernel** can be **upgraded** to the original version if provided with additional information, in the form of **preimages** which can be provided to the rollup node later on as part of its **reveal data channel**.

There are two ways to communicate with smart rollups:

1. **global inbox** -- allows Layer 1 to transmit information to all rollups. This unique inbox contains two kinds of messages: **external messages** are pushed through a Layer 1 manager operation, while **internal messages** are pushed by Layer 1 smart contracts or the protocol itself (e.g. `StartOfLevel`, `InfoPerLevel`, `EndOfLevel`).
2. **reveal data channel** -- allows the rollup to retrieve data (e.g. **preimages**) coming from data sources external to Layer 1.

The main benefit of the installer kernel is that it is small enough to be used in origination regardless of the kernel that it shall be upgraded to.

There is an [installer kernel origination topic](https://tezos.stackexchange.com/questions/4784/how-to-originating-a-smart-rollup-with-an-installer-kernel/5794#5794) for this, please consult it for further clarifications. To generate the **installer kernel**, the `smart-rollup-installer` tool is required:

```bash!
cargo install tezos-smart-rollup-installer
```

To create the installer kernel from the initial kernel:

```bash!
smart-rollup-installer get-reveal-installer --upgrade-to target/wasm32-unknown-unknown/debug/hello_world_kernel.wasm --output hello_world_kernel_installer.hex --preimages-dir preimages/
```

This command creates the following:

- <kbd>hello_world_kernel_installer.hex</kbd> -- the hexadecimal representation of the installer kernel to be used in the origination.
- <kbd>preimages/</kbd> -- a directory containing the preimages necessary for upgrading from the installer kernel to the original kernel. These preimages are transmitted to the rollup node that runs the installer kernel with the help of the [**reveal data channel**](https://tezos.gitlab.io/alpha/smart_rollups.html#reveal-data-channel).

Notice the reduced dimensions of the installer kernel:

```bash!
$ du -h hello_world_kernel_installer.hex
# 36.0K hello_world_kernel_installer.hex
```

Because of the dimension of this installer kernel, you are now ready for deployment.

Note that this shows the size of the hex encoded file, which is larger than the actual binary size of the kernel that we originate.

## 5. Deploying the Kernel

### 5.1. Sandboxed Mode

Our goal now is to create a testing environment for originating our rollup with the created kernel. In the `hello-world-kernel` repository, we offer the <kbd>sandbox-node.sh</kbd> file, which does the following:

- configures the `Octez` node to operate in [**sandbox mode**](https://tezos.gitlab.io/user/sandbox.html).
- activates the `alpha` protocol, by using an `activator` account.
- creates 5 test (bootstrapping) accounts used for manual [**baking**](https://opentezos.com/baking/cli-baker/).
- creates a loop of continuous baking.

Run the file with:

```bash!
./sandbox_node.sh
```

Ignore the "Unable to connect to the node" error, as it only comes one time because the `octez-client` command was used while the node was not yet bootstrapped.

Leave that process running. Open a new `Docker` session, which works in the same container named `octez-container`:

```bash!
docker exec -it octez-container /bin/sh
```

To check that the network has the correctly configured protocol:

```bash!
$ octez-client rpc get /chains/main/blocks/head/metadata | grep protocol

# "protocol": "ProtoALphaALphaALphaALphaALphaALphaALphaALphaDdp3zK",
# "next_protocol": "ProtoALphaALphaALphaALphaALphaALphaALphaALphaDdp3zK"
```

You are now ready for the Smart Rollup origination process.

### 5.2. Smart Rollup Origination

To originate a smart rollup using the `hello_world_kernel_installer` created above:

```bash!
$ octez-client originate smart rollup "test_smart_rollup" from "bootstrap1" of kind wasm_2_0_0 of type bytes with kernel file:hello-world-kernel/hello_world_kernel_installer.hex --burn-cap 3
     
# > Node is bootstrapped.
# ...
# Smart rollup sr1B8HjmEaQ1sawZtnPU3YNEkYZavkv54M4z memorized as "test_smart_rollup"
```

In the command above, the `--burn-cap` option specifies the amount of ꜩ you are willing to "burn" (lose) to allocate storage in the global context of the blockchain for each rollup.

To run a rollup node for the rollup using the installer kernel, you need to copy the contents of the preimages directory to `${ROLLUP_NODE_DIR}/wasm_2_0_0/`. You can set `$ROLLUP_NODE_DIR` to `~/.tezos-rollup-node`, for instance:

```bash!
mkdir -p ~/.tezos-rollup-node/wasm_2_0_0

cp hello-world-kernel/preimages/* ~/.tezos-rollup-node/wasm_2_0_0/
```

You should now be able to **run** your rollup node:

```bash!
octez-smart-rollup-node-alpha run operator for "test_smart_rollup" with operators "bootstrap2" --data-dir ~/.tezos-rollup-node/ --log-kernel-debug --log-kernel-debug-file hello_kernel.debug
```

Leave this running as well, and open another `Docker` session, as already explained, with the `octez-container`.

Each time a block is baked, a new "Hello, kernel!" message should appear in the `hello_kernel.debug` file:

```bash!
$ tail -f hello_kernel.debug 
# Hello, kernel!
# Got message: Internal(StartOfLevel)
# Got message: Internal(InfoPerLevel(InfoPerLevel { predecessor_timestamp: 2023-06-07T15:31:09Z, predecessor: BlockHash("BLQucC2rFyNhoeW4tuh1zS1g6H6ukzs2DQDUYArWNALGr6g2Jdq") }))
# Got message: Internal(EndOfLevel)
# ... (repeats)
```

Finally, you have successfully deployed a very basic yet functional smart rollup.

### 5.3. Sending an Inbox Message to the Smart Rollup

We now want to send an external message into the rollup inbox, which should be read by our kernel and sent as a debug message. First, we will wait for it to appear using:

```bash!
tail -f hello_kernel.debug | grep External
```

Open yet another `Docker` session and send an external message into the rollup inbox, you can utilize the `Octez` client:

```bash!
octez-client send smart rollup message '[ "test" ]' from "bootstrap3"
```

Once you send the Smart Rollup message, you will notice that in the debug trace, you get:

```bash!
Got message: External([116, 101, 115, 116])
```

`116, 101, 115, 116` represent the bytes of "test".

### 5.4. Test Networks

In the above section, we proposed how to create your `Octez` binaries in **sandbox mode**. Here, we propose a different approach to that, using [test networks](https://teztnets.xyz/). We encourage the reader to try at least one of the following linked tutorials:

- [Ghostnet](https://teztnets.xyz/ghostnet-about) -- uses the protocol that `Mainnet` follows as well.
- [Nairobinet](https://teztnets.xyz/nairobinet-about) -- uses the `Nairobi` protocol.
- [Mondaynet](https://teztnets.xyz/mondaynet-about) -- uses the `alpha` protocol and resets every Monday.

The workflow should be similar to the one presented for the sandbox mode:

- **configure** the network;
- run a node (needs to synchronize with the network -- can make use of [snapshots](https://tezos.gitlab.io/user/snapshots.html));
- create test accounts (which should be funded by the appropriate **Faucet**);
- originate the rollup;
- run the rollup node;
- check the debug file.

## 6. Further References & Documentation

1. [Smart Rollup Documentation](https://tezos.gitlab.io/alpha/smart_rollups.html)
2. [Smart Rollup Kernel SDK Tutorial](https://gitlab.com/tezos/tezos/-/tree/master/src/kernel_sdk)
3. [Smart Rollup Kernel Examples](https://gitlab.com/tezos/kernel-gallery/-/tree/main/)
4. [Ghostnet Indexer](https://ghost.tzstats.com/)
5. [Blockchain Explorer](https://ghostnet.tzkt.io/)
6. [Tezos Smart Rollups Resources](https://airtable.com/shrvwpb63rhHMiDg9/tbl2GNV1AZL4dkGgq)
7. [Tezos Testnets](https://teztnets.xyz/)
8. [Origination of Installer Kernel](https://tezos.stackexchange.com/questions/4784/how-to-originating-a-smart-rollup-with-an-installer-kernel/5794#5794)
9. [Docker Documentation](https://docs.docker.com/get-started/)
