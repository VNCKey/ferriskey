# 🗺️ Roadmap: Sesión 12 - Traits y Polimorfismo

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Traits`  
> **Ruta interna**: `AppRoute::TutorialTraits`  
> **Vistas asociadas**: [`src/app/views/traits/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/traits/)

---

## 🎯 Objetivos de Aprendizaje

1. **Definición e Implementación de Traits**:
   - Contratos compartidos de interfaces (`trait Imprimible { fn imprimir(&self); }`).
   - Implementaciones predeterminadas de métodos en traits.
2. **Traits Estándar del Ecosistema**:
   - `Display` y `Debug` para formateo en consola.
   - `Clone` y `Copy` para duplicación en memoria.
   - `From` e `Into` para conversiones idiomáticas entre tipos.
   - Derivación automática mediante el macro de atributos `#[derive(...)]`.
3. **Despacho Estático vs. Despacho Dinámico**:
   - **Static Dispatch**: Genéricos (`<T: Trait>`) resueltos en compilación mediante monomorfización (máxima velocidad, *inlining*).
   - **Dynamic Dispatch**: Objetos Trait (`&dyn Trait` o `Box<dyn Trait>`) resueltos en tiempo de ejecución mediante tablas de punteros virtuales (*vtables* / *fat pointers*). Permiten almacenar colecciones heterogéneas que comparten un mismo trait.

---

## 🧭 Estructura en FerrisKey
- **`Overview`**: Concepto de interfaces y polimorfismo en Rust.
- **`Estándar`**: Traits estándar (`Display`, `Clone`, `Default`) y `#[derive]`.
- **`Polimorfismo`**: Despacho estático vs dinámico con `dyn Trait` y vtables.
- **`Code Lab`**: Retos interactivos de implementación y despacho polimórfico.
