# About

This is a summary with the core concepts of chapter 02

1. `prelude` is all the default imports that come in a module
2. `cargo doc --open` will open documentation on the browser
3. `(1..100)` -> exclusive `(1..=100)` -> inclusive in range definitions
4. The type can be included in a string using `{}`. Ex:

```rust
let a = 12;
println!("var is: {a}");
println!("var is: {}", a);
```

5. `let` defines an inmutable variable while `let mut` defines a mutable variable
6. use the `&` to pass a value by reference.
7. `<Type>::fn()` means that the function is part of the type and not of the instance
8. `enumerations` are not only values but actually represent results. Ex: this parse method
   return an enumaration with an `Ok` and `Err` scenarios

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("The input is not a number!");
        continue;
    }
};
```

9. The `match` operator allows you to compare using the proper `enumerations`
10. In the case of parsing variables, for example from a `String` to an `u32` we can use `shadowing`.
    Which means that we define the variable with the same name but a different type.
