# 🧪 Code Lab: Sesión 15 - Anatomía de Proyectos Cargo y Workspaces

---

## 🎯 Reto 1: Estructurar un Paquete con Librería y Binarios

### Enunciado
Diseña la estructura de código donde `src/lib.rs` exporte una función de cifrado simple, y `src/bin/cli.rs` la consuma.

```rust
// src/lib.rs
pub fn cesar_cipher(texto: &str, desplazamiento: u8) -> String {
    texto
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (((c as u8 - base + desplazamiento) % 26) + base) as char
            } else {
                c
            }
        })
        .collect()
}

// src/bin/cli.rs
use mi_proyecto::cesar_cipher;

fn main() {
    let mensaje = "Rust 2024";
    let cifrado = cesar_cipher(mensaje, 3);
    println!("Texto cifrado: {}", cifrado);
}
```

---

## 🎯 Reto 2: Crear un Workspace con Dos Crates Locales

### Enunciado
Configura los manifiestos de un workspace con `servicios_math` (librería) y `calculadora_app` (binario ejecutable).

```toml
# Cargo.toml (raíz)
[workspace]
members = ["servicios_math", "calculadora_app"]

# servicios_math/Cargo.toml
[package]
name = "servicios_math"
version = "0.1.0"
edition = "2021"

# calculadora_app/Cargo.toml
[package]
name = "calculadora_app"
version = "0.1.0"
edition = "2021"

[dependencies]
servicios_math = { path = "../servicios_math" }
```
