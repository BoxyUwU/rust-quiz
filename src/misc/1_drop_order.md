<!-- idea by orlp -->

```rust
((a, b, c, d), panic()); // Drops in order a, b, c, d.
((a, b, c, d, panic())); // Drops in order d, c, b, a.
```