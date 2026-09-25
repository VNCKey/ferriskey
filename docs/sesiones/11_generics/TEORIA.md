# 📖 Teoría: Sesión 11 - Genéricos y Monomorfización

---

## 1. ¿Qué son los Genéricos?

Los **Genéricos** permiten escribir código reutilizable y libre de duplicaciones que puede operar sobre múltiples tipos de datos concretos, manteniendo el 100% de la seguridad de tipos estática en compilación.

```rust
// Función genérica que trabaja con cualquier tipo T:
fn identidad<T>(valor: T) -> T {
    valor
}

let n = identidad(42);       // T es i32
let s = identidad("Ferris"); // T es &str
```

---

## 2. Estructuras Genéricas (`struct`)

Podemos declarar estructuras donde uno o varios campos adopten tipos genéricos:

```rust
struct Punto<T> {
    x: T,
    y: T,
}

let entero = Punto { x: 5, y: 10 };
let flotante = Punto { x: 1.0, y: 4.0 };
// let mixto = Punto { x: 5, y: 4.0 }; // ❌ ERROR: x e y deben ser del mismo tipo T
```

Para admitir tipos distintos en campos independientes:
```rust
struct PuntoMixto<T, U> {
    x: T,
    y: U,
}
```

---

## 3. Restricciones de Traits (*Trait Bounds*)

Un tipo genérico puro `<T>` no permite realizar casi ninguna operación porque el compilador no sabe si `T` se puede sumar, imprimir o comparar. Para exigir ciertas capacidades, aplicamos restricciones (*Bounds*):

```rust
use std::fmt::Display;

// Exigimos que T implemente Display para poder imprimirlo con {}
fn mostrar_mayor<T: PartialOrd + Display>(a: T, b: T) {
    if a >= b {
        println!("El mayor es: {a}");
    } else {
        println!("El mayor es: {b}");
    }
}
```

### Sintaxis Alternativa: `impl Trait`
Para firmas sencillas, podemos escribir:
```rust
fn imprimir(item: impl Display) {
    println!("{item}");
}
```

### Cláusulas `where` para Firmas Complejas
Cuando una función tiene múltiples genéricos con varios bounds cada uno, la cláusula `where` mantiene la firma limpia:

```rust
fn procesar_datos<T, U>(dato: T, canal: U) -> String
where
    T: Display + Clone,
    U: std::fmt::Debug + Send,
{
    format!("Dato: {dato}, Canal: {canal:?}")
}
```

---

## 4. El Costo de los Genéricos: Monomorfización

¿Hacen los genéricos que Rust sea más lento en tiempo de ejecución? **No.**

Rust utiliza un proceso llamado **Monomorfización** (*Monomorphization*):
1. Durante la compilación, `rustc` examina todas las llamadas que se hacen a una función genérica.
2. Genera una copia en código máquina especializada para cada tipo concreto utilizado (ej: crea una versión nativa para `i32` y otra separada para `f64`).
3. Como resultado, en tiempo de ejecución **no existe ninguna penalización de rendimiento**: el código corre a la misma velocidad que si hubieses programado a mano cada función para su tipo específico.
