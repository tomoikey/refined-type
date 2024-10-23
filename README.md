<p align="center">
    <img width=700 src="https://github.com/user-attachments/assets/2ae4bfee-1d42-4ed7-820a-b13260d359ef">
</p>

# Refined Type

**refined_type** is a library developed for Rust. It enhances your types, making them more robust and expanding the
range of guarantees your applications can statically ensure.

You can create various rules for a certain type, such as phone numbers, addresses, times, and so on.
Once you have established the rules, you can easily combine them.
Specifically, if you create rules for 'non-empty strings' and 'strings composed only of alphabets,' you do not need to
redefine a new rule for 'non-empty strings composed only of alphabets'.
All rules can be arbitrarily combined and extended as long as the target type matches. Enjoy a wonderful type life!

# Installation

```shell
cargo add refined_type
```

# Get Started

As an example, let's convert from JSON to a struct.

```rust
// define a struct for converting from JSON.
#[derive(Debug, Deserialize)]
struct Human {
    name: NonEmptyString,
    age: MinMaxU8<18, 80>,
    friends: NonEmptyVec<String>,
}

// In the 1st example, all fields satisfy the rule, causing the conversion from JSON to succeed.
fn get_started_simple_example() -> anyhow::Result<()> {
    let json = json! {{
        "name": "john",
        "age": 20,
        "friends": ["tom", "taro"]
    }}
        .to_string();

    let human = serde_json::from_str::<Human>(&json)?;

    assert_eq!(human.name.into_value(), "john");
    assert_eq!(human.age.into_value(), 20);
    assert_eq!(human.friends.into_value(), vec!["tom", "taro"]);
    Ok(())
}

// In the 2nd example, while `name` does not satisfy the rule, `age` and `friends` do, causing the conversion from JSON to fail.
fn get_started_empty_name_example() -> anyhow::Result<()> {
    let json = json! {{
        "name": "",
        "age": 20,
        "friends": ["tom", "taro"]
    }}
        .to_string();

    // because `name` is empty
    assert!(serde_json::from_str::<Human>(&json).is_err());
    Ok(())
}

// In the 3rd example, while `age` does not satisfy the rule, `name` and `friends` do, causing the conversion from JSON to fail.
fn get_started_outbound_age_example() -> anyhow::Result<()> {
    let json = json! {{
        "name": "john",
        "age": 100,
        "friends": ["tom", "taro"]
    }}
        .to_string();

    // because `age` is not in the range of 18 to 80
    assert!(serde_json::from_str::<Human>(&json).is_err());
    Ok(())
}

// In the 4th example, while `friends` does not satisfy the rule, `name` and `age` do, causing the conversion from JSON to fail.
fn get_started_empty_vec_example() -> anyhow::Result<()> {
    let json = json! {{
        "name": "john",
        "age": 20,
        "friends": []
    }}
        .to_string();

    // because `friends` is empty
    assert!(serde_json::from_str::<Human>(&json).is_err());
    Ok(())
}
```

---

# Compose Rules

As mentioned earlier, it is possible to combine any rules as long as the target types match.
In the example below, there are standalone rules for 'strings containing Hello' and 'strings containing World'.
Since their target type is String, combining them is possible.
I have prepared something called Rule Composer (`And`, `Or`, `Not`).
By using Rule Composer, composite rules can be easily created.

### Original Rules

```rust
struct ContainsHelloRule;

struct ContainsWorldRule;

impl Rule for ContainsHelloRule {
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.contains("Hello") {
            Ok(())
        } else {
            Err(Error::new(format!("{} does not contain `Hello`", target)))
        }
    }
}

impl Rule for ContainsWorldRule {
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.contains("World") {
            Ok(())
        } else {
            Err(Error::new(format!("{} does not contain `World`", target)))
        }
    }
}
```

### 1: `And` Rule Composer

`And` Rule Composer is a rule that satisfies both of the two rules.
It is generally effective when you want to narrow down the condition range.

