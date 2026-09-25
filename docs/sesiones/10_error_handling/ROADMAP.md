# 🗺️ Roadmap: Sesión 10 - Manejo Idiomático de Errores

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Error Handling`  
> **Ruta interna**: `AppRoute::TutorialErrores` / `AppRoute::TutorialEnums`  
> **Vistas asociadas**: [`src/app/views/enums/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/enums/) y [`src/app/views/errores.rs`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/errores.rs)

---

## 🎯 Objetivos de Aprendizaje

1. **La Filosofía de Errores en Rust (Sin Excepciones ni Null)**:
   - Comprender por qué Rust no tiene `null` (el llamado "error del billón de dólares" de Tony Hoare) ni excepciones invisibles en tiempo de ejecución.
2. **El Tipo `Option<T>`**:
   - Modelado explícito de presencia (`Some(valor)`) o ausencia (`None`).
   - Métodos combinadores clave: `unwrap_or`, `unwrap_or_else`, `map`, `and_then`.
3. **El Tipo `Result<T, E>` y Operaciones Fallibles**:
   - Modelado de éxito (`Ok(valor)`) o fallo recuperable (`Err(error)`).
   - El atributo `#[must_use]`: el compilador emite una advertencia si se ignora el resultado de una operación que puede fallar.
4. **Propagación Ergonómica con el Operador `?`**:
   - Desenvolver `Ok(v)` o retornar anticipadamente `Err(e)` hacia la función llamadora.
5. **Errores Irrecuperables con `panic!`**:
   - Distinguir fallos del sistema (*bugs*, aserciones rotas) de errores esperados del negocio.
   - Mecánica de *Stack Unwinding* vs aborto inmediato.

---

## 🧭 Estructura en FerrisKey
- **`Overview`**: La dualidad entre `Option` y `Result`.
- **`Option`**: Métodos y patrones para evitar `null`.
- **`Result`**: Manejo de errores recuperables y tipado de errores.
- **`Desempaquetado`**: `unwrap`, `expect`, `unwrap_or` y el operador `?`.
- **`Code Lab`**: Retos interactivos de propagación y transformación de errores.
