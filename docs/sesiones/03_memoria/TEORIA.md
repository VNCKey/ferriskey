# 📖 Teoría: 03 - Memoria (Ownership, Borrowing & Stack vs Heap)

## 1. El Sistema de Ownership
Ownership es el sistema central de seguridad de memoria de Rust. Se rige por **tres reglas simples** verificadas en tiempo de compilación para garantizar cero fugas de memoria y prevenir el error de doble liberación (*double free*):

### Reglas de Oro de Ownership
| Regla | Sintaxis en Código | Comportamiento en Memoria | Garantía de Seguridad |
|---|---|---|---|
| **Owner** | `let s1 = String::from("a");` | Cada valor tiene una única variable dueña (*Owner*). | Evita punteros colgantes o compartición insegura. |
| **Move** | `let s2 = s1;` | El nuevo dueño es `s2`. `s1` queda inmediatamente inválida. | Previene *double free* al salir de scope. |
| **Drop** | `}` (fin de bloque) | Al salir del scope, el valor se destruye automáticamente (*Drop*). | Cero fugas de memoria (*memory leaks*). |

---

## 2. Memoria Stack vs Heap

```
STACK (Memoria Pila - Tamaños Fijos)        HEAP (Memoria Montículo - Dinámica)
┌──────────────────────────────┐           ┌──────────────────────────────┐
│ s1 [ptr, len: 6, cap: 6]     │───puntero─►│ "Ferris"                     │
└──────────────────────────────┘           └──────────────────────────────┘
```

### A. Stack & Copy
- Tipos escalares con tamaño fijo conocido en compilación (`i32`, `f64`, `bool`, `char`, arreglos fijos).
- Implementan el trait `Copy`. Al asignar `let y = x;`, se realiza una copia rápida bit a bit en Stack. Ambas variables permanecen válidas.

### B. Heap & Move
- Tipos de tamaño dinámico (`String`, `Vec<T>`).
- Al asignar `let s2 = s1;`, se mueven los metadatos de la Stack (ptr, cap, len) a `s2` y `s1` se invalida (*Ownership Move*). NO se duplican los datos en Heap.

---

## 3. Borrowing (Préstamos) & Referencias

Borrowing permite acceder a los datos sin transferir su propiedad mediante referencias (`&`).

| Tipo de Préstamo | Sintaxis | Permisos | Cantidad Simultánea | Regla de Seguridad |
|---|---|---|---|---|
| **Inmutable (`&T`)** | `let r = &s;` | Solo Lectura | Ilimitadas referencias simultáneas | Múltiples lectores pueden observar los datos a la vez. |
| **Mutable (`&mut T`)** | `let r = &mut s;` | Lectura y Escritura | **EXACTAMENTE UNA** a la vez | Acceso exclusivo mientras vive el préstamo. |
| **Exclusividad** | `&` y `&mut` combinados | Conflicto Prohibido | 0 mutables si hay lectores | Previene lecturas inconsistentes y *Data Races*. |

---

## 4. Distinción: `String` vs `&str`

- **`String`**: Cadena dinámica alojada en el Heap, mutable y con propiedad (*Owned String*). Contiene puntero, longitud y capacidad.
- **`&str`**: Slice de cadena prestado (*String Slice*), inmutable, apunta a una secuencia de bytes en memoria (UTF-8).
