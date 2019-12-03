## Test impl Error
    
Try to use the `impl Trait` for `Error` handling in vanilla rust.
```rust
fn awesome_function()-> Result<String, impl Error> {
//    ...
}
```

## Sample code

```rust

fn load_play(play_name: String) -> Result<Play, impl Error> {
    // ...
}

//...

fn main() -> Result<(), impl Error> {
    let mut play = load_play("Romeo and Juliette".to_string())?;
    let text = get_text(play.borrow_mut())?;
    println!("{}", play.name);
    Ok(())
}
```

### Error

```
error[E0282]: type annotations needed
  --> vanilla/src/main.rs:11:49
   |
11 | fn load_play(play_name: String) -> Result<Play, impl Error> {
   |                                                 ^^^^^^^^^^ cannot infer type

error: aborting due to previous error
```


### Reason

https://doc.rust-lang.org/error-index.html#E0282

The type inference did not result in one unique possible type, it needs to be type annotated