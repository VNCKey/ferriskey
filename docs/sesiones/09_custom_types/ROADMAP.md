# 🗺️ Roadmap: Sesión 09 - Tipos Personalizados (Structs & Enums)

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Custom Types`  
> **Ruta interna**: `AppRoute::TutorialStructs`  
> **Vistas asociadas**: [`src/app/views/structs/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/structs/)

---

## 🎯 Objetivos de Aprendizaje

1. **Modelado con Estructuras (`struct`)**:
   - Structs clásicos con campos nombrados.
   - *Tuple Structs* para crear tipos de dominio fuertes sin sobrecoste (*Newtype Pattern*).
   - *Unit-like Structs* para marcadores de estado y rasgos sin datos.
2. **Implementación de Comportamiento (`impl`)**:
   - Constructores y funciones asociadas sin instancia (`Self::new`).
   - Métodos de solo lectura (`&self`).
   - Métodos de mutación de estado (`&mut self`).
   - Métodos que consumen la instancia (`self`).
3. **Enumeraciones Algebraicas (`enum`)**:
   - Variantes con datos asociados (tupla-variantes y struct-variantes).
   - Representación en memoria: etiqueta discriminante (*Tag*) + unión del tamaño del campo más grande.
4. **Desestructuración con `match` e `if let`**:
   - Extracción segura de datos encapsulados en variantes.

---

## 🧭 Estructura en FerrisKey
- **`Overview`**: Concepto de tipos de datos algebraicos (ADT).
- **`Definición`**: Structs con campos nombrados, Tuple Structs y bloques `impl`.
- **`Enums`**: Variantes con datos y tipos suma.
- **`Pattern Matching`**: Desestructuración con `match` e `if let`.
- **`Traits`**: Integración con derivaciones automáticas `#[derive(...)]`.
- **`Code Lab`**: Retos interactivos de construcción de modelos de dominio.
