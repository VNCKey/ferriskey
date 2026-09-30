# 🗺️ Roadmap: Sesión 13 - Tiempos de Vida (Lifetimes `'a`)

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Lifetimes`  
> **Ruta interna**: `AppRoute::TutorialLifetimes`  
> **Vistas asociadas**: [`src/app/views/lifetimes/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/lifetimes/)

---

## 🎯 Objetivos de Aprendizaje

1. **Comprensión del *Borrow Checker* y Tiempos de Vida**:
   - Por qué cada referencia en Rust tiene un tiempo de vida (*lifetime*).
   - Prevención de referencias colgantes (*Dangling References*) y punteros salvajes en tiempo de compilación.
2. **Sintaxis de Parámetros de Lifetime Genéricos (`'a`)**:
   - Relacionar los tiempos de vida de múltiples parámetros de entrada con el valor de retorno.
   - Anotación en funciones (`fn mayor<'a>(x: &'a str, y: &'a str) -> &'a str`).
3. **Reglas de Elisión de Tiempos de Vida (*Lifetime Elision Rules*)**:
   - Los 3 patrones deterministas que permiten al compilador inferir lifetimes sin anotaciones manuales.
4. **Lifetimes en Estructuras y Métodos**:
   - `struct Libro<'a> { titulo: &'a str }`.
   - Implementación de bloques `impl<'a>`.
5. **El Lifetime Estático (`'static`)**:
   - Datos almacenados en el binario y variables que viven durante toda la ejecución.

---

## 🧭 Estructura en FerrisKey
- **`Concepto & Dangling References`**: Visualización de ámbitos y por qué falla el préstamo.
- **`Funciones con 'a`**: Relación de préstamos entre entradas y salidas.
- **`Elision Rules`**: Las 3 reglas automáticas de Rust.
- **`Structs con Lifetimes`**: Guardar referencias dentro de estructuras de datos.
- **`Code Lab`**: Retos guiados de compilación segura de referencias.
