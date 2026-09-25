# 📖 Teoría: Sesión 03 - Memoria (Ownership, Borrowing & Stack vs Heap)

---

## 1. La Arquitectura de la Memoria: Stack vs. Heap

En los lenguajes de sistemas, entender dónde se almacena cada byte en el hardware es fundamental para el rendimiento y la seguridad.

```text
┌─────────────────────────────────────────┬─────────────────────────────────────────┐
│              STACK (Pila)               │             HEAP (Montículo)            │
├─────────────────────────────────────────┼─────────────────────────────────────────┤
│ • Estructura LIFO (Last In, First Out). │ • Memoria dinámica sin orden secuencial.│
│ • Tamaño conocido en compilación.       │ • Tamaño desconocido o variable en run. │
│ • Asignación ultra rápida (puntero SP). │ • Asignación costosa (llamada al SO).   │
│ • Acceso directo contiguo en memoria.   │ • Acceso indirecto mediante punteros.  │
│ • Limpieza automática al salir del {}.  │ • Administrado estrictamente por Rust.  │
└─────────────────────────────────────────┴─────────────────────────────────────────┘
```

### Anatomía de un `String` en Memoria

Cuando creas una cadena dinámica en Rust:

```rust
let s1 = String::from("Ferris");
```

La memoria se organiza físicamente en dos sectores:

```text
STACK (Stack Frame de la función)                   HEAP (Memoria Dinámica del SO)
┌─────────────────────────────────┐                ┌───┬───┬───┬───┬───┬───┐
│ ptr: 0x7ffd00 (puntero al Heap) ├─referencia────►│ F │ e │ r │ r │ i │ s │
│ len: 6 bytes (longitud actual)  │                └───┴───┴───┴───┴───┴───┘
│ cap: 6 bytes (capacidad total)  │                Índice: 0   1   2   3   4   5
└─────────────────────────────────┘
```

- En el **Stack**: se alojan 24 bytes (en arquitecturas de 64 bits):
  - `ptr` (8 bytes): dirección de memoria física en el Heap donde comienzan los caracteres.
  - `len` (8 bytes): cuántos bytes de contenido están en uso actualmente.
  - `cap` (8 bytes): cuántos bytes en total ha reservado el sistema operativo en el Heap.
- En el **Heap**: se aloja el contenido textual real en una secuencia contigua de bytes UTF-8.

---

## 2. Las Tres Reglas de Oro del Ownership

El modelo de **Ownership** (Propiedad) es la innovación central de Rust que permite prescindir por completo de un recolector de basura (*Garbage Collector*) sin exigir que el desarrollador gestione la memoria manualmente con `malloc()` / `free()`.

1. **Cada valor en Rust tiene un único propietario** (una variable).
2. **Solo puede haber un único propietario a la vez**.
3. **Cuando el propietario sale de su ámbito (*scope*), el valor se destruye automáticamente** (se invoca la función `drop`).

### ¿Por qué existe esta regla? La Prevención del *Double Free*
En lenguajes como C++, si dos variables apuntan al mismo bloque de memoria en el Heap y ambas intentan liberarlo cuando salen del ámbito, se produce un fallo crítico de seguridad conocido como **Double Free** (doble liberación), que puede corromper la memoria y abrir vulnerabilidades de ejecución arbitraria de código. Rust resuelve esto mediante **Move Semantics**.

---

## 3. Semántica de Copia (Copy) vs. Movimiento (Move)

### A. Tipos con Trait `Copy` (Stack)
Los tipos primitivos escalares cuyo tamaño es fijo y conocido en tiempo de compilación implementan el trait `Copy`. Al asignarlos a otra variable, se realiza una copia rápida bit a bit en el Stack:

```rust
let x = 42;
let y = x; // Copia bit a bit en el Stack

println!("x = {x}, y = {y}"); // ✅ Ambas variables siguen siendo completamente válidas
```

*Tipos que implementan `Copy`:* todos los tipos numéricos (`i32`, `u64`, `f64`, etc.), `bool`, `char` y tuplas/arreglos fijos que solo contengan tipos `Copy`.

