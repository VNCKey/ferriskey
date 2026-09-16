# 🧪 Code Lab: 07 - Closures

## Ejercicio 01: Captura por Préstamo Inmutable
```rust
fn main() {
    let factor = 10;
    let multiplicar = |x: i32| x * factor;

    println!("Resultado de 5 * 10: {}", multiplicar(5));
    println!("El factor sigue accesible: {}", factor);
}
```

---

## Ejercicio 02: Captura con Transferencia de Propiedad (`move`)
```rust
fn main() {
    let mensaje = String::from("Hola desde FerrisKey");

    // 'move' fuerza la transferencia de propiedad al closure (útil para threads)
    let imprimir = move || println!("{}", mensaje);

    imprimir();
    // println!("{}", mensaje); // ❌ Error! 'mensaje' fue movido al closure
}
```

---

## Ejercicio 03: Closures como Parámetros de Función (`impl Fn`)
```rust
fn ejecutar_operacion<F>(val: i32, operacion: F) -> i32
where
    F: Fn(i32) -> i32,
{
    operacion(val)
}

fn main() {
    let resultado = ejecutar_operacion(7, |n| n * n);
    println!("Cuadrado de 7: {}", resultado);
}
```
