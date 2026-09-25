# 🗺️ Roadmap: Sesión 02 - Conceptos Fundamentales

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Conceptos`  
> **Ruta interna**: `AppRoute::Comenzando`  
> **Vistas asociadas**: [`src/app/views/conceptos/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/conceptos/)

---

## 🎯 Objetivos de Aprendizaje

1. **Mecánicas Centrales de Variables**:
   - Comprender la inmutabilidad por defecto (`let`) y por qué previene efectos secundarios indeseados y condiciones de carrera.
   - Declarar mutabilidad explícita y controlada con `mut`.
   - Dominar el mecanismo de **Shadowing** (sombreado de variables) y sus diferencias con la mutación.
   - Distinguir constantes en tiempo de compilación (`const`) y memoria global estática (`static`).

2. **Gestión de Memoria en el Stack y Ámbitos (Scopes)**:
   - Entender cómo los bloques `{}` delimitan el ciclo de vida (*lifetime/scope*) físico en el Stack Frame.
   - Observar la liberación automática de recursos al salir de un bloque mediante la semántica RAII (*Resource Acquisition Is Initialization*).

3. **Sistema de Tipos Primitivos Escalares**:
   - Clasificar y operar con los tipos escalares fundamentales:
     - **Enteros con y sin signo**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`.
     - **Flotantes IEEE 754**: `f32` y `f64`.
     - **Booleanos**: `bool` (`true`, `false`).
     - **Caracteres Unicode**: `char` (valor escalar Unicode de 4 bytes / 32 bits).
   - Conversiones de tipos explícitas (*casting*) con la palabra clave `as`.
   - Comportamiento ante el desbordamiento de enteros (*Integer Overflow*) en modos `debug` vs `release`.

4. **Expresiones vs. Sentencias (Statements vs. Expressions)**:
   - Reconocer la diferencia física entre una sentencia (que termina en `;` y no produce valor, retornando el tipo unit `()`) y una expresión (que evalúa y produce un valor).
   - Utilizar bloques como expresiones que devuelven resultados.

5. **Funciones y Metaprogramación con Macros**:
   - Declarar funciones con sintaxis `fn`, parámetros fuertemente tipados y tipos de retorno `->`.
   - Diferenciar retornos implícitos (última expresión sin `;`) del retorno explícito `return`.
   - Comprender las macros más utilizadas: `println!`, `print!`, `eprintln!`, `format!`, `panic!`, `todo!`, `unimplemented!`.
   - Conocer las cadenas de formato (*Format Strings*) y especificadores (`{}`, `{:?}`, `{:#?}`, `{:x}`).

---

## 🧭 Estructura de Navegación en FerrisKey

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        BARRA SUPERIOR DE NAVEGACIÓN                    │
├──────────────────────────────────────┬─────────────────────────────────┤
│ 📖 TEORÍA (Lado Izquierdo)           │ 🧪 PRÁCTICA (Lado Derecho)      │
│  [7] Core (Mecánicas Centrales)      │  [0] Code Lab (Retos 1 a 8)     │
│  [4] Data Types (Tipos Primitivos)   │                                 │
│  [5] Macros (Declarativas & Format)  │                                 │
└──────────────────────────────────────┴─────────────────────────────────┘
```

### 1. Pestañas de Teoría
- **`Core` (`mutabilidad.rs`, `scopes.rs`, `globales.rs`, `statements.rs`)**:
  - Inmutabilidad por defecto como garantía de concurrencia y seguridad.
  - Tabla comparativa `let` vs `let mut` vs `const` vs `static`.
  - Mecánica de Shadowing: reutilización de nombres y transformación de tipos en la memoria.
  - Creación y destrucción de variables en los Stack Frames.
- **`Data Types` (`primitivos.rs`)**:
  - Desglose completo de enteros: rangos mínimos, máximos, representaciones literales (`0x`, `0o`, `0b`, `_`) y control de desbordamiento.
  - Flotantes de precisión simple y doble, límites e indeterminaciones (`NaN`).
  - Caracteres Unicode de 4 bytes (soporte nativo de alfabetos globales y emojis 🦀).
  - Casting seguro y advertencias de pérdida de precisión con `as`.
- **`Macros` (`mod.rs`)**:
  - Tabla completa de Declarative Macros de la librería estándar.
  - Formateadores posicionales, nombrados y de depuración (`Display` vs `Debug`).
  - Diagrama de ferrocarril interactivo (*Railroad*) para la sintaxis de sentencias `let`.

### 2. Pestaña de Práctica: Code Lab
- Consola de ejecución en tiempo real (`cargo run` / `rustc`) con drawer de retos guiados:
  1. **Reto 1**: Variables paso a paso (`let`, `let mut`, `shadowing`).
  2. **Reto 2**: Macros (`println!`, `eprintln!`, `format!`).
  3. **Reto 3**: Blocks & Scope (Stack frames y ámbitos `{}`).
  4. **Reto 4**: Statements & Expressions (el papel del punto y coma `;`).
  5. **Reto 5**: Data Types (enteros, flotantes, bool, char y casting).
  6. **Reto 6**: Comments & Docs (comentarios de código y `cargo doc`).
  7. **Reto 7**: Functions (parámetros, tipos de retorno y expresiones).
  8. **Reto 8**: Questions (evaluación técnica interactiva de 15 preguntas).

---

## ⏱️ Matriz de Competencias

| Concepto Clave | Vista / Componente en App | Nivel | Verificación Práctica |
|---|---|---|---|
| Inmutabilidad y `mut` | `mutabilidad.rs` | Básico | Corrección de errores de compilación por reasignación inmutable |
| Shadowing de Variables | `mutabilidad.rs`, `Reto 1` | Básico | Transformación de tipo de dato reutilizando el identificador |
| Stack Frames y Scopes | `scopes.rs`, `Reto 3` | Básico | Acceso a variables en bloques anidados y detección de fuera de ámbito |
| Tipos Numéricos y Límites | `primitivos.rs`, `Reto 5` | Intermedio | Selección adecuada del tipo según memoria (`u8` vs `usize`) |
| Expresiones vs Sentencias | `statements.rs`, `Reto 4` | Intermedio | Asignación de valores directamente desde bloques `{}` sin `return` |
| Sintaxis de Funciones | `funciones.rs`, `Reto 7` | Básico | Definición de firmas de función con tipos explícitos |
| Metaprogramación con Macros | `mostrar_contenido_macros`, `Reto 2` | Intermedio | Interpolación de variables y formateo avanzado en terminal |
