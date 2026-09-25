# 🗺️ Roadmap: Sesión 07 - Funciones y Closures

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Closures` / `Funciones`  
> **Ruta interna**: `AppRoute::TutorialFunciones`  
> **Vistas asociadas**: [`src/app/views/funciones/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/funciones/)

---

## 🎯 Objetivos de Aprendizaje

1. **Sintaxis y Naturaleza de los Closures**:
   - Funciones anónimas con sintaxis de barras verticales `|param| expresión`.
   - Inferencia automática de tipos de entrada y salida basada en el primer uso.
2. **Las Tres Formas de Captura del Entorno**:
   - **Por referencia inmutable (`Fn`)**: Observa variables del entorno exterior sin alterarlas.
   - **Por referencia mutable (`FnMut`)**: Modifica variables del entorno exterior.
   - **Por movimiento de propiedad (`FnOnce` / `move`)**: Fuerza la transferencia de propiedad (*Ownership Move*) de las variables capturadas hacia el cierre.
3. **Closures como Argumentos y Retornos**:
   - Pasar closures a funciones de orden superior mediante genéricos y *Trait Bounds* (`F: Fn(i32) -> i32`).
   - Uso crítico de la palabra clave `move` para enviar tareas a otros hilos (`std::thread::spawn`).

---

## 🧭 Estructura en FerrisKey
- **`Overview`**: Anatomía de funciones y closures en memoria.
- **`Parámetros`**: Paso por valor, referencia `&` y referencia mutable `&mut`.
- **`Retorno`**: Expresiones implícitas, sentencias de retorno y tuplas de salida.
- **`Closures`**: Sintaxis, inferencia, captura y la cláusula `move`.
- **`Code Lab`**: Retos guiados de implementación y evaluación de closures.
