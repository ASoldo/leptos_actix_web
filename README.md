<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Starter Template

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool.

## Creating your template repo

If you don't have `cargo-leptos` installed you can install it with

```sh
cargo install cargo-leptos --locked
```

Then run

```sh
cargo leptos new --git leptos-rs/start-actix
```

to generate a new project template (you will be prompted to enter a project name).

```sh
cd {projectname}
```

to go to your newly created project.

Of course, you should explore around the project structure, but the best place to start with your application code is in `src/app.rs`.

## Running your project

We are using **cargo-leptos** for building and serving our project.

```sh
tailwindcss -i ./style/main.css -o ./style/output.css --watch
```
Download `TailwindCss CLI` standalone [TailwindCss CLI - Github Releases](https://github.com/tailwindlabs/tailwindcss/releases)

This command watches and generates `output.css` from `main.css`.

Start the Leptos server with:

```sh
cargo leptos watch --hot-reload
```

By default, you can access your local project at [http://localhost:3000](http://localhost:3000)

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future)

## Executing a Server on a Remote Machine Without the Toolchain

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:

```
leptos_start
site/
```

Set the following environment variables (updating for your project as needed):

```sh
export LEPTOS_OUTPUT_NAME="main"
export LEPTOS_SITE_ROOT="target/site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```

Finally, run the server binary.

## Licensing

MIT

