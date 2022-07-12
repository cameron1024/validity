# validity

Compile-time enforcement of arbitrary invariants

In general, it is a good idea to make invalid states unrepresentable. This can often be achieved using the type system, but there is a limit to the type system's power.

For example, imagine you're trying to represent a user's email address. You might write a struct like this: 
```rust
struct Email(pub String);
```
But you don't want any old string to be allowed, you might want to validate that it is actually a valid email address. So your code might look like:
```rust
fn validate_email(email: &Email) -> bool {
  // ...
}

fn handle_email(email: Email) {
  if !validate_email(&email) {
    panic!("invalid email");  // or return Err(...)
  }
  
  do_stuff_with_valid_email(email: Email);
}

fn do_stuff_with_valid_email(email: Email) {
  println!("definitely a valid email: {}", email.0);
}
```
This works, but we're "hiding" a particular invariant from the compiler: `do_stuff_with_email` cannot be called with an `Email` that might return `false` when passed to `validate_email`.

However, with `validity`, we can make a few small changes to tell the compiler about this invariant:

 - Firstly, define what it means for an email to be "valid"
```rust
impl Validate for Email {
  type Error = &'static str;  // preferably a more meaningful error type
  
  fn is_valid(&self) -> Result<(), Self::Error> {
    match validate_email(self) {
      true => Ok(()),
      false => Err("invalid email"),
    }
  }
}
```
 - Then, we mark `do_stuff_with_valid_email` as needing a valid email address:
```rust
fn do_stuff_with_valid_email(email: Valid<Email>) {
  // ...
}
```
The only way to get a `Valid<Email>` is by going through the `validate` function and handling any potential errors:
```rust
fn handle_email(email: Email) {
  match email.validate() {
    Ok(valid_email) => do_stuff_with_valid_email(valid_email),
    Err(_) => println!("uh oh!"),
  }
}
```

Great! We now *can't forget to validate* emails, since there is no other way to create a `Valid<T>`.

### Warning - determinism

Note, this implementation assumes that `is_valid` is deterministic (i.e. it gives the same result every time it's called). For example, the following implementation can allow an invalid value to exist inside a `Valid<T>`:
```rust
impl Valid for MyType {
  type Error = ();
  fn is_valid(&self) -> Result<(), ()> {
    match rand::random() {
      true => Ok(()),
      false => Err(()),
    }
  }
}
```




