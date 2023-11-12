# Refined-Type

**refined-type** is a library developed for Rust. It enhances your types, making them more robust and expanding the range of guarantees your applications can statically ensure.

# Overview
You can create various rules for a certain type, such as phone numbers, addresses, times, and so on. 
Once you have established the rules, you can easily combine them. 
Specifically, if you create rules for 'non-empty strings' and 'strings composed only of alphabets,' you do not need to redefine a new rule for 'non-empty strings composed only of alphabets'. 
All rules can be arbitrarily combined and extended as long as the target type matches. Enjoy a wonderful type life!


# Example Usage
To use this library, first, create a rule instance (e.g., MinMaxU8Rule), and then use it to refine your values with the Refined type. Check if the refined value is within the specified range using the is_ok() and is_err() methods.

Feel free to explore more rules and integrate them into your Rust projects to enhance type safety.
```rust
use refined_type::rule::MinMaxU8Rule;
use refined_type::Refined;

let rule = MinMaxU8Rule::new(1, 6).unwrap();

let five: Refined<MinMaxU8Rule, u8> = Refined::new(5u8, rule);
assert!(five.is_ok());

let eight: Refined<MinMaxU8Rule, u8> = Refined::new(8u8, rule);
assert!(eight.is_err());
```

# Installation
### (まだインストールできないヨ💦😅 ごめんネ❗️)
To use refined-type in your Rust project, add the following to your Cargo.toml file

```toml
[dependencies]
refined-type = "?.?.?" # coming soon...
```
