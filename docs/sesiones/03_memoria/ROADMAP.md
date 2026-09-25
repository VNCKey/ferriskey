# 🗺️ Roadmap: Sesión 03 - Memoria y Ownership

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Memoria`  
> **Ruta interna**: `AppRoute::TutorialOwnership` / `AppRoute::TutorialMemoria`  
> **Vistas asociadas**: [`src/app/views/memoria/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/memoria/)

---

## 🎯 Objetivos de Aprendizaje

1. **Arquitectura Física de la Memoria (Stack vs. Heap)**:
   - Identificar cómo se asigna memoria en el **Stack** (rápida, contigua, LIFO, tamaños conocidos en tiempo de compilación).
   - Comprender la asignación en el **Heap** (dinámica, administrada mediante punteros, coste de llamadas al sistema operativo).

2. **Las Tres Leyes Fundamentales del Ownership**:
   - *Ley 1*: Cada valor en Rust tiene una única variable responsable denominada su **propietario** (*Owner*).
   - *Ley 2*: Solo puede existir **un único propietario activo a la vez**.
   - *Ley 3*: Cuando el propietario sale de su ámbito (*scope*), el valor se destruye y su memoria se libera inmediatamente mediante el mecanismo `Drop` (filosofía RAII).

3. **Semántica de Copia (Copy) vs. Semántica de Movimiento (Move)**:
   - Tipos primitivos con el trait `Copy` en el Stack (copia automática bit a bit sin invalidar el origen).
   - Tipos dinámicos en el Heap (`String`, `Vec<T>`) y transferencia de propiedad (*Move*): invalidación instantánea de la variable origen para prevenir el error catastrófico de *Double Free*.

4. **El Sistema de Préstamos (Borrowing) y Referencias**:
   - Referencias compartidas e inmutables (`&T`): múltiples lectores concurrentes.
   - Referencias exclusivas y mutables (`&mut T`): modificación segura con garantía de acceso único.
   - Exclusión mutua garantizada por el **Borrow Checker**: prohibición estricta de mezclar lectores activos con un escritor mutable para eliminar *Data Races*.
   - Ciclos de vida no léxicos (*Non-Lexical Lifetimes - NLL*).

5. **Profundización en Texto: `String` vs. `&str`**:
   - Estructura física de un `String`: tripleta en el Stack `[puntero, longitud, capacidad]` apuntando a un buffer en el Heap.
   - Estructura física de un `&str`: tupla `[puntero, longitud]` representando un *Slice* prestado de bytes UTF-8 válidos.
   - Coerción de desreferenciación (*Deref Coercion*) al pasar `&String` a parámetros de tipo `&str`.

---

## 🧭 Estructura de Navegación en FerrisKey

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        BARRA SUPERIOR DE NAVEGACIÓN                    │
├──────────────────────────────────────┬─────────────────────────────────┤
│ 📖 TEORÍA (Lado Izquierdo)           │ 🧪 PRÁCTICA (Lado Derecho)      │
│  [0] Overview (Mapa de Memoria)      │  [2] Code Lab (Retos 1 a 5)     │
│  [1] String vs &str (Física & Texto) │                                 │
└──────────────────────────────────────┴─────────────────────────────────┘
```

### 1. Pestañas de Teoría
- **`Overview` (`overview.rs`, `borrowing.rs`, `ownership.rs`, `stack_copy.rs`, `heap_move.rs`)**:
  - Mapa interactivo de la memoria: el recorrido completo de un valor.
  - Reglas de oro del Ownership y prevención de *Double Free*.
  - Tabla comparativa de permisos entre referencias `&T` y `&mut T`.
  - Explicación visual del paso de variables entre Stack Frames.
- **`String vs &str` (`strings.rs`)**:
  - Anatomía interna y diagramas de distribución de bytes.
  - Creación y crecimiento dinámico con `push_str`.
  - Técnicas idiomáticas para aceptar cadenas en funciones sin transferir propiedad.

### 2. Pestaña de Práctica: Code Lab
- Editor integrado en tiempo real conectado al drawer de retos progresivos:
  1. **Reto 1**: Stack & Copy Semantics (`let x = 42; let y = x;`).
  2. **Reto 2**: String, Heap & Move Semantics (`String::from`, reubicación en memoria y descarte de variable previa).
  3. **Reto 3**: Ownership (propietario único y ciclo de vida de los scopes).
  4. **Reto 4**: Borrowing (referencias inmutables múltiples vs mutabilidad exclusiva).
  5. **Reto 5**: String vs &str (diseño de APIs eficientes con slices prestados).

---

## ⏱️ Matriz de Competencias

| Concepto Clave | Vista / Componente en App | Nivel | Verificación Práctica |
|---|---|---|---|
| Stack vs. Heap | `overview.rs`, `stack_copy.rs` | Básico | Identificación de tipos fijos vs dinámicos |
| Move Semantics | `heap_move.rs`, `Reto 2` | Fundamental | Detección de error de compilación por uso de variable movida |
| Reglas de Ownership | `ownership.rs`, `Reto 3` | Fundamental | Liberación determinista de recursos sin recolector de basura |
| Borrowing y Exclusividad | `borrowing.rs`, `Reto 4` | Intermedio | Concurrencia de lectores y aislamiento de escritor mutable |
| String vs &str | `strings.rs`, `Reto 5` | Intermedio | Creación de funciones con parámetros `&str` y slices |
