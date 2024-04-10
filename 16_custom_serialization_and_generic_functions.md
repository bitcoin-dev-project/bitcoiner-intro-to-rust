# Custom Serialization and Generic Functions

So what we want to do is keep the amount's type in the `Output` struct as `Amount` and not an `f64`. Instead of storing it as an f64, we can add a custom field serialization attribute to serialize it as an `f64` type denominated in Bitcoin. 

Serde offers an attribute for custom serialization methods on a particular struct's fields. Here is the relevant documentation: https://serde.rs/field-attrs.html#serialize_with

As the docs explain, we can implement a custom function that will be called to provide the serialization logic. However, that function must be *callable* as the following type: `fn<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer`.

Let's break this function signature down a bit. You may notice the unfamiliar `<S>` before the arguments and then a `where` clause after the return type. This is known as a **generic** function. The phrase `<S>` is what makes it generic and is called a **type parameter**. What this means is that throughout the body of the function, `S` stands for some type that implements the **Serializer** trait. The `where` clause is sometimes more readable if there are a lot of different traits that the type is required to implement. However, this could also be written as `fn<S: Serializer>(&T, S) -> Result<S::Ok, S::Error>`. Finally, you'll notice the `&T` as the first function argument, which just means that argument is a reference to any generic type `T`. 

So let's try making the following modifications:
1. Add the `serde serialize_with` attribute to the `amount` field of the `Output` struct
2. Implement the serialization function that we are calling `as_btc`

```rust
...

#[derive(Debug, Serialize)]
struct Output {
    #[serde(serialize_with = "as_btc")]
    amount: Amount,
    script_pubkey: String,
}

fn as_btc<S: Serializer>(t: &T, s: S) -> Result<S::Ok, S::Error> {
    let btc = t.to_btc();
    s.serialize_f64(btc)
}

...
```

The methods available for the `Serializer` trait can be found here: https://docs.rs/serde/latest/serde/ser/trait.Serializer.html#required-methods. I looked through it and found the relevant one for us which is to serialize the `f64` type with the `serialize_f64` method.

If we run this now, we'll get a few compiler errors:
1. We'll need to bring the `Serializer` trait into scope by modifying the `use` statement above.
2. We'll need to include `T` in the type parameters, since the compiler doesn't understand what `T` means in the arguments otherwise.
3. We'll need to derive the `Debug` attribute for the `Amount` struct. This is because the `Output` is attempting to derive the `Debug` attribute and the only way it can do that is if all its fields implements the `Debug` trait.
4. Finally, we'll need to remove the call to `to_btc()` after reading the amount in our main function, since we want the Output to have an `Amount` field and not an `f64` field.

Let's make all those changes:

```rust
use std::io::Read;
use serde::{Serialize, Serializer};

...

#[derive(Debug)]
pub struct Amount(u64);

...

fn as_btc<T, S: Serializer>(t: &T, s: S) -> Result<S::Ok, S::Error> {
    let btc = t.to_btc();
    s.serialize_f64(btc)
}

...

    for _ in 0..output_length {
        let amount = read_amount(&mut bytes_slice);
        let script_pubkey = read_script(&mut bytes_slice);

        outputs.push(Output {
            amount,
            script_pubkey,
        });
    }

...
```

If we run this now, we should get one remaining compiler error:
```shell
error[E0599]: no method named `to_btc` found for reference `&T` in the current scope
  --> src/main.rs:36:17
   |
36 |     let btc = t.to_btc();
   |                 ^^^^^^ method not found in `&T`
```

In our `as_btc` serializer method, we can't simply call `to_btc` on the generic type `T`. There are two problems here:
1. We need to bound the trait so that only certain types can be passed into this method. 
2. The `to_btc` method is not a trait method. It's simply a method on a struct. So we need to create a trait method as that is the only way to call a method on a generic type parameter. 

One way we can do this, which is similar to how `Rust-Bitcoin` library does it, is create a new trait and implement that trait's method for `Amount`. We'll call the trait `SerdeAmount`, similar to `Rust-Bitcoin`. https://github.com/rust-bitcoin/rust-bitcoin/blob/163bf64fcc36ed12e3e07301fb2d18d30742a0eb/units/src/amount.rs#L1639

Take a look at the changes below:

```rust
...

#[derive(Debug)]
pub struct Amount(u64);

impl Amount {
    fn to_btc(self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}

pub trait SerdeAmount {
    fn ser_btc<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error>;
}

impl SerdeAmount for Amount {
    fn ser_btc<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_f64(self.to_btc())
    }
}

#[derive(Debug, Serialize)]
struct Output {
    #[serde(serialize_with = "as_btc")]
    amount: Amount,
    script_pubkey: String,
}

fn as_btc<T: SerdeAmount, S: Serializer>(t: &T, s: S) -> Result<S::Ok, S::Error> {
    t.ser_btc(s)
}

...
```

Let's go through the changes:
1. We created a new trait `SerdeAmount` which provides the template method `ser_btc`. This template method assumes a `Serializer` will be passed in and then return the appropriate `Result` type. Notice how we pass in a shared reference to `self` for both of these methods. We'll go into more detail why we're doing this in the next section.
2. We then implement that trait for the `Amount` struct. Note that we still call the correct struct method with `self.to_btc()` and then pass it in to the `serialize_f64` method.
3. We added a trait bound for `T` in the `as_btc` custom serialization method which must implement the `SerdeAmount` trait. And in the function body we simply call the trait method `ser_btc` and pass in the serializer. 

This may seem like a lot and look fairly unfamiliar. Take some time to go through it and get familiar with generic functions, type parameters and trait bounds. 

You may also feel as if this is overkill. It might feel like a lot extra, unnecessary code. But remember, the advantage here is that we separate the `Amount` type for internal purposes and calculations from how its serialized and displayed to the user. 

Alright if we run this now, we'll run into yet another compiler error: 
```shell
error[E0507]: cannot move out of `*self` which is behind a shared reference
  --> src/main.rs:34:25
   |
34 |         s.serialize_f64(self.to_btc())
   |                         ^^^^ -------- `*self` moved due to this method call
   |                         |
   |                         move occurs because `*self` has type `Amount`, which does not implement the `Copy` trait
   |
note: `Amount::to_btc` takes ownership of the receiver `self`, which moves `*self`
  --> src/main.rs:23:15
   |
23 |     fn to_btc(self) -> f64 {
   |               ^^^^
```

This is an interesting one and has to do with the topic of copying and moving. We'll explore this more in the next section. 

### Quiz
*In the `Rust-Bitcoin` library, the `SerdeAmount` trait includes the bounds `Copy` and `Sized`. What is the purpose of each of those traits and are why they included here? https://github.com/rust-bitcoin/rust-bitcoin/blob/163bf64fcc36ed12e3e07301fb2d18d30742a0eb/units/src/amount.rs#L1639*
