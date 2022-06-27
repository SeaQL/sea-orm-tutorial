# Create a Rocket project

Create a new binary crate:

```sh
$ cargo new rocket-example --bin
$ cd rocket-example
```

Add [`rocket`](https://crates.io/crates/rocket) as a dependency:

```diff
# Cargo.toml

...

[dependencies]
+ rocket = { version = "^0.5.0-rc.2", features = ["json"] }

...

```

*Pay attention to the version that you're using. `rocket` and/or its dependencies may not compile on the stable build of the Rust compiler if an early version of `rocket` is used.*

The following should compile and run:

```rust, no_run
// src/main.rs

use rocket::*;

#[get("/")]
async fn index() -> &'static str {
    "Hello, bakeries!"
}

#[launch] // The "main" function of the program
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

```

To verify it works:

```sh
$ cargo run
```

```
GET localhost:8000/

"Hello, bakeries!"
```