```rust
fn example_5() {
    type HelloAndWorldRule = And![ContainsHelloRule, ContainsWorldRule];

    let rule_ok = Refined::<HelloAndWorldRule>::new("Hello! World!".to_string());
    assert!(rule_ok.is_ok());

    let rule_err = Refined::<HelloAndWorldRule>::new("Hello, world!".to_string());
    assert!(rule_err.is_err());
}
```

### 2: `Or` Rule Composer

`Or` Rule Composer is a rule that satisfies either of the two rules.
It is generally effective when you want to expand the condition range.

```rust
fn example_6() {
    type HelloOrWorldRule = Or![ContainsHelloRule, ContainsWorldRule];

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
fn example_7() {
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
struct StartsWithHelloRule;

struct StartsWithByeRule;

struct EndsWithJohnRule;

impl Rule for StartsWithHelloRule {
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.starts_with("Hello") {
            Ok(())
        } else {
            Err(Error::new(format!("{} does not start with `Hello`", target)))
        }
    }
}

impl Rule for StartsWithByeRule {
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.starts_with("Bye") {
            Ok(())
        } else {
            Err(Error::new(format!("{} does not start with `Bye`", target)))
        }
    }
}

impl Rule for EndsWithJohnRule {
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.ends_with("John") {
            Ok(())
        } else {
            Err(Error::new(format!("{} does not end with `John`", target)))
        }
    }
}

#[test]
fn example_8() {
    type GreetingRule = And![
        Or![StartsWithHelloRule, StartsWithByeRule],
        EndsWithJohnRule
    ];

    assert!(GreetingRule::validate(&"Hello! Nice to meet you John".to_string()).is_ok());
    assert!(GreetingRule::validate(&"Bye! Have a good day John".to_string()).is_ok());
    assert!(GreetingRule::validate(&"How are you? Have a good day John".to_string()).is_err());
    assert!(GreetingRule::validate(&"Bye! Have a good day Tom".to_string()).is_err());
}
```

# JSON

`refined_type` is compatible with `serde_json`. This ensures type-safe communication and eliminates the need to write
new validation processes. All you need to do is implement a set of rules once and implement `serde`’s `Serialize`
and `Deserialize`.

### Serialize

```rust
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
struct Human2 {
    name: NonEmptyString,
    age: u8,
}

fn example_9() -> anyhow::Result<()> {
    let john = Human2 {
        name: NonEmptyString::new("john".to_string())?,
        age: 8,
    };

    let actual = json!(john);
    let expected = json! {{
        "name": "john",
        "age": 8
    }};
    assert_eq!(actual, expected);
    Ok(())
}
```

### Deserialize

```rust
fn example_10() -> anyhow::Result<()> {
    let json = json! {{
        "name": "john",
        "age": 8
    }}
        .to_string();

    let actual = serde_json::from_str::<Human2>(&json)?;

    let expected = Human2 {
        name: NonEmptyString::new("john".to_string())?,
        age: 8,
    };
    assert_eq!(actual, expected);
    Ok(())
}
```

# Number

## `MinMax`

`MinMax` is a type that signifies the target exists between a certain number and another number.

```rust
type Age = MinMaxU8<18, 80>;

fn min_max_example() -> Result<(), Error<u8>> {
    let age = Age::new(18)?;
    assert_eq!(age.into_value(), 18);

    let age = Age::new(80)?;
    assert_eq!(age.into_value(), 80);

    let age = Age::new(17);
    assert!(age.is_err());

    let age = Age::new(81);
    assert!(age.is_err());
    Ok(())
}
```

## `Less`

`Less` is a type that signifies the target is less than a certain number.

```rust
type Age = LessU8<80>;

fn less_example() -> Result<(), Error<u8>> {
    let age = Age::new(79)?;
    assert_eq!(age.into_value(), 79);

    let age = Age::new(80);
    assert!(age.is_err());

    Ok(())
}
```

## `Greater`

`Greater` is a type that signifies the target is greater than a certain number.

