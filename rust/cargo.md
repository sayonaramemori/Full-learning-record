### Installation  
- Installed with Rust already  





### Create a Rust project  
- `cargo new proj --[bin|lib]`

### Cargo.toml versus Cargo.lock 
- `Cargo.toml` describe your dependencies in a broad sense and is written by you.  
- `Cargo.lock` contains exact (version) information about your dependencies. It is maintained by Cargo and should not be manually edited.
```shell
cargo update         # updates all dependencies
cargo update regex   # updates just “regex”
```

### Cargo Toml Config  

#### Binaries  
- A binary’s source can be `src/main.rs` and/or stored in the `src/bin/` directory. For src/main.rs, the default binary name is the package name.
```toml
[[bin]]
name = 'target'
test = false
bench = false

[[bin]]
name = 'another'
```

#### Workspaces  
```toml
[workspace]
members = ["member1", "path/to/member2", "crates/*"]
exclude = ["crates/foo", "path/to/other"]

[package]
name = "hello_world" # the name of the package
version = "0.1.0"    # the current version, obeying semver
authors = ["Alice <a@example.com>", "Bob <b@example.com>"]
```



