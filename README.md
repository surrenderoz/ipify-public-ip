# ipfy_public_ip #

A tiny crate providing the `find_ip` function, which will return the public ip of the request.


## Documentation ##

this is simple to use tiny package.
API documentation is [here](https://docs.rs/ipfy_public_ip).

## Installation ##

In `Cargo.toml`:

```toml
[dependencies]
ipfy-public-ip = "0.1.1"

```

And in your crate root:

```rust
use ipfy_public_ip;
```

## Example ##
```
use ipfy_public_ip;

let res = ipfy_public_ip::find_ip();
println!("{}", res);
```