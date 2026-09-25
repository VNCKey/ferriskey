# 🧪 Code Lab: Sesión 09 - Tipos Personalizados (Structs & Enums)

> **Entorno**: Code Lab interactivo integrado en FerrisKey ([`src/app/views/structs/mod.rs`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/structs/mod.rs))  
> **Herramientas activas**: Editor interactivo Syntect, Terminal embebida y Drawer de Retos.

---

## 🎯 Descripción del Entorno de Práctica

En este Code Lab modelarás un dominio completo en Rust utilizando estructuras con constructores y métodos (`impl`), y diseñarás enumeraciones ricas procesadas con *Pattern Matching*.

---

## 🏁 Ruta de Ejercicios Prácticos

### 📌 Ejercicio 1: Definición de Structs y Bloque `impl`
- **Código Base**:
  ```rust
  struct CuentaBancaria {
      titular: String,
      saldo: f64,
  }

  impl CuentaBancaria {
      // Constructor
      pub fn new(titular: &str, saldo_inicial: f64) -> Self {
          Self {
              titular: titular.to_string(),
              saldo: saldo_inicial,
          }
      }

      // Método de lectura
      pub fn consultar_saldo(&self) -> f64 {
          self.saldo
      }

      // Método de mutación
      pub fn depositar(&mut self, monto: f64) {
          self.saldo += monto;
      }
  }

  fn main() {
      let mut cuenta = CuentaBancaria::new("Ferris", 500.0);
      cuenta.depositar(150.0);
      println!("Saldo final de {}: ${}", cuenta.titular, cuenta.consultar_saldo());
  }
  ```
- **Tu Tarea**:
  1. Implementa un método `retirar(&mut self, monto: f64) -> bool` que impida retirar fondos si el monto supera el saldo disponible.

---

### 📌 Ejercicio 2: Modelado con Enumeraciones Ricas
- **Código Base**:
  ```rust
  enum EventoServidor {
      Inicio,
      PeticionWeb { ruta: String, metodo: String },
      Fallo(u16, String),
  }

  fn procesar_evento(evento: EventoServidor) {
      match evento {
          EventoServidor::Inicio => println!("Servidor en línea"),
          EventoServidor::PeticionWeb { ruta, metodo } => {
              println!("HTTP {metodo} en {ruta}");
          }
          EventoServidor::Fallo(codigo, mensaje) => {
              println!("Error {codigo}: {mensaje}");
          }
      }
  }

  fn main() {
      let e1 = EventoServidor::PeticionWeb {
          ruta: String::from("/api/datos"),
          metodo: String::from("GET"),
      };
      let e2 = EventoServidor::Fallo(404, String::from("No encontrado"));

      procesar_evento(e1);
      procesar_evento(e2);
  }
  ```
- **Tu Tarea**:
  1. Utiliza `if let` en `main` para capturar e imprimir únicamente los eventos de tipo `Fallo`.
