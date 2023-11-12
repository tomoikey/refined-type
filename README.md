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

fn main() {
    let rule = MinMaxU8Rule::new(1, 6).unwrap();

    let five = Refined::new(5u8, rule);
    assert!(five.is_ok());

    let eight = Refined::new(8u8, rule);
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
use refined_type::rule::Rule;
use refined_type::result::{Error, Result};
use refined_type::Refined;

struct BiggerRule {
    than: u32
}

impl BiggerRule {
    pub fn new(than: u32) -> Self {
        Self { than }
    }
}

impl Rule for BiggerRule {
    type Item = u32;
    fn validate(&self, target: Self::Item) -> Result<Self::Item> {
        if target > self.than {
            Ok(target)
        }
        else {
            Err(Error::new(
                format!("{} is not bigger than {}", target, self.than)
            ))
        }
    }
}

fn main() {
    let bigger_than_five_rule = BiggerRule::new(5);

    let bigger_than_five_result_ok = Refined::new(7, &bigger_than_five_rule);
    let bigger_than_five_result_err = Refined::new(3, &bigger_than_five_rule);

    assert!(bigger_than_five_result_ok.is_ok());
    assert!(bigger_than_five_result_err.is_err());
}
```

# Compose Rules
As mentioned earlier, it is possible to combine any rules as long as the target types match. 
In the example below, there are standalone rules for 'strings containing Hello' and 'strings containing World.' 
Since their target type is String, combining them is possible. 
I have prepared something called RuleBinder. 
By using RuleBinder, composite rules can be easily created.

```rust
use refined_type::rule::{Rule, RuleBinder};
use refined_type::result::{Error, Result};
use refined_type::Refined;

struct ContainsHelloRule;
struct ContainsWorldRule;

impl Rule for ContainsHelloRule {
    type Item = String;

    fn validate(&self, target: Self::Item) -> Result<Self::Item> {
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

    fn validate(&self, target: Self::Item) -> Result<Self::Item> {
        if target.contains("World") {
            Ok(target)
        }
        else {
            Err(Error::new(format!("{} does not contain `World`", target)))
        }
    }
}

fn main() {
    let contains_hello_and_world_rule = RuleBinder::bind(ContainsHelloRule, ContainsWorldRule);

    let contains_hello_and_world_result_ok = Refined::new("Hello! World!".to_string(), &contains_hello_and_world_rule);
    assert!(contains_hello_and_world_result_ok.is_ok());

    let contains_hello_and_world_result_err = Refined::new("Hello, world!".to_string(), &contains_hello_and_world_rule);
    assert!(contains_hello_and_world_result_err.is_err());
}
```

# Tips
Directly writing `RuleBinder` or `Refined` can often lead to a decrease in readability. 
Therefore, using **type aliases** can help make your code clearer.

```rust
type ContainsHelloAndWorldRule = RuleBinder<ContainsHelloRule, ContainsWorldRule>;

type ContainsHelloAndWorld = Refined<ContainsHelloAndWorldRule, String>;
```

# License
MIT License

Copyright (c) 2023 Tomoki Someya

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