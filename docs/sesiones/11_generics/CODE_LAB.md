# 🧪 Code Lab: 11 - Generics

```rust
struct Par<T, U> {
    primero: T,
    segundo: U,
}

fn main() {
    let p = Par { primero: 10, segundo: "Hola" };
    println!("Par: {}, {}", p.primero, p.segundo);
}
```
