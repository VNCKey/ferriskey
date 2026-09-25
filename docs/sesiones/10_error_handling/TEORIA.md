# 📖 Teoría: Sesión 10 - Manejo Idiomático de Errores

---

## 1. La Filosofía de Rust frente a los Errores

A diferencia de Java, Python o C++ que utilizan excepciones lanzadas en tiempo de ejecución, o de C que utiliza punteros `NULL` y códigos de retorno numéricos no forzados:
- **Rust no tiene excepciones**: Todo fallo que pueda ocurrir forma parte explícita de la firma de tipos de la función.
- **Rust no tiene `NULL`**: Se elimina la categoría completa de vulnerabilidades por desreferenciación de punteros nulos (*NullPointerException*).

Rust clasifica los errores en dos grandes categorías:
1. **Errores Recuperables**: Situaciones normales de la vida real (un archivo no existe, la red falló, el usuario escribió letras en vez de números). Se representan con **`Result<T, E>`**.
2. **Errores Irrecuperables**: Errores de lógica del programador o estados corruptos del sistema (acceder a un índice de array fuera de rango, división exacta por cero). Se manifiestan con **`panic!`**.

---

## 2. El Enum `Option<T>` (Ausencia Segura de Valor)

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Para acceder al valor contenido en un `Option`:
- **`match` o `if let`**: La forma más segura y explícita.
- **`.unwrap()`**: Obtiene el valor si es `Some`, o provoca un `panic!` si es `None` (usar solo en pruebas o ejemplos rápidos).
- **`.expect("mensaje")`**: Igual que `unwrap`, pero imprime un mensaje descriptivo si falla.
- **`.unwrap_or(defecto)`**: Devuelve el valor contenido o un valor predeterminado si es `None`.
- **`.map(|v| ...)`**: Transforma el valor interior si existe, conservando `None` si estaba ausente.

---

## 3. El Enum `Result<T, E>` (Operaciones Fallibles)

```rust
enum Result<T, E> {
    Ok(T),  // La operación tuvo éxito y contiene el resultado T
    Err(E), // La operación falló y contiene el error descriptivo E
}
```

### El Atributo `#[must_use]`
`Result` está anotado con `#[must_use]`. Si una función retorna un `Result` y el desarrollador ignora el valor devuelto sin gestionarlo, el compilador genera inmediatamente una advertencia obligatoria.

---

## 4. El Operador de Propagación de Errores `?`

El operador interrogación `?` es la herramienta más idiomática para trabajar con funciones que pueden fallar:

```rust
use std::fs::File;
use std::io::{self, Read};

fn leer_nombre_archivo(ruta: &str) -> Result<String, io::Error> {
    let mut archivo = File::open(ruta)?; // Si falla, retorna Err(e) inmediatamente de la función
    let mut contenido = String::new();
    archivo.read_to_string(&mut contenido)?; // Si falla, retorna Err(e)
    Ok(contenido) // Si todo salió bien, retorna Ok con el contenido
}
```

- Si el resultado es `Ok(valor)`, la expresión con `?` se evalúa a `valor` y el flujo continúa.
- Si el resultado es `Err(error)`, la función actual **retorna anticipadamente** ese mismo `Err(error)`.
- El operador `?` también funciona sobre `Option<T>`, desempaquetando `Some(v)` o retornando `None` anticipadamente.
