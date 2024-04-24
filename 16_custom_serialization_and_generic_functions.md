# Custom Serialization and Generic Functions

So what we want to do is keep the amount's type in the `Output` struct as `Amount` and not an `f64`. Instead of storing it as an `f64`, we can add a custom field serialization attribute to serialize it as an `f64` type denominated in Bitcoin.

Serde offers an attribute for custom serialization methods on a particular struct's fields. Here is the relevant documentation: https://serde.rs/field-attrs.html#serialize_with

As the docs explain, we can implement a custom function that will be called to provide the serialization logic. However, that function must be *callable* as the following type: `fn<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer`.

Let's break this function signature down a bit. 

You may notice the unfamiliar `<S>` before the arguments and then a `where` clause after the return type. This is known as a **generic** function. The phrase `<S>` is what makes it generic and is called a **type parameter**. What this means is that throughout the body of the function, `S` stands for some type that *implements* the **Serializer** trait. The `where` clause is sometimes more readable if there are a lot of different traits that the type is required to implement. However, this could also be written as `fn<S: Serializer>(&T, S) -> Result<S::Ok, S::Error>`. Finally, you'll notice the `&T` as the first function argument, which just means that argument is a reference to any generic type `T`.

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

One way to do this is to create a new `BitcoinValue` trait which will have a `to_btc` method and have the `Amount` struct *implement* that trait.

Take a look at the changes below:
```rust
...

#[derive(Debug)]
pub struct Amount(u64);

trait BitcoinValue {
    fn to_btc(&self) -> f64;
}

impl BitcoinValue for Amount {
    fn to_btc(&self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}

#[derive(Debug, Serialize)]
struct Output {
    #[serde(serialize_with = "as_btc")]
    amount: Amount,
    script_pubkey: String,
}

fn as_btc<T: BitcoinValue, S: Serializer>(t: &T, s: S) -> Result<S::Ok, S::Error> {
    let btc = t.to_btc();
    s.serialize_f64(btc)
}

...
```

Let's review the changes:
1. We created a new trait `BitcoinValue` which declares the method signature `to_btc`. We set the argument type to be a shared reference to `self`.
2. We then implement that trait for the `Amount` struct
3. We added the `BitcoinValue` trait bound for `T` in the `as_btc` custom serialization method. And in the function body we simply call the trait method `to_btc` on `t` and then serialize it.

This may seem like a lot and look fairly unfamiliar. Take some time to go through it and get familiar with generic functions, type parameters and trait bounds.

You may also feel as if this is overkill. It might seem like a lot extra, unnecessary code. But remember, the advantage here is that we separate the `Amount` type for internal purposes and calculations from how its serialized and displayed to the user. And we could potentially have other types in the future that implement the `BitcoinValue` trait.

Alright if we run this now, we'll get the output we want!
```console
Transaction: {
  "version": 1,
  "inputs": [
    {
      "txid": "8073cdf947ac97c23b77b055217da78d3ad71d30e1f6c095be8b30f7d6c1d542",
      "output_index": 1,
      "script": "4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5",
      "sequence": 4294967294
    },
    {
      "txid": "9cb414caf4a633b3446c22d6174be670b3e0e746024cc0c1ef0e15f3c57cc875",
      "output_index": 0,
      "script": "483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abf",
      "sequence": 4294967294
    }
  ],
  "outputs": [
    {
      "amount": 0.01028587,
      "script_pubkey": "76a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac"
    },
    {
      "amount": 0.02002,
      "script_pubkey": "a91476c0c8f2fc403c5edaea365f6a284317b9cdf72587"
    }
  ]
}
```

Excellent! But our code is starting to look a little unruly, let's see if we can organize it a bit into different files and modules.

<hr/>

<div>
  <p align="right"><a href="17_file_organization_and_modules.md">>>> Next Lesson: File Organization and Modules</a></p>
</div>
