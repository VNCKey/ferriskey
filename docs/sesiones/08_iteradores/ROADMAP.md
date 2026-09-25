# 🗺️ Roadmap: Sesión 08 - Iteradores y Combinadores

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Iteradores`  
> **Ruta interna**: `AppRoute::TutorialIteradores`  
> **Vistas asociadas**: [`src/app/views/iteradores/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/iteradores/)

---

## 🎯 Objetivos de Aprendizaje

1. **El Trait `Iterator` y el Método `next()`**:
   - Comprender cómo un iterador entrega secuencialmente elementos envueltos en `Option<Self::Item>`.
2. **Los Tres Modos Fundamentales de Iteración**:
   - `.iter()`: Préstamo inmutable (`&T`), la colección se conserva intacta.
   - `.iter_mut()`: Préstamo mutable (`&mut T`), modificación in-situ de los elementos.
   - `.into_iter()`: Movimiento por valor (`T`), consume la colección original.
3. **Adaptadores Perezosos (Lazy Iterator Adapters)**:
   - Encadenar transformaciones con `.map()`, `.filter()`, `.take()`, `.enumerate()`.
   - Entender que los adaptadores no ejecutan trabajo hasta que un consumidor los solicita (*Lazy Evaluation*).
4. **Métodos Consumidores (Consumers)**:
   - Drenar el flujo y recolectar resultados en nuevas estructuras con `.collect()`.
   - Reducciones y agregaciones con `.sum()`, `.fold()`, `.count()`, `.any()`, `.all()`.
5. **Abstracción de Costo Cero (*Zero-Cost Abstractions*)**:
   - Verificar cómo el compilador desenrolla y optimiza el pipeline de iteradores en ensamblador equivalente o superior a un bucle `for` manual.

---

## 🧭 Estructura en FerrisKey
- **`Overview`**: Concepto de secuencias perezosas y el trait `Iterator`.
- **`Modos`**: Comparativa `.iter()`, `.iter_mut()`, `.into_iter()`.
- **`Adaptadores`**: Combinadores funcionales (`map`, `filter`, `take`).
- **`Consumidores`**: Recolección de datos con `collect` y agregaciones.
- **`Pipeline`**: Visualización de optimizaciones de bajo nivel de LLVM.
- **`Code Lab`**: Retos interactivos de construcción y optimización de pipelines.
