# 📖 Teoría: 07 - Closures (Funciones Anónimas & Captura de Entorno)

## 1. ¿Qué es una Closure?
Un **closure** en Rust es una función anónima que puede capturar variables del ámbito (*Scope*) donde fue declarada. Se escribe usando la sintaxis de tuberías `|params| cuerpo`.

```rust
let sumar_uno = |x: i32| x + 1;
```

---

## 2. Tipos de Captura de Entorno

| Forma de Captura | Sintaxis | Comportamiento en Memoria | Trait Automático |
|---|---|---|---|
| **Lectura Prestada (`&T`)** | `let f = \|x\| x * factor;` | Lee variables del entorno sin transferir propiedad. | `Fn` |
| **Préstamo Mutable (`&mut T`)** | `let mut f = \|\| contador += 1;` | Modifica variables del entorno de forma exclusiva. | `FnMut` |
| **Transferencia (`move`)** | `let f = move \|\| println!("{}", s);` | Toma propiedad exclusiva (*Ownership Move*) de las variables capturadas. | `FnOnce` |

---

## 3. Los Traits de Closures (`Fn`, `FnMut`, `FnOnce`)
- **`FnOnce`**: Consume las variables capturadas. Puede ser llamada **al menos una vez**.
- **`FnMut`**: Modifica el entorno capturado. Puede ser llamada **múltiples veces**.
- **`Fn`**: Lee el entorno inmutablemente. Puede ser llamada **múltiples veces de forma segura y concurrente**.
