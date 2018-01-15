# derive\_deref

This crate adds a simple `#[derive(Deref)]` and `#[derive(DerefMut)]`.
It can be used on any struct with exactly one field.
If the type of that field is a reference,
the reference will be returned directly.

# Example

```rust
#[derive(Deref)]
struct MyInt(i32);

assert_eq!(&1, &*MyInt(1));
assert_eq!(&2, &*MyInt(2));

#[derive(Deref)]
struct MyString<'a>(&'a str);

// Note that we deref to &str, not &&str
assert_eq!("foo", &*MyString("foo"));
assert_eq!("bar", &*MyString("bar"));
```

## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

### Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit
for inclusion in the work, as defined in the Apache-2.0 license, shall be
dual-licensed as above, without any additional terms or conditions.
