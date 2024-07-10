> [!WARNING]
> This shows that UniFII does **NOT** work with workspace
> when using `newtype` paradigm.

# **NOT** Works

Simple demo of workspace setup with two UniFFI consuming crates:

- `one`
- `two`

```sh
$ cargo tree -i one --format "{lib}"
one
└── two
```

If you try to build it will fail:

```sh
cargo build
```

Results in:

```sh
error[E0277]: the trait bound `One: uniffi::TypeId<UniFfiTag>` is not satisfied
  --> crates/two/src/models/mod.rs:10:10
   |
10 |     one: One,
   |          ^^^ the trait `uniffi::TypeId<UniFfiTag>` is not implemented for `One`
   |
   = help: the trait `uniffi::TypeId<one::UniFfiTag>` is implemented for `One`
   = help: for that trait implementation, expected `one::UniFfiTag`, found `UniFfiTag`

error[E0277]: the trait bound `One: Lower<UniFfiTag>` is not satisfied
  --> crates/two/src/models/mod.rs:10:10
   |
10 |     one: One,
   |          ^^^ the trait `Lower<UniFfiTag>` is not implemented for `One`
   |
   = help: the trait `Lower<one::UniFfiTag>` is implemented for `One`
   = help: for that trait implementation, expected `one::UniFfiTag`, found `UniFfiTag`

error[E0277]: the trait bound `One: Lift<UniFfiTag>` is not satisfied
  --> crates/two/src/models/mod.rs:10:10
   |
10 |     one: One,
   |          ^^^ the trait `Lift<UniFfiTag>` is not implemented for `One`
   |
   = help: the trait `Lift<one::UniFfiTag>` is implemented for `One`
   = help: for that trait implementation, expected `one::UniFfiTag`, found `UniFfiTag`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `two` (lib) due to 3 previous errors
```

> [IMPORTANT]
> An important gotcha: It is only `BetaRecord`, referencing `One` which is broken,
> however, `BetaObject` does compile! Which is also referencing `One`. So it seems
> the `uniffi::Record` macro is broken, but not `uniffi::Object`?

# Design

This crate is identical to _working_ demo [`alpha`](https://github.com/Sajjon/uf-ws-alpha), except that the type `One` in crate `one` is a `newtype` around `bool`, and made UniFFI capable with `uniffi::custom_newtype!` macro.

Exactly like `alpha` demo, the crate `two` has a `udl` file which declares dependency on this external type, like so:

```webudl
[ExternalExport="one"]
typedef extern One;
```

And this does not work. So the behavior differs between a Rust struct which is `uniffi::Record` and a Rust newtype which is `uniffi::custom_newtype!`.
