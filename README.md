![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Ross-Morgan/repeat-for/.github%2Fworkflows%2Frust.yml?style=for-the-badge)
![Crates.io Version](https://img.shields.io/crates/v/repeat-for?style=for-the-badge)
![Crates.io License](https://img.shields.io/crates/l/repeat-for?style=for-the-badge)
![Crates.io Total Downloads](https://img.shields.io/crates/d/repeat-for?style=for-the-badge)
![Crates.io Size](https://img.shields.io/crates/size/repeat_for?style=for-the-badge)
![docs.rs](https://img.shields.io/docsrs/repeat_for?style=for-the-badge)

# repeat_for

A macro that repeats a block of code while iterating over an array or a range, substituting in values at compile-time.

# Examples

## List

```rust
struct Foo<const N: usize>;

// Type of each substituted variable is inferred, and each may be different
repeat_for!(x in [0, 1, 2] => {
    impl Foo<x> {
        fn bar() -> u8 {
            x
        }
    }
});
```

## Range

```rust
struct Foo<const N: usize>;

// Type of each substituted variable is inferred, and each may be different
repeat_for!(x in (0..=2) => {
    impl Foo<x> {
        fn bar() -> u8 {
            x
        }
    }
});
```

## Expands To

```rust
struct A<const N: usize>;

impl A<1> {
    fn bar() -> u8 {
        1
    }
}

impl A<2> {
    fn bar() -> u8 {
        2
    }
}

impl A<3> {
    fn bar() -> u8 {
        3
    }
}
```
