# 📖 Teoría: 10 - Error Handling (`Option`, `Result` & Familia `unwrap`)

## 1. El Enum `Option<T>` (Presencia o Ausencia)
El tipo `Option<T>` reemplaza completamente la necesidad de valores nulos (`null` / `nil`), obligando a manejar la ausencia de valores en tiempo de compilación.

```rust
pub enum Option<T> {
    Some(T),
    None,
}
```

### Familia de Combinadores de `Option<T>`

| Método | Sintaxis | Descripción & Caso de Uso |
|---|---|---|
| **`.unwrap_or()`** | `opt.unwrap_or("Invitado");` | Extrae el valor si es `Some`; si es `None`, devuelve un valor por defecto precalculado. |
| **`.unwrap_or_else()`** | `opt.unwrap_or_else(\|| f());` | Ejecuta una closure para calcular el valor por defecto de forma perezosa (*Lazy*) solo si era `None`. |
| **`.unwrap_or_default()`** | `opt.unwrap_or_default();` | Devuelve el valor predeterminado estándar del tipo `T` (ej: `0` para enteros, `""` para `String`). |
| **`.expect()`** | `opt.expect("Mensaje");` | Extrae el valor o provoca *panic!* imprimiendo un mensaje explicativo personalizado. |
| **`.map()` / `.and_then()`** | `opt.map(\|s\| s.len());` | Transforma el valor interior si es `Some`, manteniendo la envoltura `Option`. |

---

## 2. El Enum `Result<T, E>` & Operador `?`
El tipo `Result<T, E>` representa el éxito (`Ok(T)`) o una falla esperada (`Err(E)`).

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Propagación con `?`
El operador `?` extrae el valor `T` si es `Ok` o retorna inmediatamente `Err(E)` desde la función actual:

```rust
fn leer_config() -> Result<String, std::io::Error> {
    let contenido = std::fs::read_to_string("app.conf")?; // Retorna error si no existe
    Ok(contenido)
}
```
