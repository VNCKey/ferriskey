# 📖 Teoría: Sesión 05 - Tipos Compuestos y Colecciones

---

## 1. Arreglos (`[T; N]`): Colecciones de Tamaño Fijo en el Stack

Un **Array** en Rust agrupa múltiples valores del **mismo tipo** de forma contigua en memoria, con una longitud fija conocida en tiempo de compilación.

```rust
let numeros: [i32; 5] = [10, 20, 30, 40, 50];
let ceros = [0; 100]; // Inicializa 100 enteros con valor 0 instantáneamente
```

### Características Técnicas del Array
1. **El tamaño `N` es parte del tipo**: Un arreglo `[i32; 4]` es un tipo de dato completamente distinto a `[i32; 5]`. No pueden asignarse entre sí.
2. **Memoria en el Stack**: Ocupa exactamente `N * size_of::<T>()` bytes contiguos en el Stack Frame.
3. **Bounds Checking (Verificación de Límites)**: Acceder a un índice que exceda o iguale la longitud `N` causa un `panic!` en tiempo de ejecución de manera segura, impidiendo la lectura o sobreescritura de memoria adyacente (*Buffer Overflow*).
4. **Desestructuración con Patrones**:
   ```rust
   let notas = [10, 12, 15, 18];
   let [primera, _, .., ultima] = notas;
   // primera = 10, ultima = 18 (el resto se ignora con ..)
   ```

---

## 2. Tuplas: Colecciones Heterogéneas

Una **Tupla** permite agrupar varios valores de **diferentes tipos** en una única estructura compuesta con longitud fija en el Stack.

```rust
let usuario: (&str, u32, bool) = ("Ferris", 25, true);
```

### Acceso a Campos y Modificación
- **Acceso por Posición**: Se utiliza la notación de punto seguida del índice:
  ```rust
  let nombre = usuario.0;
  let edad = usuario.1;
  let activo = usuario.2;
  ```
- **Mutabilidad**: Si se declara con `let mut`, los valores individuales pueden modificarse, pero la estructura no puede ganar ni perder campos.
- **Desestructuración**:
  ```rust
  let (nombre, edad, activo) = usuario;
  let (primero, .., ultimo) = (1, 2, 3, 4, 5);
  ```
- **El Tipo Unitario `()`**: Una tupla sin elementos se denomina **Unit** y representa una expresión que no retorna ningún valor útil.

---

## 3. Colecciones Dinámicas en el Heap: `Vec<T>` y `HashMap<K, V>`

A diferencia de los arreglos fijos, las colecciones estándar alojan sus datos de forma dinámica en el Heap.

### A. Vectores (`Vec<T>`)
Un vector es una secuencia dinámica y contigua de elementos del mismo tipo:

```rust
let mut numeros = Vec::new();
numeros.push(10);
numeros.push(20);
```
O con la macro de inicialización:
```rust
let mut niveles = vec![1, 2, 4, 8];
```

#### Simulador de Capacidad del Heap (Capacidad vs. Longitud)
- **`len` (Longitud)**: Número de elementos actualmente almacenados en el vector.
- **`cap` (Capacidad)**: Cantidad total de elementos que el vector puede almacenar en su búfer actual del Heap antes de tener que solicitar una reasignación al sistema operativo.
- **Mecánica de Reasignación**: Cuando `len` supera `cap`, Rust reserva un nuevo bloque en el Heap con el **doble de capacidad** ($1 \rightarrow 2 \rightarrow 4 \rightarrow 8 \rightarrow 16 \dots$), copia los elementos anteriores y libera el bloque antiguo.

### B. Mapas Hash (`HashMap<K, V>`)
Estructura asociativa que almacena pares clave-valor utilizando una tabla hash:

```rust
use std::collections::HashMap;

let mut puntuaciones = HashMap::new();
puntuaciones.insert("Azul", 10);
puntuaciones.insert("Rojo", 50);

let puntaje = puntuaciones.get("Azul"); // Retorna Option<&i32>
```

---

## 4. Slices (`&[T]` y `&mut [T]`): Vistas Prestadas

Un **Slice** es una referencia no dueña (*borrowed view*) a una porción contigua de memoria dentro de un Array o Vector.

```text
ARRAY EN EL STACK:  [ 10,  20,  30,  40,  50 ]
Índices:               0    1    2    3    4
                            ▲              ▲
                            │              │
SLICE: &arr[1..4] ──────────┴──────────────┘
(ptr: &arr[1], len: 3 elementos: [20, 30, 40])
```

- **Sintaxis de Rangos**:
  - `&arr[1..4]`: Desde el índice 1 (inclusivo) hasta el 4 (exclusivo).
  - `&arr[..3]`: Desde el inicio hasta el índice 3.
  - `&arr[2..]`: Desde el índice 2 hasta el final.
  - `&arr[..]`: Todos los elementos.
- **Slices Mutables (`&mut [T]`)**: Permiten modificar los elementos de la sección prestada sin reasignar la colección completa:
  ```rust
  let mut valores = [10, 20, 30, 40];
  let parte = &mut valores[1..3];
  parte[0] = 99; // valores ahora es [10, 99, 30, 40]
  ```
- **Relación con `&str`**: Un `&str` es esencialmente un slice de bytes (`&[u8]`) que garantiza representar caracteres UTF-8 válidos en todo momento.
