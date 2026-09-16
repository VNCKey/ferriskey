# 📖 Teoría: 08 - Iteradores & Pipeline Perezoso

## 1. El Trait `Iterator` & Evaluación Perezosa (*Lazy Evaluation*)
En Rust, un **iterador** es una abstracción de cero costo sobre una secuencia contigua o calculada de elementos. Los iteradores son **perezosos (*Lazy*)**: los adaptadores como `.map()` o `.filter()` NO realizan ningún cálculo hasta que un **consumidor** (como `.collect()`, `.sum()`, `.for_each()`) fuerza la ejecución de la cadena.

---

## 2. Los 3 Modos de Iteración

| Método | Tipo de Elemento | Transferencia de Propiedad | Uso Común |
|---|---|---|---|
| **`.iter()`** | `&T` | Préstamo inmutable (Lectura) | Inspección de elementos sin modificar ni destruir la colección. |
| **`.iter_mut()`** | `&mut T` | Préstamo mutable (Escritura) | Modificación in-place de elementos dentro de la colección. |
| **`.into_iter()`** | `T` | Transferencia (*Move*) | Consume la colección original y toma propiedad de sus elementos. |

---

## 3. Adaptadores vs Consumidores

```
[Origen: vec!] ──► .iter() ──► .filter(es_par) ──► .map(cuadrado) ──► .collect::<Vec<_>>()
(Colección)       (Iterador)     (Adaptador)        (Adaptador)           (Consumidor Final)
```

- **Adaptadores (Transformadores)**: Retornan una nueva estructura de iterador. No evalúan elementos por sí mismos.
  - `.filter(|&x| x > 0)`: Filtra elementos por una condición booleana.
  - `.map(|x| x * 2)`: Transforma cada elemento.
  - `.enumerate()`: Añade el índice del elemento en una tupla `(indice, val)`.
  - `.take(n)`: Limita el número de elementos producidos.
- **Consumidores (Evaluadores)**: Consumen la secuencia y producen un resultado final.
  - `.collect()`: Recoge elementos en una colección elegida (`Vec`, `HashMap`, etc.).
  - `.sum()`, `.product()`: Acumula numéricamente.
  - `.find(|&x| x == target)`: Devuelve `Option<T>` con la primera coincidencia.
