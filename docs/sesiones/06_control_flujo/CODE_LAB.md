# 🧪 Code Lab: Sesión 06 - Control de Flujo

> **Entorno**: Code Lab interactivo integrado en FerrisKey ([`src/app/views/control_flujo/mod.rs`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/control_flujo/mod.rs))  
> **Herramientas activas**: Editor interactivo Syntect, Terminal embebida y Drawer de Retos.

---

## 🎯 Descripción del Entorno de Práctica

En este Code Lab ejercitarás la toma de decisiones con `if` como expresión, los bucles de repetición (`loop`, `while`, `for`) y el motor de *Pattern Matching* con `match`.

---

## 🏁 Ruta de los 4 Retos Prácticos

### 📌 Reto 1: Decisiones con `if` como Expresión
- **Código Base**:
  ```rust
  fn main() {
      let edad = 20;
      let tiene_permiso = true;

      let acceso = if edad >= 18 && tiene_permiso {
          "permitido"
      } else {
          "denegado"
      };

      println!("Estado de acceso: {acceso}");
  }
  ```
- **Tu Tarea**:
  1. Diseña una expresión `if / else if / else` que clasifique la temperatura ambiental en `"frío"`, `"templado"` o `"calor"`, asignando el resultado directamente a una variable inmutable.

---

### 📌 Reto 2: Repeticiones y Bucles (`loop`, `while`, `for`)
- **Código Base**:
  ```rust
  fn main() {
      // 1. Contador con while
      let mut contador = 0;
      while contador < 3 {
          contador += 1;
      }

      // 2. Recorrido con for y rango inclusivo
      for numero in 1..=3 {
          println!("Vuelta con for: {numero}");
      }

      // 3. Retorno de valor con loop y break
      let mut pasos = 0;
      let resultado = loop {
          pasos += 1;
          if pasos == 5 {
              break pasos * 10;
          }
      };
      println!("Resultado de loop: {resultado}");
  }
  ```
- **Tu Tarea**:
  1. Implementa una búsqueda con bucles anidados utilizando una etiqueta `'busqueda:` para salir inmediatamente de ambos bucles cuando se encuentre un número par específico.

---

### 📌 Reto 3: Pattern Matching Exhaustivo con `match`
- **Código Base**:
  ```rust
  fn main() {
      let nota = 85;

      let resultado = match nota {
          90..=100 => 'A',
          70..=89  => 'B',
          50..=69  => 'C',
          _        => 'F',
      };

      println!("Calificación final obtenida: {resultado}");
  }
  ```
- **Tu Tarea**:
  1. Modifica el `match` para que utilice una guarda `if` (*Match Guard*) detectando si el estudiante obtuvo un bono de participación adicional.

---

### 📌 Reto 4: Evaluación Técnica (Questions)
- **Nivel**: Autoevaluación | **Tags**: 10 Preguntas Clave
- Cuestionario de 10 preguntas sobre:
  1. Por qué no se admiten tipos no booleanos en condiciones `if`.
  2. La regla de tipos idénticos en las ramas de un `if` como expresión.
  3. Cómo `loop` puede retornar un valor mediante `break`.
  4. La diferencia entre `1..5` y `1..=5`.
  5. El requisito de exhaustividad estricta en expresiones `match`.
