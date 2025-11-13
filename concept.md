### Different types of Result:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
Result<T, E> // <- this is enum
Result<(), std::io::Error> // <- return void or i/o bound error

/// This is type alias of 
/// Result<T, std::io::Error>
/// known at compile time bacause it is concreate
std::io::Result<()>
type Result<T> = Result<T, std::io::Error>

/// dynamic error with heap allocaion
Result<(), Box<dyn std::error::Error>>

/*
// Example
fn main() -> std::io::Result<()>
fn main() -> Result<(), std::io::Error>
fn main() -> Result<(), Box<dyn std::error::Error>>
*/

```

---
