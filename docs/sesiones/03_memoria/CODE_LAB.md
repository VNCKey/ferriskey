# 🧪 Code Lab: 03 - Memoria (Ownership, Borrowing & Stack vs Heap)

## Ejercicio 01: Stack & Copy Semantics
```rust
fn main() {
    let x = 42;
    let y = x; // Copia implícita bit a bit (Trait Copy)

    println!("x sigue siendo válido: {}", x);
    println!("y contiene una copia: {}", y);
}
```

---

## Ejercicio 02: Heap & Ownership Move
```rust
fn main() {
    let s1 = String::from("FerrisKey");
    let s2 = s1; // Move semantics: Ownership transferida a s2

    // println!("{}", s1); // ❌ Error de compilación: use of moved value `s1`
    println!("s2 es el único dueño activo: {}", s2);
}
```

---

## Ejercicio 03: Préstamos Mutables (`&mut T`)
```rust
fn agregar_sufijo(texto: &mut String) {
    texto.push_str(" - Aprende Rust sin errores");
}

fn main() {
    let mut mensaje = String::from("FerrisKey");
    
    // Préstamo mutable exclusivo
    agregar_sufijo(&mut mensaje);

    println!("Resultado: {}", mensaje);
}
```

---

## Ejercicio 04: Préstamo Inmutable vs Mutable (Borrow Checker Error)
```rust
fn main() {
    let mut datos = String::from("Rust");

    let r1 = &datos; // Préstamo inmutable
    let r2 = &datos; // Segundo préstamo inmutable

    println!("Lectores activos: {}, {}", r1, r2);
    // r1 y r2 ya no se usan después de esta línea (NLL - Non-Lexical Lifetimes)

    let r3 = &mut datos; // ✅ Válido porque r1 y r2 ya finalizaron su scope de uso
    r3.push_str(" 2026");
    println!("Modificación mutable: {}", r3);
}
```

---

## Ejercicio 05: Distinción entre `String` y `&str`
```rust
fn procesar_slice(slice: &str) {
    println!("Contenido prestado: '{}' | Longitud: {} bytes", slice, slice.len());
}

fn main() {
    let texto_heap: String = String::from("FerrisKey Desktop");
    let texto_literal: &str = "Hola Rust";

    // Pasamos slices &str
    procesar_slice(&texto_heap);
    procesar_slice(texto_literal);
    procesar_slice(&texto_heap[0..9]); // Slice parcial
}
```
