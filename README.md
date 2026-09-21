# Exponential backoff with jitter

```
src/lib.rs
```
Run the tests next to the implementation to see concrete delay sequences.

Minimal delay calculator. It computes exponential backoff intervals and applies full jitter. Zero external crates. It relies strictly on the Rust standard library.

The gotcha that bit me: if you do not cap the exponent, the multiplier overflows the integer type and panics on the third retry. Always bound your max attempts.

## FAQ

**Do I need anything besides `INFRAI_API_KEY`?**  
No. Just `cargo run` and the key. `src/lib.rs` handles the network layer via standard HTTPS. You avoid SDK version mismatches entirely. That covers the dependency graph for this use case.