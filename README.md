# Refined Type

**refined_type** is a library developed for Rust. It enhances your types, making them more robust and expanding the
range of guarantees your applications can statically ensure.

# Installation

```shell
cargo add refined_type
```

# Overview

You can create various rules for a certain type, such as phone numbers, addresses, times, and so on.
Once you have established the rules, you can easily combine them.
Specifically, if you create rules for 'non-empty strings' and 'strings composed only of alphabets,' you do not need to
redefine a new rule for 'non-empty strings composed only of alphabets'.
All rules can be arbitrarily combined and extended as long as the target type matches. Enjoy a wonderful type life!

# Example Usage

As an example, let's convert from JSON to a struct.

```rust
// define the constraints you expect by combining 'Refined' and 'Rule'.
type MyNonEmptyString = Refined<NonEmptyRule<String>>;
type MyNonEmptyVec<T> = Refined<NonEmptyRule<Vec<T>>>;

// define a struct for converting from JSON.
#[derive(Debug, Eq, PartialEq, Deserialize)]
struct Human {
    name: MyNonEmptyString,
    friends: MyNonEmptyVec<String>,
}

fn example_1() -> anyhow::Result<()> {
    let json = json! {{
        "name": "john",
        "friends": ["tom", "taro"]
    }}
        .to_string();

    let actual = serde_json::from_str::<Human>(&json)?;
    let expected = Human {
        name: MyNonEmptyString::new("john".to_string())?,
        friends: MyNonEmptyVec::new(vec!["tom".to_string(), "taro".to_string()])?,
    };
    assert_eq!(actual, expected);
    Ok(())
}

// In the second example, while `friends` meets the rule, `name` does not, causing the conversion from JSON to fail
fn example_2() -> anyhow::Result<()> {
    let json = json! {{
        "name": "",
        "friends": ["tom", "taro"]
    }}
        .to_string();

    // because `name` is empty
    assert!(serde_json::from_str::<Human>(&json).is_err());
    Ok(())
}

// In the third example, while `name` satisfies the rule, `friends` does not, causing the conversion from JSON to fail.
fn example_3() -> anyhow::Result<()> {
    let json = json! {{
        "name": "john",
        "friends": []
    }}
        .to_string();

    // because `friends` is empty
    assert!(serde_json::from_str::<Human>(&json).is_err());
    Ok(())
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
        let refined = Index1Vec::<NonEmptyStringRule>::new(value.clone());
        assert_eq!(refined.is_ok(), expected);
    }

    Ok(())
}
```

if you need more, you can define it like this.

```rust
define_index_refined!(11, 12, 13);
define_index_rule!(11, 12, 13);
```

## `Reverse`

`Reverse` is a rule that applies a specific rule to all elements in the Iterator in reverse order.  
`refined_type` crate has `Index0` to `Index10` by default.

```rust
fn example_18() -> Result<(), Error<Vec<i32>>> {
    let table = vec![
        (vec!["good morning".to_string(), "hello".to_string()], true),
        (vec!["good morning".to_string(), "".to_string()], false),
        (vec!["".to_string(), "hello".to_string()], true),
        (vec!["".to_string(), "".to_string()], false),
    ];

    for (value, expected) in table {
        let refined = Reverse::<Index0VecRule<NonEmptyStringRule>>::new(value.clone());
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

### String

```rust
fn example_23() -> Result<(), Error> {
    length_greater_than!(5);
    length_equal!(5, 10);
    length_less_than!(10);

    type Password = Refined<From5To10Rule<String>>;

    type From5To10Rule<T> = And![
        Or![LengthEqualRule5<T>, LengthGreaterThanRule5<T>],
        Or![LengthLessThanRule10<T>, LengthEqualRule10<T>]
    ];

    // length is 8. so, this is valid
    let raw_password = "password";
    let password = Password::new(raw_password.to_string())?;
    assert_eq!(password.into_value(), "password");

    // length is 4. so, this is invalid
    let raw_password = "pswd";
    let password = Password::new(raw_password.to_string());
    assert!(password.is_err());

    // length is 17. so, this is invalid
    let raw_password = "password password";
    let password = Password::new(raw_password.to_string());
    assert!(password.is_err());

    Ok(())
}
```

### Vec

```rust
#[test]
fn example_24() -> anyhow::Result<()> {
    length_greater_than!(5);
    length_equal!(5, 10);
    length_less_than!(10);

    type Friends = Refined<From5To10Rule<Vec<String>>>;

    type From5To10Rule<T> = And![
        Or![LengthEqualRule5<T>, LengthGreaterThanRule5<T>],
        Or![LengthLessThanRule10<T>, LengthEqualRule10<T>],
    ];

    // length is 6. so, this is valid
    let raw_friends = vec![
        "Tom".to_string(),
        "Taro".to_string(),
        "Jiro".to_string(),
        "Hanako".to_string(),
        "Sachiko".to_string(),
        "Yoshiko".to_string(),
    ];
    let friends = Friends::new(raw_friends.clone())?;
    assert_eq!(friends.into_value(), raw_friends);

    // length is 2. so, this is invalid
    let raw_friends = vec!["Tom".to_string(), "Taro".to_string()];
    let friends = Friends::new(raw_friends.clone());
    assert!(friends.is_err());

    // length is 11. so, this is invalid
    let raw_friends = vec![
        "Tom".to_string(),
        "Taro".to_string(),
        "Jiro".to_string(),
        "Hanako".to_string(),
        "Sachiko".to_string(),
        "Yuiko".to_string(),
        "Taiko".to_string(),
        "John".to_string(),
        "Jane".to_string(),
        "Jack".to_string(),
        "Jill".to_string(),
    ];
    let friends = Friends::new(raw_friends.clone());
    assert!(friends.is_err());

    Ok(())
}
```

### Custom Length

You can define a length for any type. Therefore, if you want to implement a length that is not provided
by `refined_type`, you can easily do so using `LengthDefinition`.

```rust
#[test]
fn example_25() -> anyhow::Result<()> {
    length_equal!(5);

    #[derive(Debug, PartialEq)]
    struct Hello;
    impl LengthDefinition for Hello {
        fn length(&self) -> usize {
            5
        }
    }

    let hello = Refined::<LengthEqualRule5<Hello>>::new(Hello)?;
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