```rust
type Age = GreaterU8<18>;

fn greater_example() -> Result<(), Error<u8>> {
    let age = Age::new(19)?;
    assert_eq!(age.into_value(), 19);

    let age = Age::new(18);
    assert!(age.is_err());

    Ok(())
}
```

## `Equal`

`Equal` is a type that signifies the target is equal to a certain number.

```rust
type Age = EqualU8<18>;

fn equal_example() -> Result<(), Error<u8>> {
    let age = Age::new(18)?;
    assert_eq!(age.into_value(), 18);

    let age = Age::new(19);
    assert!(age.is_err());

    Ok(())
}
```

# Iterator

`refined_type` has several useful refined types for Iterators.

## `ForAll`

`ForAll` is a rule that applies a specific rule to all elements in the Iterator.

```rust
fn example_11() -> anyhow::Result<()> {
    let vec = vec!["Hello".to_string(), "World".to_string()];
    let for_all_ok = ForAllVec::<NonEmptyStringRule>::new(vec.clone())?;
    assert_eq!(vec, for_all_ok.into_value());

    let vec = vec!["Hello".to_string(), "".to_string()];
    let for_all_err = ForAllVec::<NonEmptyStringRule>::new(vec.clone());
    assert!(for_all_err.is_err());
    Ok(())
}
```

## `Exists`

`Exists` is a rule that applies a specific rule to at least one element in the Iterator.

```rust
fn example_12() -> anyhow::Result<()> {
    let vec = vec!["Hello".to_string(), "".to_string()];
    let exists_ok = ExistsVec::<NonEmptyStringRule>::new(vec.clone())?;
    assert_eq!(vec, exists_ok.into_value());

    let vec = vec!["".to_string(), "".to_string()];
    let exists_err = ExistsVec::<NonEmptyStringRule>::new(vec.clone());
    assert!(exists_err.is_err());
    Ok(())
}
```

## `Head`

`Head` is a rule that applies a specific rule to the first element in the Iterator.

```rust
fn example_13() -> anyhow::Result<()> {
    let table = vec![
        (vec!["good morning".to_string(), "".to_string()], true), // PASS
        (vec!["hello".to_string(), "hello".to_string()], true),   // PASS
        (vec![], false),                                          // FAIL
        (vec!["".to_string()], false),                            // FAIL
        (vec!["".to_string(), "hello".to_string()], false),       // FAIL
    ];

    for (value, ok) in table {
        let head = HeadVec::<NonEmptyStringRule>::new(value.clone());
        assert_eq!(head.is_ok(), ok);
    }

    Ok(())
}
```

## `Last`

`Last` is a rule that applies a specific rule to the last element in the Iterator.

```rust
fn example_14() -> anyhow::Result<()> {
    let table = vec![
        (vec!["".to_string(), "hello".to_string()], true), // PASS
        (vec!["good morning".to_string(), "hello".to_string()], true), // PASS
        (vec![], false),                                   // FAIL
        (vec!["".to_string()], false),                     // FAIL
        (vec!["hello".to_string(), "".to_string()], false), // FAIL
    ];

    for (value, ok) in table {
        let last = LastVec::<NonEmptyStringRule>::new(value.clone());
        assert_eq!(last.is_ok(), ok);
    }

    Ok(())
}
```

## `Tail`

`Tail` is a rule that applies a specific rule to all elements except the first element in the Iterator.

```rust
fn example_15() -> anyhow::Result<()> {
    let table = vec![
        (vec!["hey".to_string(), "hello".to_string(), "world".to_string()], true),
        (vec!["hey".to_string(), "hello".to_string(), "".to_string()], false),
        (vec!["hey".to_string(), "".to_string(), "world".to_string()], false),
        (vec!["hey".to_string(), "".to_string(), "".to_string()], false),
        (vec!["".to_string(), "hello".to_string(), "world".to_string()], true),
        (vec!["".to_string(), "hello".to_string(), "".to_string()], false),
        (vec!["".to_string(), "".to_string(), "world".to_string()], false),
        (vec!["".to_string(), "".to_string(), "".to_string()], false),
    ];

    for (value, ok) in table {
        let tail = TailVec::<NonEmptyStringRule>::new(value.clone());
        assert_eq!(tail.is_ok(), ok);
    }

    Ok(())
}
```

