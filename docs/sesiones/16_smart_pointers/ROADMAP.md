# 🗺️ Roadmap: Sesión 16 - Punteros Inteligentes (Smart Pointers)

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Smart Pointers`  
> **Ruta interna**: `AppRoute::TutorialSmartPointers`  
> **Vistas asociadas**: [`src/app/views/smart_pointers/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/smart_pointers/)

---

## 🎯 Objetivos de Aprendizaje

1. **Memoria en el Heap con `Box<T>`**:
   * Alocación dinámica de datos cuando su tamaño no se conoce en compilación (*Recursive Types*).
   * Transferencia de propiedad sin copias pesadas en el Stack.
2. **Propiedad Compartida con `Rc<T>` y `Arc<T>`**:
   * *Reference Counting* (conteo de referencias) monohilo (`Rc<T>`).
   * *Atomic Reference Counting* multihilo (`Arc<T>`).
3. **Mutabilidad Interior (*Interior Mutability*)**:
   * `Cell<T>` para tipos `Copy`.
   * `RefCell<T>` con comprobación dinámica de préstamos en tiempo de ejecución (`borrow()` y `borrow_mut()`).
4. **Los Traits Fundamentales: `Deref` y `Drop`**:
   * Desreferenciación implícita transparente con `Deref`.
   * Destrucción determinista de recursos con `Drop`.
