# 🧪 Code Lab: Sesión 11 - Genéricos y Monomorfización

> **Entorno**: Code Lab interactivo integrado en FerrisKey ([`src/app/views/genericos/mod.rs`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/genericos/mod.rs))  
> **Herramientas activas**: Editor interactivo Syntect, Terminal embebida y Drawer de Retos.

---

## 🎯 Descripción del Entorno de Práctica

En este Code Lab programarás funciones y estructuras genéricas que admiten múltiples tipos de datos de forma segura, agregando restricciones de traits (*Trait Bounds*) cuando sea necesario operar sobre los valores.

---

## 🏁 Ruta de Ejercicios Prácticos

### 📌 Ejercicio 1: Funciones y Structs Genéricas
- **Código Base**:
  ```rust
  // Función genérica identidad
  fn duplicar_valor<T: Clone>(item: &T) -> (T, T) {
      (item.clone(), item.clone())
  }

  // Struct genérica
  struct Contenedor<T> {
      contenido: T,
  }

  impl<T> Contenedor<T> {
      fn new(contenido: T) -> Self {
          Self { contenido }
      }

      fn extraer(self) -> T {
          self.contenido
      }
  }

  fn main() {
      let caja_entera = Contenedor::new(100);
      let caja_texto = Contenedor::new(String::from("Ferris"));

      println!("Caja 1: {}", caja_entera.extraer());
      println!("Caja 2: {}", caja_texto.extraer());
  }
  ```
- **Tu Tarea**:
  1. Define una struct `ParOrdenado<T, U> { primero: T, segundo: U }` que permita almacenar dos valores de tipos totalmente independientes.

---

### 📌 Ejercicio 2: Restricciones con Trait Bounds y Cláusula `where`
- **Código Base**:
  ```rust
  use std::fmt::Display;

  fn comparar_e_imprimir<T>(a: T, b: T)
  where
      T: PartialOrd + Display,
  {
      if a > b {
          println!("El valor {a} es mayor que {b}");
      } else if a < b {
          println!("El valor {b} es mayor que {a}");
      } else {
          println!("Ambos valores son idénticos: {a}");
      }
  }

  fn main() {
      comparar_e_imprimir(15, 42);
      comparar_e_imprimir(3.14, 2.71);
      comparar_e_imprimir("manzana", "banana");
  }
  ```
- **Tu Tarea**:
  1. Diseña una función genérica `encontrar_maximo<T: PartialOrd + Copy>(lista: &[T]) -> Option<T>` que devuelva el elemento mayor de un slice.
