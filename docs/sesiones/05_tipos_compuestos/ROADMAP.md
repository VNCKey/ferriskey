# 🗺️ Roadmap: Sesión 05 - Tipos Compuestos y Colecciones

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Tipos Compuestos`  
> **Ruta interna**: `AppRoute::TutorialTiposDatos` / `AppRoute::TutorialColecciones`  
> **Vistas asociadas**: [`src/app/views/tipos_compuestos/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/tipos_compuestos/)

---

## 🎯 Objetivos de Aprendizaje

1. **Arreglos de Tamaño Fijo (`[T; N]`)**:
   - Memoria contigua en el Stack con tamaño fijo e inmutable en tiempo de compilación.
   - Sintaxis de repetición `[valor; N]` y verificación estricta de límites (*Bounds Checking*).
   - Patrones de desestructuración con comodines `_` y rango `..`.

2. **Tuplas Heterogéneas (`(T1, T2, ...)` )**:
   - Agrupación ordenada de valores con tipos dispares en el Stack.
   - Acceso indexado numérico (`tupla.0`, `tupla.1`).
   - El tipo especial Unit `()` como tupla vacía.
   - Desestructuración en variables locales independientes.

3. **Colecciones Dinámicas en el Heap**:
   - Vectores (`Vec<T>`): memoria contigua redimensionable en el Heap.
   - Mecánica de crecimiento y duplicación de capacidad (*Cap*) frente a longitud (*Len*).
   - Métodos esenciales: `push`, `pop`, `insert`, `remove`, `get`.
   - Mapas Hash (`HashMap<K, V>`): indexación asociativa por clave-valor basada en hashes criptográficos SipHash.

4. **Slices (`&[T]` y `&mut [T]`)**:
   - Vistas prestadas de longitud dinámica sin duplicación de memoria (puntero + longitud).
   - Sintaxis de rangos (`&arr[1..4]`, `&arr[..]`, `&arr[2..]`).
   - Slices mutables para modificar subsecciones contiguas del contenedor original.
   - Relación fundamental con `&str` como slice de bytes UTF-8 de una `String`.

---

## 🧭 Estructura de Navegación en FerrisKey

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        BARRA SUPERIOR DE NAVEGACIÓN                    │
├──────────────────────────────────────┬─────────────────────────────────┤
│ 📖 TEORÍA (Lado Izquierdo)           │ 🧪 PRÁCTICA (Lado Derecho)      │
│  [0] Overview (Mapa de Tipos)        │  [5] Code Lab (Retos 1 a 5)     │
│  [1] Array ([T; N])                  │                                 │
│  [2] Tuplas ((T1, T2))               │                                 │
│  [3] Collections (Vec & HashMap)     │                                 │
│  [4] Slices (&[T])                   │                                 │
└──────────────────────────────────────┴─────────────────────────────────┘
```

### 1. Pestañas de Teoría
- **`Overview`**: Resumen conceptual del almacenamiento Stack vs Heap de cada estructura compuesta.
- **`Array`**: Memoria física, desestructuración y prevención de *Buffer Overflows*.
- **`Tuplas`**: Composición heterogénea, patrones y retornos múltiples en funciones.
- **`Collections`**: Simulador interactivo de capacidad del Heap para `Vec<T>` y tablas de `HashMap<K, V>`.
- **`Slices`**: Reglas de indexación y préstamos inmutables/mutables.

### 2. Pestaña de Práctica: Code Lab
- Retos progresivos integrados:
  1. **Reto 1**: Array (inicialización, mutabilidad y pattern matching).
  2. **Reto 2**: Tuplas (acceso por posición y desestructuración).
  3. **Reto 3**: Collections (`Vec` y `HashMap`).
  4. **Reto 4**: Slices (vistas de arreglos y slices de cadenas).
  5. **Reto 5**: Questions (evaluación técnica de 10 preguntas).

---

## ⏱️ Matriz de Competencias

| Concepto Clave | Vista / Componente en App | Nivel | Verificación Práctica |
|---|---|---|---|
| Arreglos Estáticos | `arrays.rs`, `Reto 1` | Básico | Creación con `[0; 4]` y desestructuración con `..` |
| Tuplas Heterogéneas | `tuplas.rs`, `Reto 2` | Básico | Uso de accesos `punto.0` y desempaquetado de tuplas |
| Vectores y Capacidad | `simulador.rs`, `Reto 3` | Intermedio | Observación visual del doblado de capacidad en Heap |
| Tablas Hash | `hashmaps.rs`, `Reto 3` | Intermedio | Inserción y búsqueda por clave en `HashMap` |
| Slices y Vistas Prestadas | `slices.rs`, `Reto 4` | Intermedio | Creación de rangos `&datos[inicio..fin]` sin copias |
