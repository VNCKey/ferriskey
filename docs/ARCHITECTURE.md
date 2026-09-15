# Arquitectura de FerrisKey

FerrisKey separa la aplicación en cuatro responsabilidades principales:

```text
main.rs
  └── app.rs                 composición de la ventana y navegación
      ├── state/              estado mutable de la sesión
      ├── views/              renderizado egui y eventos de interacción
      ├── application/        casos de uso y reglas de la aplicación
      ├── domain/             tipos y validaciones independientes de egui
      ├── infrastructure/     disco, configuración y procesos del sistema
      └── content/             ejemplos educativos iniciales
```

## Reglas de dependencia

- `domain` no depende de egui ni de la interfaz.
- `application` puede usar `domain`, `errors`, `platform` e `infrastructure`.
- `views` consume casos de uso y actualiza el estado de la sesión.
- `state` conserva estado mutable; no debe contener operaciones de disco o procesos.
- `main.rs` solo configura el runtime de eframe y tracing.

## Proyectos y terminal

La lectura y escritura de archivos pasa por `ProjectService`. La ejecución de
comandos y la detección de comandos Cargo viven en `terminal_service`. Las
tareas que pueden tardar se lanzan mediante `TaskManager`, evitando bloquear
el hilo de egui.

Los errores de disco, configuración y tareas se representan con
`FerrisKeyError`, para que la interfaz pueda mostrar mensajes comprensibles al
estudiante.

## Persistencia

La sesión mínima se guarda en el directorio de configuración estándar del
sistema operativo. Actualmente conserva el proyecto y archivo seleccionados;
el formato puede ampliarse sin acoplarlo al estado visual.