## `Init`

`Init` is a rule that applies a specific rule to all elements except the last element in the Iterator.

```rust
fn example_16() -> anyhow::Result<()> {
    let table = vec![
        (vec!["hey".to_string(), "hello".to_string(), "world".to_string()], true),
        (vec!["hey".to_string(), "hello".to_string(), "".to_string()], true),
        (vec!["hey".to_string(), "".to_string(), "world".to_string()], false),
        (vec!["hey".to_string(), "".to_string(), "".to_string()], false),
        (vec!["".to_string(), "hello".to_string(), "world".to_string()], false),
        (vec!["".to_string(), "hello".to_string(), "".to_string()], false),
        (vec!["".to_string(), "".to_string(), "world".to_string()], false),
        (vec!["".to_string(), "".to_string(), "".to_string()], false),
    ];

    for (value, ok) in table {
        let init = InitVec::<NonEmptyStringRule>::new(value.clone());
        assert_eq!(init.is_ok(), ok);
    }

    Ok(())
}
```

## `Index`

`Index` is a rule that applies a specific rule to the element at a specific index in the Iterator.

```rust
fn example_17() -> anyhow::Result<()> {
    let table = vec![
        (vec!["good morning".to_string(), "hello".to_string()], true),
        (vec!["good morning".to_string(), "".to_string()], false),
        (vec!["".to_string(), "hello".to_string()], true),
        (vec!["".to_string(), "".to_string()], false),
    ];

    for (value, expected) in table {
        let refined = IndexVec::<1, NonEmptyStringRule>::new(value.clone());
        assert_eq!(refined.is_ok(), expected);
    }

    Ok(())
}
```

## `Reverse`

`Reverse` is a rule that applies a specific rule to all elements in the Iterator in reverse order.  

```rust
fn example_18() -> Result<(), Error<Vec<i32>>> {
    let table = vec![
        (vec!["good morning".to_string(), "hello".to_string()], true),
        (vec!["good morning".to_string(), "".to_string()], false),
        (vec!["".to_string(), "hello".to_string()], true),
        (vec!["".to_string(), "".to_string()], false),
    ];

    for (value, expected) in table {
        let refined = Reverse::<IndexRuleVec<0, NonEmptyStringRule>>::new(value.clone());
        assert_eq!(refined.is_ok(), expected);
    }

    Ok(())
}
```

# `Skip`

`Skip` is a rule that applies a specific rule to the elements of the Iterator while skipping the elements according
to `SkipOption`.

```rust
fn example_19() -> Result<(), Error<Vec<i32>>> {
    let table = vec![
        (vec!["hey".to_string(), "hello".to_string(), "world".to_string()], true),
        (vec!["hey".to_string(), "hello".to_string(), "".to_string()], false),
        (vec!["hey".to_string(), "".to_string(), "world".to_string()], false),
        (vec!["hey".to_string(), "".to_string(), "".to_string()], false),
        (vec!["".to_string(), "hello".to_string(), "world".to_string()], true),
        (vec!["".to_string(), "hello".to_string(), "".to_string()], false),
        (vec!["".to_string(), "".to_string(), "world".to_string()], false),
        (vec!["".to_string(), "".to_string(), "".to_string()], false),
    ];

    for (value, ok) in table {
        let init = SkipVec::<NonEmptyStringRule, SkipFirst<_>>::new(value.clone());
        assert_eq!(init.is_ok(), ok);
    }

    Ok(())
}
```

if you need more skip option, you can define it like this.

```rust
pub struct NoSkip<T> {
    _phantom_data: std::marker::PhantomData<T>,
}

impl<ITEM> SkipOption for NoSkip<ITEM> {
    type Item = ITEM;
    type Accumulator = ();
    fn should_skip(_: usize, _: Option<&mut Self::Accumulator>, _: &Self::Item) -> bool {
        false
    }
}
```

