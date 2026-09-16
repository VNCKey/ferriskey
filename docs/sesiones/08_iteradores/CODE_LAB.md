# 🧪 Code Lab: 08 - Iteradores & Pipeline Perezoso

## Ejercicio 01: Comparación de Modos de Iteración (`.iter()` vs `.into_iter()`)
```rust
fn main() {
    let numeros = vec![1, 2, 3];

    // 1. .iter() presta referencias inmutables
    for num in numeros.iter() {
        println!("Referencia: {}", num);
    }
    // 'numeros' sigue siendo válido aquí

    // 2. .into_iter() consume la colección
    let suma: i32 = numeros.into_iter().sum();
    println!("Suma total consumida: {}", suma);
    // 'numeros' ya NO es accesible aquí
}
```

---

## Ejercicio 02: Pipeline Perezoso de Transformación Funcional
```rust
fn main() {
    let datos = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Cadena de iteración: Filtrar pares, multiplicar por 10 y recolectar en Vec
    let resultado: Vec<i32> = datos
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 10)
        .collect();

    println!("Resultado del pipeline: {:?}", resultado);
}
```

---

## Ejercicio 03: Uso de Turbofish (`::<T>`) con `.collect()`
```rust
use std::collections::HashSet;

fn main() {
    let nombres = vec!["Alice", "Bob", "Alice", "Charlie"];

    // Sintaxis Turbofish ::<HashSet<_>> para resolver el tipo de destino
    let unicos = nombres.into_iter().collect::<HashSet<_>>();

    println!("Nombres únicos recopilados: {:?}", unicos);
}
```