### B. Tipos con Semántica `Move` (Heap)
Para tipos dinámicos como `String` o `Vec<T>`, una asignación no duplica los datos en el Heap (lo cual sería ineficiente en tiempo y consumo de memoria). En su lugar, Rust **mueve la propiedad** copiando la tripleta del Stack e **invalidando inmediatamente la variable original**:

```rust
let s1 = String::from("Ferris");
let s2 = s1; // 🚚 Move: s2 ahora es el único dueño. s1 queda INVALIDADA.

// println!("{s1}"); // ❌ ERROR DE COMPILACIÓN: borrow of moved value: `s1`
println!("{s2}");    // ✅ Válido: s2 es el único responsable
```

Si realmente se desea duplicar los datos en el Heap, se debe invocar explícitamente el método `.clone()`:

```rust
let s2 = s1.clone(); // Duplicación profunda (deep copy) en el Heap
```

---

## 4. El Sistema de Préstamos (Borrowing) y Referencias

Para utilizar un dato sin arrebatarle su propiedad, Rust introduce el concepto de **Borrowing** (Préstamos) mediante **Referencias (`&`)**:

```rust
fn calcular_longitud(texto: &String) -> usize {
    texto.len()
} // Aquí `texto` sale del ámbito, pero como no es el dueño, NO libera el Heap
```

### Tabla Comparativa de Préstamos

| Tipo de Préstamo | Sintaxis | Permisos de Acceso | Concurrencia Permitida | Regla de Oro |
|---|---|---|---|---|
| **Inmutable (`&T`)** | `let r = &s;` | Solo Lectura | Ilimitadas referencias simultáneas | Múltiples observadores pueden leer en paralelo sin peligro. |
| **Mutable (`&mut T`)** | `let r = &mut s;` | Lectura y Escritura | **EXACTAMENTE UNA** activa a la vez | Acceso exclusivo garantizado mientras viva el préstamo. |
| **Conflicto Prohibido** | `&` y `&mut` combinados | Incompatible | **0 mutables si existen lectores** | Previene que un escritor altere datos mientras otros los leen (*Data Race*). |

```rust
let mut mensaje = String::from("Hola");

let r1 = &mensaje; // Préstamo inmutable
let r2 = &mensaje; // Otro préstamo inmutable
println!("{r1} y {r2}"); // Lectura compartida

// A partir de aquí r1 y r2 ya no se usan (Non-Lexical Lifetimes)

let r3 = &mut mensaje; // ✅ Préstamo mutable permitido: no hay lectores activos
r3.push_str(" Mundo");
println!("{r3}");
```

---

## 5. Profundización: `String` frente a `&str`

```text
┌─────────────────────────────────┬─────────────────────────────────┐
│             String              │              &str               │
├─────────────────────────────────┼─────────────────────────────────┤
│ • Cadena con propiedad (Owned). │ • Vista prestada (String Slice).│
│ • Vive en el Heap (dinámica).   │ • Apunta a Heap, Stack o .rodata│
│ • Puede crecer y mutar.         │ • Longitud fija, inmutable.     │
│ • Pesa 24 bytes en Stack        │ • Pesa 16 bytes en Stack        │
│   (ptr, len, cap).              │   (ptr, len).                   │
└─────────────────────────────────┴─────────────────────────────────┘
```

### ¿Por qué las funciones idiomáticas reciben `&str`?
Gracias a la **Deref Coercion** de Rust, cualquier función que acepte `&str` puede recibir tanto literales de cadena como referencias a un `String`:

```rust
fn imprimir_mensaje(texto: &str) {
    println!("Mensaje: {texto}");
}

let literal: &str = "Texto en el binario (.rodata)";
let dinamico: String = String::from("Texto dinámico en el Heap");

imprimir_mensaje(literal);    // ✅ Pasa directamente
imprimir_mensaje(&dinamico);  // ✅ Deref coercion automática (&String -> &str)
```
Recibir `&String` en una función se considera un anti-patrón en Rust porque restringe innecesariamente la función a tipos `String`, impidiendo su uso con literales o rodajas (*slices*).
