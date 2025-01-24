# Tari Exchange

Proof of Concept project on implementing modern DEX on modern [Tari]() Protocol.

## Getting started

### Environment configuration

#### Tari Swarm

To set up the [Tari Swarm](), you will need to run it from the official repos.

* [Tari](https://github.com/tari-project/tari)
* [The Ootle](https://github.com/tari-project/tari-dan)

First, copy `tari` and `tari-dan` projects to a new folder on your `$HOME` and checkout `feature-dan2` branch for [Tari]().

```bash
cd ~/
mkdir tari_swarm
cd tari_swarm
git clone https://github.com/tari-project/tari.git
git clone https://github.com/tari-project/tari-dan.git
cd tari && git checkout feature-dan2 && cd ../
```

Next, change the directory to the [Tari Dan]() folder. You will need to create a config file for the [Tari Swarm](), so run:

```bash
cd tari-dan
cargo run --bin tari_swarm_daemon --release -- -c data/swarm/config.toml init
```

This will create the necessary `config.toml` file.
You should update the number of validator nodes to at least 2.

```toml
[[instances]]
name = "Validator node"
instance_type = "TariValidatorNode"
num_instances = 2
```

Now you will need to run Tari Swarm:

```bash
cargo run --bin tari_swarm_daemon --release -- -c data/swarm/config.toml start
```
In a couple of minutes you should be able to browse to the [Tari Swarm]() by visiting: [http://localhost:8080](http://localhost:8080)

### Component template

We already have [Component Templates] in our repo inside `component_templates` folder. 
We generated them using [Cargo Generate](https://cargo-generate.github.io/cargo-generate/):

```bash
cargo generate https://github.com/tari-project/wasm-template.git wasm_templates/fungible
```

Now we will compile TEX [Component Template]() into WASM module and deploy to our [Validator Nodes]().

```bash
cargo build-wasm -p tex --release
ls target/wasm32-unknown-unknown/release/
```

You should see `tex.wasm` inside `target/wasm32-unknown-unknown/release/` now.

Open [Tari Swarm UI](http://localhost:8080) and scroll down until you see *Templates* section.
Click **Choose file**, select `target/wasm32-unknown-unknown/release/tex.wasm`, click **upload template**.

Open [Tari Validator Node UI](http://localhost:12005/templates) and check that a new Template with *Tex* name exists there.

### Accounts

```bash
cargo run -p scripts --bin create_users
```

### Component instance

Let's instantiate a [Component Instance]() by calling `new` method on a template.

Copy template address (for example `3b5490a65751641b2c3401c6e8fd1cedbe719198c3116bac6f09972fee1075d6`) from
[UI](http://localhost:12005/templates) and paste in `developemnt_tools/scripts/src/bin/crate_component_instances.rs`.

```bash
cargo run -p scripts --bin create_component_instances
```

## Testing

### Script

Let's test our exchange by calling it's methods.


```bash
cargo run -p scripts --bin execute_exchange_scenario
```

## Known issues

### Hickory dependency issues

If facing - add `hickory-proto = { version = "=0.25.0-alpha.2" }` to failing package.