---

## `into_iter()` and `iter()`

The Iterator I’ve prepared has `into_iter` and `iter` implemented.
Therefore, you can easily map or convert it to a different Iterator using `collect`.
Feel free to explore the capabilities of the Iterator you’ve been given!

### `into_iter()`

```rust
fn example_20() -> anyhow::Result<()> {
    let ne_vec = NonEmptyVec::new(vec![1, 2, 3])?;
    let ne_vec: NonEmptyVec<i32> = ne_vec.into_iter().map(|n| n * 2).map(|n| n * 3).collect();
    assert_eq!(ne_vec.into_value(), vec![6, 12, 18]);
    Ok(())
}
```

### `iter()`

```rust
fn example_21() -> anyhow::Result<()> {
    let ne_vec = NonEmptyVec::new(vec![1, 2, 3])?;
    let ne_vec: NonEmptyVec<i32> = ne_vec.iter().map(|n| n * 2).map(|n| n * 3).collect();
    assert_eq!(ne_vec.into_value(), vec![6, 12, 18]);
    Ok(())
}
```

### `NonEmptyVec` to `NonEmptyVecDeque` using `collect()`

```rust
fn example_22() -> anyhow::Result<()> {
    let ne_vec = NonEmptyVec::new(vec![1, 2, 3])?;
    let ne_vec_deque: NonEmptyVecDeque<i32> = ne_vec.into_iter().collect();
    assert_eq!(ne_vec_deque.into_value(), vec![1, 2, 3]);
    Ok(())
}
```

# Length

You can impose constraints on objects that have a length, such as `String` or `Vec`.

## `LengthMinMax`
`LengthMinMax` is a type that signifies the target has a length between a certain number and another number.

```rust
fn length_min_max_example() -> Result<(), Error<String>> {
    type Password = LengthMinMax<5, 10, String>;

    let password = Password::new("123456".to_string())?;
    assert_eq!(password.into_value(), "123456");

    let password = Password::new("1234".to_string());
    assert!(password.is_err());

    let password = Password::new("12345678901".to_string());
    assert!(password.is_err());

    Ok(())
}
```

## `LengthGreater`
`LengthGreater` is a type that signifies the target has a length greater than a certain number.

```rust
fn length_greater_example() -> Result<(), Error<String>> {
    type Password = LengthGreater<5, String>;

    let password = Password::new("123456".to_string())?;
    assert_eq!(password.into_value(), "123456");

    let password = Password::new("1234".to_string());
    assert!(password.is_err());

    Ok(())
}
```

## `LengthLess`
`LengthLess` is a type that signifies the target has a length less than a certain number.

```rust
fn length_less_example() -> Result<(), Error<String>> {
    type Password = LengthLess<10, String>;

    let password = Password::new("123456".to_string())?;
    assert_eq!(password.into_value(), "123456");

    let password = Password::new("12345678901".to_string());
    assert!(password.is_err());

    Ok(())
}
```

## `LengthEqual`
`LengthEqual` is a type that signifies the target has a length equal to a certain number.

```rust
fn length_equal_example() -> Result<(), Error<String>> {
    type Password = LengthEqual<5, String>;

    let password = Password::new("12345".to_string())?;
    assert_eq!(password.into_value(), "12345");

    let password = Password::new("1234".to_string());
    assert!(password.is_err());

    Ok(())
}
```

## Custom Length

You can define a length for any type. Therefore, if you want to implement a length that is not provided
by `refined_type`, you can easily do so using `LengthDefinition`.

```rust
#[derive(Debug, PartialEq)]
struct Hello;
impl LengthDefinition for Hello {
    fn length(&self) -> usize {
        5
    }
}

fn custom_length_example() -> Result<(), Error<Hello>> {
    let hello = Refined::<LengthEqualRule<5, Hello>>::new(Hello)?;
    assert_eq!(hello.into_value(), Hello);
    Ok(())
}
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
