# 🧪 Code Lab: Sesión 12 - Traits y Polimorfismo

> **Entorno**: Code Lab interactivo integrado en FerrisKey ([`src/app/views/traits/mod.rs`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/traits/mod.rs))  
> **Herramientas activas**: Editor interactivo Syntect, Terminal embebida y Drawer de Retos.

---

## 🎯 Descripción del Entorno de Práctica

En este Code Lab definirás e implementarás traits personalizados, aplicarás traits estándar de la librería estándar (`Display`, `Clone`) y compararás el despacho estático frente al despacho dinámico con `&dyn Trait`.

---

## 🏁 Ruta de Ejercicios Prácticos

### 📌 Ejercicio 1: Definición e Implementación de un Trait
- **Código Base**:
  ```rust
  trait Imprimible {
      fn imprimir(&self);
  }

  struct Mensaje(String);

  impl Imprimible for Mensaje {
      fn imprimir(&self) {
          println!("Mensaje: {}", self.0);
      }
  }

  fn main() {
      let m = Mensaje(String::from("Hola desde un Trait en FerrisKey"));
      m.imprimir();
  }
  ```
- **Tu Tarea**:
  1. Agrega un método con implementación por defecto `fn imprimir_duplicado(&self)` al trait `Imprimible`.

---

### 📌 Ejercicio 2: Implementación de `Display`
- **Código Base**:
  ```rust
  use std::fmt;

  struct Punto {
      x: i32,
      y: i32,
  }

  impl fmt::Display for Punto {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "Punto(x: {}, y: {})", self.x, self.y)
      }
  }

  fn main() {
      let p = Punto { x: 5, y: 12 };
      println!("Coordenada formateada: {p}");
  }
  ```
- **Tu Tarea**:
  1. Añade `#[derive(Debug, Clone, PartialEq)]` sobre el struct `Punto` y comprueba que puedes clonarlo y compararlo con `==`.

---

### 📌 Ejercicio 3: Polimorfismo Dinámico con `&dyn Trait`
- **Código Base**:
  ```rust
  trait Notificacion {
      fn enviar(&self);
  }

  struct Email(String);
  struct SMS(String);

  impl Notificacion for Email {
      fn enviar(&self) {
          println!("Enviando Email: {}", self.0);
      }
  }

  impl Notificacion for SMS {
      fn enviar(&self) {
          println!("Enviando SMS: {}", self.0);
      }
  }

  fn main() {
      let email = Email(String::from("bienvenido@rust.org"));
      let sms = SMS(String::from("+123456789"));

      // Colección heterogénea utilizando trait objects dinámicos:
      let notificaciones: Vec<&dyn Notificacion> = vec![&email, &sms];

      for n in notificaciones {
          n.enviar(); // Despacho dinámico a través de la vtable
      }
  }
  ```
- **Tu Tarea**:
  1. Implementa una tercera variante `struct Slack(String)` y añádela al vector de notificaciones sin modificar la firma del bucle.
