# monadic-traits


A lightweight functional programming toolkit for Rust nightly, mainly for experimenting, providing:


- **Functor**, **Applicative**, and **Monad** traits
- A custom `Maybe<T>` type
- Full nightly `?` operator support via `Try` and `FromResidual`
- Examples and tests


## 🚀 Nightly Requirement
This crate relies on the following nightly features:
- `try_trait_v2`
- `control_flow_enum`
- `never_type`


Install nightly:
```bash
rustup override set nightly
```