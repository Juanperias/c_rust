# C rust

experimental macro that run a combination of C and rust program, example:
```rust
fn other_rust_fn() -> u64 {
    10
}

c_rust! {
    int c_main() {
        int a = other_rust_fn() + 2;
        return a;
    }
}
```
