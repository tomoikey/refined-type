# Refined-Type
**refined-type** is a library developed for Rust. It enhances your types, making them more robust and expanding the range of guarantees your applications can statically ensure.

# Overview
You can create various rules for a certain type, such as phone numbers, addresses, times, and so on. 
Once you have established the rules, you can easily combine them. 
Specifically, if you create rules for 'non-empty strings' and 'strings composed only of alphabets,' you do not need to redefine a new rule for 'non-empty strings composed only of alphabets'. 
All rules can be arbitrarily combined and extended as long as the target type matches. Enjoy a wonderful type life!

# Example Usage
```rust
use refined_type::rule::MinMaxU8Rule;
use refined_type::Refined;
use std::ops::Deref;

type NonEmptyString = Refined<NonEmptyStringRule>;

fn main() {
    let hello_world = NonEmptyString::new("hello world".to_string());
    assert_eq!(five.deref(), "hello world");

    let empty_string = NonEmptyString::new("".to_string());
    assert!(eight.is_err());
}
```

# Installation
```shell
cargo add refined-type
```

# Custom Rule
There are many situations where you may want to define custom rules. 
To define rules for a specific target type, you first need to define a struct. 
In the struct, define fields for specifying detailed conditions. 
Once the definition is complete, all you need to do is implement the Rule trait. 
Add your preferred conditions as you like.

```rust
use refined_type::rule::{NonEmptyString, NonEmptyStringRule};
use refined_type::Refined;

fn main() {
    let non_empty_string_result = Refined::<NonEmptyStringRule>::new("Hello World".to_string());
    assert_eq!(non_empty_string_result.unwrap().deref(), "Hello World");

    let empty_string_result = Refined::<NonEmptyStringRule>::new("".to_string());
    assert!(empty_string_result.is_err())   
}
```

# Compose Rules
As mentioned earlier, it is possible to combine any rules as long as the target types match. 
In the example below, there are standalone rules for 'strings containing Hello' and 'strings containing World'. 
Since their target type is String, combining them is possible. 
I have prepared something called Rule Composer (`And`, `Or`, `Not`). 
By using Rule Composer, composite rules can be easily created.

### Original Rules
```rust
use refined_type::result::Error;
use refined_type::rule::Rule;
use refined_type::rule::composer::And;
use refined_type::Refined;

struct ContainsHelloRule;
struct ContainsWorldRule;

impl Rule for ContainsHelloRule {
    type Item = String;

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        if target.contains("Hello") {
            Ok(target)
        }
        else {
            Err(Error::new(format!("{} does not contain `Hello`", target)))
        }
    }
}

impl Rule for ContainsWorldRule {
    type Item = String;

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        if target.contains("World") {
            Ok(target)
        }
        else {
            Err(Error::new(format!("{} does not contain `World`", target)))
        }
    }
}
```

### 1: `And` Rule Composer
`And` Rule Composer is a rule that satisfies both of the two rules. 
It is generally effective when you want to narrow down the condition range.
```rust
fn main() {
    type HelloAndWorldRule = And<ContainsHelloRule, ContainsWorldRule>;

    let rule_ok = Refind::<HelloAndWorldRule>::new("Hello! World!".to_string());
    assert!(rule_ok.is_ok());

    let rule_err = Refined::<HelloAndWorldRule>::new("Hello, world!".to_string());
    assert!(rule_err.is_err());
}
```

### 2: `Or` Rule Composer
`Or` Rule Composer is a rule that satisfies either of the two rules. 
It is generally effective when you want to expand the condition range.
```rust
fn main() {
    type HelloOrWorldRule = Or<ContainsHelloRule, ContainsWorldRule>;

    let rule_ok_1 = Refined::<HelloOrWorldRule>::new("Hello! World!".to_string());
    assert!(rule_ok_1.is_ok());

    let rule_ok_2 = Refined::<HelloOrWorldRule>::new("hello World!".to_string());
    assert!(rule_ok_2.is_ok());

    let rule_err = Refined::<HelloOrWorldRule>::new("hello, world!".to_string());
    assert!(rule_err.is_err());
}
```

### 3: `Not` Rule Composer
`Not` Rule Composer is a rule that does not satisfy a specific condition. 
It is generally effective when you want to discard only certain situations.
```rust
fn main() {
    type NotHelloRule = Not<ContainsHelloRule>;

    let rule_ok = Refined::<NotHelloRule>::new("hello! World!".to_string());
    assert!(rule_ok.is_ok());

    let rule_err = Refined::<NotHelloRule>::new("Hello, World!".to_string());
    assert!(rule_err.is_err());
}
```

### 4: Compose Rule Composer
Rule Composer is also a rule. 
Therefore, it can be treated much like a composite function
```rust
fn main() {
    struct StartWithHelloRule;
    struct StartWithByeRule;

    struct EndWithJohnRule;

    type StartWithHelloOrByeRule = Or<StartWithHelloRule, StartWithByeRule>;
    type GreetingRule = And<StartWithHelloOrByeRule, EndWithJohnRule>;

    type Greeting = Refined<GreetingRule>;

    assert!(GreetingRule::validate("Hello! Nice to meet you John".to_string()).is_ok());
    assert!(Greeting::validate("Bye! Have a good day John".to_string()).is_ok());
    assert!(Greeting::validate("How are you? Have a good day John".to_string()).is_err());
    assert!(Greeting::validate("Bye! Have a good day Tom".to_string()).is_err());
}
```

# Tips
Directly writing `And`, `Or`, `Not` or `Refined` can often lead to a decrease in readability. 
Therefore, using **type aliases** can help make your code clearer.

```rust
type ContainsHelloAndWorldRule = And<ContainsHelloRule, ContainsWorldRule>;

type ContainsHelloAndWorld = Refined<ContainsHelloAndWorldRule>;
```

# License
MIT License

Copyright (c) 2024 Tomoki Someya

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.