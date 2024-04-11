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

One way to do this is to create a new `BitcoinValue` trait which will have a `to_btc` method and have the `Amount` struct *implement* that trait.s

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

Let's go through the changes:
1. We created a new trait `BitcoinValue` which declares the method signature `to_btc`. We set the argument type to be a shared reference to `self`.
2. We then implement that trait for the `Amount` struct
3. We added the `BitcoinValue` trait bound for `T` in the `as_btc` custom serialization method. And in the function body we simply call the trait method `to_btc` on `t` and then serialize it. 

This may seem like a lot and look fairly unfamiliar. Take some time to go through it and get familiar with generic functions, type parameters and trait bounds.

You may also feel as if this is overkill. It might feel like a lot extra, unnecessary code. But remember, the advantage here is that we separate the `Amount` type for internal purposes and calculations from how its serialized and displayed to the user. 

Alright if we run this now, we'll get the output we want! 
```shell
Transaction: {
  "version": 1,
  "inputs": [
    {
      "txid": "f8c693771b2992a11b53c045369ab31a920de1d921ff3c148a9d0487c8f90baf",
      "output_index": 16,
      "script": "483045022100904a2e0e8f597fc1cc271b6294b097a6edc952e30c453e3530f92492749769a8022018464c225b03c28791af06bc7fed129dcaaeff9ec8135ada1fb11762ce081ea9014104da289192b0845d5b89ce82665d88ac89d757cfc5fd997b1de8ae47f7780ce6a32207583b7458d1d2f3fd6b3a3b842aea9eb789e2bea57b03d40e684d8e1e0569",
      "sequence": 4294967295
    },
    {
      "txid": "e51d2177332baff9cfbbc08427cf0d85d28afdc81411cdbb84f40c95858b080d",
      "output_index": 1,
      "script": "4830450220369df7d42795239eabf9d41aee75e3ff20521754522bd067890f8eedf6044c6d0221009acfbd88d51d842db87ab990a48bed12b1f816e95502d0198ed080de456a988d014104e0ec988a679936cea80a88e6063d62dc85182e548a535faecd6e569fb565633de5b4e83d5a11fbad8b01908ce71e0374b006d84694b06f10bdc153ca58a53f87",
      "sequence": 4294967295
    }
  ],
  "outputs": [
    {
      "amount": 61.92597494,
      "script_pubkey": "76a914764b8c407b9b05cf35e9346f70985945507fa83a88ac"
    },
    {
      "amount": 1.27,
      "script_pubkey": "76a9141d1310fe87b53fec8dbc8911f0ebc112570e34b288ac"
    }
  ]
}
```

Excellent! But our code is starting to look a little unruly, let's see if we can organize it a bit into different files and modules. 

----------------------------------------------------------------------------------------------------------------------------------------------------

<div>
    <p align="right"><a href="17_file_organization_and_modules.md">>>> Next Lesson: File Organization and Modules</a></p>
</div>
