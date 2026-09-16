# 📖 Teoría: 06 - Control de Flujo (Condicionales, Bucles & Match)

## 1. Condicionales & Expresiones `if / else`
En Rust, `if` es una expresión: puede devolver un valor asignable a una variable.

```rust
let condicion = true;
let numero = if condicion { 5 } else { 6 }; // Ambas ramas deben devolver el mismo tipo
```

---

## 2. Bucles (`loop`, `while`, `for`)
- **`loop`**: Bucle infinito expresivo. Puede devolver un valor al romper con `break valor;`.
- **`while`**: Se ejecuta mientras la condición sea verdadera.
- **`for`**: Forma más segura e idiomática para iterar sobre rangos o colecciones (`for i in 0..5`).

---

## 3. Pattern Matching Exhaustivo (`match`)
La expresión `match` compara un valor contra múltiples patrones y ejecuta el primer brazo coincidente. El compilador **exige exhaustividad total** (cubrir todos los casos posibles).

| Patrón | Ejemplo de Sintaxis | Reglas & Descripción |
|---|---|---|
| **Literal Exacto** | `1 => println!("Uno"),` | Coincidencia exacta con un valor explícito (números, caracteres, strings). |
| **Rangos Inclusivos** | `0..=12 => println!("Niño"),` | Coincidencia inclusiva dentro de un intervalo numérico. |
| **Patrón Múltiple (OR)** | `'a' \| 'e' \| 'i' => ...` | Evalúa si coincide con cualquiera de los patrones separados por `\|`. |
| **Comodín `_`** | `_ => println!("Otro"),` | Captura cualquier caso no listado previamente para cumplir exhaustividad. |
| **Match Guards (`if`)** | `n if n % 2 == 0 => ...` | Añade una condición booleana adicional al patrón evaluado. |

---

## 4. `if let` y `while let`
Permiten evaluar un único patrón específico omitiendo la exhaustividad obligatoria de `match`:

```rust
let valor: Option<i32> = Some(7);

if let Some(x) = valor {
    println!("Valor extraído: {}", x);
}
```
