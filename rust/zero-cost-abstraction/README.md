# Zero Cost Abstraction





# example

```rust
fn add(x: i32) -> i32 {
    x + 2 - 3 * 5 - x
}
```

this function does not use x, but just return -13 whatever x is.
So, compiler would optimize this.

# Tips

how to write output to assenbly code

```bash
cargo rustc -- --emit asm

# for release mode: I recommend this
cargo rustc --release -- --emit asm

```


## materials
* An introduction to structs, traits, and zero-cost abstractions by Tim McLean - Rust KW Meetup
https://www.youtube.com/watch?v=Sn3JklPAVLk&t=237s