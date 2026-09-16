# 📖 Teoría: 05 - Tipos Compuestos (Arreglos, Tuplas & Slices)

## 1. Arreglos (`[T; N]`)
Un Array en Rust almacena valores del **mismo tipo** de forma contigua en la **Stack** con un tamaño **fijo `N`**.

| Aspecto | Sintaxis | Comportamiento / Garantía |
|---|---|---|
| **Tipo Fijo** | `[i32; 5]` | El tipo `T` y la longitud `N` son inmutables en tiempo de compilación. `N` es parte del tipo (`[i32; 5]` $\neq$ `[i32; 10]`). |
| **Acceso Seguro** | `arr[i]` | Acceder fuera de rango (`i >= N`) genera un *panic* seguro en tiempo de ejecución, previniendo *Buffer Overflow*. |
| **Longitud Fija** | `arr.len()` | Devuelve `N`. No puede crecer ni reducirse. |
| **Sintaxis Repetición** | `[0; 100]` | Inicializa un arreglo de 100 ceros instantáneamente en Stack. |
| **Desestructuración** | `let [a, b, ..] = arr;` | Extrae elementos individuales o sub-patrones. |

---

## 2. Tuplas (`(T1, T2, ...)` )
Una Tupla agrupa elementos de **diferentes tipos** con un tamaño fijo en Stack.

| Operación | Sintaxis | Notas |
|---|---|---|
| **Declaración** | `let t = (42, 3.14, "Rust");` | Mezcla tipos heterogéneos. |
| **Acceso por Posición** | `t.0`, `t.1`, `t.2` | Acceso directo indexado por campos. |
| **Desestructuración** | `let (x, y, z) = t;` | Asigna cada componente a una variable independiente. |
| **Tupla Unitaria** | `()` | El tipo *Unit* representa la ausencia de valor (retorno de sentencias). |

---

## 3. Slices (`&[T]`)
Un **Slice** es una vista prestada sobre una secuencia contigua de elementos almacenados en un Array o Vector.

- Consiste en un puntero a los datos y la longitud del slice (sin copia).
- Sintaxis de rango: `&arr[1..4]` (incluye el índice 1, excluye el 4).
