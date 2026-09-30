# 🧪 Code Lab: Sesión 13 - Tiempos de Vida (Lifetimes `'a`)

---

## 🎯 Reto 1: Comparador de Cadenas con Lifetime `'a`

### Enunciado
Implementa una función llamada `seleccionar_primero<'a>` que reciba dos referencias a cadenas (`&'a str`) y devuelva la que alfabéticamente sea menor según `.cmp()`.

```rust
fn seleccionar_primero<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a < b { a } else { b }
}

fn main() {
    let palabra1 = String::from("Alfa");
    let palabra2 = String::from("Beta");
    let res = seleccionar_primero(&palabra1, &palabra2);
    println!("Primera en alfabeto: {}", res);
}
```

---

## 🎯 Reto 2: Struct con Referencia y Método de Extracción

### Enunciado
Crea una estructura `ResumenArticulo<'a>` que contenga una referencia a un titular (`&'a str`) y un método `primeras_palabras(&self, n: usize) -> &'a str`.

```rust
struct ResumenArticulo<'a> {
    titular: &'a str,
}

impl<'a> ResumenArticulo<'a> {
    fn new(titular: &'a str) -> Self {
        Self { titular }
    }

    fn obtener_titular(&self) -> &'a str {
        self.titular
    }
}

fn main() {
    let texto = String::from("Rust 2024 lanzado oficialmente");
    let resumen = ResumenArticulo::new(&texto);
    println!("Titular: {}", resumen.obtener_titular());
}
```
