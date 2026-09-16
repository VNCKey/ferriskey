# 🧪 Code Lab: 09 - Custom Types

```rust
struct Usuario {
    nombre: String,
}

impl Usuario {
    fn nuevo(nombre: &str) -> Self {
        Self { nombre: nombre.to_string() }
    }
}

fn main() {
    let u = Usuario::nuevo("Ferris");
    println!("Usuario: {}", u.nombre);
}
```
