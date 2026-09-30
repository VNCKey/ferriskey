# 🗺️ Roadmap: Sesión 14 - Testing y Calidad de Código

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Testing`  
> **Ruta interna**: `AppRoute::TutorialTesting`  
> **Vistas asociadas**: [`src/app/views/testing/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/testing/)

---

## 🎯 Objetivos de Aprendizaje

1. **El Arnés de Pruebas Nativo de Rust (`cargo test`)**:
   - Cómo compilar y ejecutar tests sin dependencias externas.
2. **Pruebas Unitarias (`Unit Tests`)**:
   - El módulo interno `#[cfg(test)] mod tests { ... }`.
   - La macro `#[test]` y las aserciones `assert!`, `assert_eq!`, `assert_ne!`.
   - Manejo de panics esperados con `#[should_panic]`.
3. **Pruebas de Integración (`Integration Tests`)**:
   - La carpeta `tests/` en la raíz del proyecto.
   - Probar la crate como un consumidor externo utilizando solo su API pública.
4. **Pruebas de Documentación (`Doc-Tests`)**:
   - Ejemplos ejecutables dentro de comentarios de documentación triple barra (`///`).
5. **Filtros y Argumentos de Ejecución**:
   - `cargo test -- --nocapture` (ver prints).
   - `cargo test nombre_test` (ejecutar un test específico).
   - `cargo test -- --ignored` (tests lentos marcados con `#[ignore]`).
