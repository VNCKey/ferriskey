# 🗺️ Roadmap: Sesión 06 - Control de Flujo

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Control de Flujo`  
> **Ruta interna**: `AppRoute::TutorialControlFlujo`  
> **Vistas asociadas**: [`src/app/views/control_flujo/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/control_flujo/)

---

## 🎯 Objetivos de Aprendizaje

1. **Bifurcaciones Condicionales (`if / else if / else`)**:
   - Evaluación booleana estricta (sin coerción implícita de enteros a booleanos).
   - `if` como expresión generadora de valor (todas las ramas deben devolver el mismo tipo).
2. **Bucles y Repetición (`loop`, `while`, `for`)**:
   - `loop`: repetición infinita y retorno de valores con `break valor;`.
   - `while`: iteración basada en condición booleana.
   - `for`: iteración segura sobre rangos (`1..5` exclusivo vs `1..=5` inclusivo) e iteradores.
   - Control de bucles anidados con etiquetas de bucle (`'outer: loop`).
3. **Pattern Matching Exhaustivo con `match`**:
   - Comprobación exhaustiva obligatoria en compilación.
   - Patrón comodín `_`, alternativas `|`, rangos de patrones `1..=10` y guardas booleanas (*Match Guards* con `if`).
   - `match` como expresión generadora de resultados.

---

## 🧭 Estructura en FerrisKey
- **Pestaña `Overview`**: Mapa conceptual de bifurcaciones y ciclos de CPU.
- **Pestaña `Condicionales`**: Estructuras condicionales y evaluación en cortocircuito (`&&`, `||`).
- **Pestaña `Bucles`**: `loop`, `while`, `for`, `break` y `continue`.
- **Pestaña `Match`**: Pattern matching exhaustivo y match guards.
- **Pestaña `Code Lab`**: 4 Retos guiados (Condicionales, Bucles, Match y Evaluación de 10 preguntas).
