# 🗺️ Roadmap: Sesión 18 - Asincronía (Async / Await con Tokio)

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Async / Await`  
> **Ruta interna**: `AppRoute::TutorialAsync`  
> **Vistas asociadas**: [`src/app/views/asincronia/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/asincronia/)

---

## 🎯 Objetivos de Aprendizaje

1. **Fundamentos del Modelo Asíncrono de Rust**:
   * Diferencia entre operaciones bloqueantes (*Sync/Threads*) y no bloqueantes (*Async/Futures*).
   * El trait `Future` y cómo las tareas no hacen nada hasta que se invocan con `.await` (*Pull-based Futures*).
2. **Sintaxis `async fn` y el Operador `.await`**:
   * Funciones que retornan tipos opacos `impl Future<Output = T>`.
   * Puntos de suspensión y retorno voluntario de control al *Executor*.
3. **El Runtime Tokio**:
   * La macro `#[tokio::main]`.
   * Lanzar tareas concurrentes ultraligeras con `tokio::spawn`.
   * Tokio Sleep y temporizadores sin bloquear hilos del SO (`tokio::time::sleep`).
4. **Primitivas de Sincronización Asíncrona**:
   * `tokio::sync::Mutex` vs `std::sync::Mutex`.
   * Canales asíncronos (`tokio::sync::mpsc`, `broadcast`, `oneshot`, `watch`).
   * Manejo de múltiples tareas competitivas con `tokio::select!` y `tokio::join!`.
