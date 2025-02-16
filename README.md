<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Axum Application.

This is an application of [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) Rust tool using [Axum](https://github.com/tokio-rs/axum) web server.

## Prerequisites:

1. Installation of [Nix](https://nixos.org/download/) package manager.

2. Clone the repo:
```
git clone https://github.com/rottegher/farmtasker-au
cd farmtasker-au
```

## Activating dev environment:

1. Using [direnv](https://direnv.net/):
```bash
direnv allow
```

2. Using just [Nix](https://nixos.org/download):
```
nix develop . --impure
```

Wait for a while for nix to download the rust toolchain and all dependencies to then automatically enable the dev enviroment.

## Running the project:

2. Export your test [Stripe API key](https://dashboard.stripe.com/test/apikeys) to the dev environment:
```bash
export STRIPE_KEY "pk_test_***************************************************************************************************"
```

2. Run and watch the application in dev mode:
```bash
cargo leptos watch
```

## Compiling for Release:
```bash
cargo leptos build --release
```

This will generate your server binary in target/release and your site package in target/site.

## Executing a Server on a Remote Machine Without the Toolchain.

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`
3. Stripe api key.

Copy these files to your remote server. The directory structure should be:
```text
farmtasker-au
site/
```
Set the following environment variables (updating for your project as needed):
```text
LEPTOS_OUTPUT_NAME="farmtasker-au"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
STRIPE_KEY="...your key here..."
```

Finally, run the server binary.
```
./farmtasker-au
```

## Testing the project:
```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.  
Tests are located in end2end/tests directory.

## License:
```txt
                  GNU LESSER GENERAL PUBLIC LICENSE
                       Version 2.1, February 1999

farmtasker.au a marketplace website for local farmers in Tasmania.
Copyright (C) 2024 Dmytro Serdiukov & FARMTASKER PTY LTD

NOTE: All images, logos, and branding are the exclusive property of FARMTASKER PTY LTD and are not covered by the open-source license.
These assets may not be used publically without prior written permission.

This software is a free software; you can redistribute it and/or
modify it under the terms of the GNU Lesser General Public
License as published by the Free Software Foundation; either
version 2.1 of the License, or (at your option) any later version.

This software is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

See the GNU Lesser General Public License for more details.

FARMTASKER PDY LTD, hereby disclaims all copyright interest in the
software `farmtasker.au' (a marketplace website for local farmers in Tasmania) written by Dmytro Serdiukov.

11 December 2024
Olesia Trukhanska, founder of FARMTASKER PTY LTD

You can contact us at info@farmtasker.au
```
See `LICENSE` file or [GNU LGPL](https://www.gnu.org/licenses/old-licenses/lgpl-2.1.html).



