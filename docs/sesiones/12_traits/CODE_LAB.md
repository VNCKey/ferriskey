# 🧪 Code Lab: 12 - Traits

```rust
trait Imprimible {
    fn imprimir(&self);
}

struct Mensaje(String);

impl Imprimible for Mensaje {
    fn imprimir(&self) {
        println!("{}", self.0);
    }
}

fn main() {
    let m = Mensaje("Hola desde Trait".to_string());
    m.imprimir();
}
```
