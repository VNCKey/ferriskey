# 📖 Teoría: Sesión 08 - Iteradores y Combinadores

---

## 1. El Trait `Iterator`

En Rust, todos los iteradores implementan el trait `Iterator` definido en la librería estándar:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

- Cada invocación a `.next()` avanza el puntero interno del iterador un elemento.
- Devuelve `Some(valor)` mientras haya elementos disponibles y `None` cuando la secuencia ha concluido.
- El bucle `for` de Rust es azúcar sintáctico que convierte automáticamente la colección en un iterador llamando a `into_iter()` y consumiéndolo con `next()`.

---

## 2. Los Tres Modos de Iteración

| Método | Tipo que entrega | ¿Qué ocurre con la colección? | Caso de Uso |
|---|---|---|---|
| **`.iter()`** | Referencia Inmutable (`&T`) | Permanece intacta y reutilizable. | Inspección, filtrado y lectura compartida. |
| **`.iter_mut()`** | Referencia Mutable (`&mut T`) | Modifica sus elementos in-situ. | Transformación directa de datos sin reasignar memoria. |
| **`.into_iter()`** | Valor Directo (`T`) | **Se consume y destruye** (Ownership Move). | Cuando los elementos deben transferirse a otra estructura. |

```rust
let mut numeros = vec![1, 2, 3];

// 1. iter()
for &n in numeros.iter() {
    println!("{n}");
}

// 2. iter_mut()
for n in numeros.iter_mut() {
    *n *= 2; // Duplica cada elemento in-situ
}

// 3. into_iter()
for n in numeros.into_iter() {
    println!("Consumido: {n}");
}
// println!("{:?}", numeros); // ❌ ERROR: numeros fue consumida
```

---

## 3. Adaptadores Perezosos (*Lazy Adapters*)

Los adaptadores toman un iterador y producen un nuevo iterador con transformaciones aplicadas. **Son perezosos (*lazy*)**: no ejecutan ningún cálculo hasta que se llama a un método consumidor:

- **`.map(|x| ...)`**: Aplica una transformación elemento a elemento.
- **`.filter(|&x| ...)`**: Conserva solo los elementos que cumplan una condición booleana.
- **`.take(n)`**: Limita el recorrido a los primeros `n` elementos.
- **`.enumerate()`**: Empareja cada elemento con su índice `(usize, T)`.

```rust
let numeros = vec![1, 2, 3, 4, 5, 6];

// Nada se ejecuta aquí: solo se define el pipeline
let pares_al_cuadrado = numeros
    .iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x);
```

---

## 4. Métodos Consumidores (*Consumers*)

Son métodos que ejecutan el bucle interno y consumen el iterador para producir un resultado final:

- **`.collect::<Coleccion>()`**: Drena el iterador y transforma los elementos en una colección como `Vec`, `HashSet` o `HashMap`.
- **`.sum()`**: Acumula y suma todos los valores.
- **`.fold(inicio, |acum, x| ...)`**: Reducción funcional acumulativa.
- **`.any(|x| ...)`** y **`.all(|x| ...)`**: Búsqueda booleana con evaluación en cortocircuito.

```rust
let numeros = vec![1, 2, 3, 4, 5, 6];

let resultado: Vec<i32> = numeros
    .iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * 10)
    .collect(); // 👈 El consumidor desata la ejecución del pipeline

println!("{:?}", resultado); // [20, 40, 60]
```

---

## 5. Abstracciones de Costo Cero (*Zero-Cost Abstractions*)

A pesar de la alta expresividad funcional, el compilador de Rust y LLVM aplican técnicas de **fusión de bucles** (*loop fusion*) y vectorización SIMD:
- Se elimina la comprobación de límites (*Bounds Checking*) porque el iterador sabe matemáticamente cuántos elementos existen.
- El código en ensamblador generado suele ser más rápido y compacto que un bucle `for` indexado manualmente con `arr[i]`.
