# 🗺️ Roadmap: Sesión 11 - Genéricos y Monomorfización

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Generics`  
> **Ruta interna**: `AppRoute::TutorialGenericos`  
> **Vistas asociadas**: [`src/app/views/genericos/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/genericos/)

---

## 🎯 Objetivos de Aprendizaje

1. **Abstracción sobre Tipos con Parámetros Genéricos (`<T>`)**:
   - Funciones genéricas que operan sobre cualquier tipo compatible.
   - Estructuras y enumeraciones genéricas (`struct Punto<T> { x: T, y: T }`).
2. **Restricciones de Comportamiento (*Trait Bounds*)**:
   - Limitar los tipos aceptados a aquellos que implementen comportamientos requeridos (`<T: Display + PartialOrd>`).
   - Sintaxis ergonómica de parámetros con `impl Trait`.
   - Limpieza de firmas complejas mediante la cláusula `where`.
3. **Monomorfización a Costo Cero (*Zero-Cost Abstractions*)**:
   - Comprender cómo el compilador genera copias especializadas de código nativo para cada tipo concreto utilizado durante la compilación.
   - Diferencia fundamental con los genéricos por *type erasure* de Java o punteros void de C.

---

## 🧭 Estructura en FerrisKey
- **`Overview`**: Concepto de código genérico tipado estáticamente.
- **`Funciones`**: Funciones con `<T>`, parámetros y retornos.
- **`Structs`**: Estructuras genéricas con campos adaptables.
- **`Bounds`**: Restricciones de traits y cláusulas `where`.
- **`Code Lab`**: Retos interactivos de implementación genérica.
