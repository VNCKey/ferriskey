# 🗺️ Roadmap: Sesión 17 - Concurrencia y Multihilo (*Fearless Concurrency*)

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Concurrency`  
> **Ruta interna**: `AppRoute::TutorialConcurrencia`  
> **Vistas asociadas**: [`src/app/views/concurrencia/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/concurrencia/)

---

## 🎯 Objetivos de Aprendizaje

1. **Creación y Sincronización de Hilos Nativos**:
   * `std::thread::spawn` para bifurcar ejecución en paralelo.
   * `handle.join()` para esperar la finalización y recolectar resultados.
   * La palabra clave `move` en closures para transferir ownership al nuevo hilo.
2. **Paso de Mensajes (*Message Passing*) con Canales**:
   * `std::sync::mpsc` (*Multiple Producer, Single Consumer*).
   * Envío asíncrono con `tx.send(msg)` y recepción bloqueante/no bloqueante (`rx.recv()` y `rx.try_recv()`).
3. **Estado Compartido Seguro (*Shared-State Concurrency*)**:
   * El patrón `Arc<Mutex<T>>` para acceso exclusivo y exclusión mutua.
   * Lecturas múltiples concurrentes con `Arc<RwLock<T>>`.
   * Prevención de bloqueos mutuos (*Deadlocks*).
4. **Traits Marcadores del Compilador: `Send` y `Sync`**:
   * `Send`: Tipos cuya propiedad puede transferirse de forma segura a otro hilo.
   * `Sync`: Tipos cuyas referencias pueden compartirse de forma segura entre múltiples hilos.
