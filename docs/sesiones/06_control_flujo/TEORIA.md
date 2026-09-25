# 📖 Teoría: Sesión 06 - Control de Flujo

---

## 1. Condicionales como Expresiones (`if / else`)

En Rust, `if` es una **expresión**, lo que significa que evalúa y produce un valor final:

```rust
let edad = 20;
let tiene_permiso = true;

// Todas las ramas deben evaluar exactamente al mismo tipo de dato
let acceso: &str = if edad >= 18 && tiene_permiso {
    "permitido"
} else {
    "denegado"
};
```
- La condición debe ser estrictamente de tipo `bool` (Rust no tiene *truthy/falsy* como JavaScript o Python).
- No se requieren paréntesis alrededor de la condición booleana, pero las llaves `{}` son obligatorias.

---

## 2. Bucles en Rust: Tres Sabores

### A. `loop` (Repetición Infinita con Retorno)
`loop` repite un bloque indefinidamente hasta encontrar una sentencia `break`. Permite retornar un valor evaluado:

```rust
let mut intentos = 0;
let resultado = loop {
    intentos += 1;
    if intentos == 10 {
        break intentos * 2; // Devuelve 20 de la expresión loop
    }
};
```

### B. `while` (Bucle Condicional)
Evalúa una condición booleana antes de cada iteración:

```rust
let mut contador = 3;
while contador > 0 {
    println!("{contador}...");
    contador -= 1;
}
```

### C. `for` (Recorrido Seguro sin Bounds Checking)
Es el bucle más seguro y eficiente en Rust. Evita accesos fuera de rango:

```rust
// Rango exclusivo: 1 al 4
for num in 1..5 {
    println!("{num}");
}

// Rango inclusivo: 1 al 5
for num in 1..=5 {
    println!("{num}");
}
```

### Etiquetas de Bucle (*Loop Labels*)
Cuando existen bucles anidados, las etiquetas identificadas con comilla simple (`'nombre:`) permiten que `break` o `continue` afecten al bucle exterior:

```rust
'exterior: for x in 0..10 {
    for y in 0..10 {
        if x + y == 15 {
            break 'exterior; // Rompe el bucle exterior directamente
        }
    }
}
```

---

## 3. Pattern Matching Exhaustivo (`match`)

La expresión `match` compara un valor contra múltiples patrones y ejecuta el primer brazo que coincida:

```rust
let calificacion = 85;

let letra = match calificacion {
    90..=100 => 'A',
    80..=89  => 'B',
    70..=79  => 'C',
    60..=69  => 'D',
    _        => 'F', // Comodín obligatorio para garantizar exhaustividad
};
```

### Características de `match`:
1. **Exhaustividad Garantizada**: El compilador exige que se cubran todos los posibles valores del tipo analizado.
2. **Alternativas Múltiples (`|`)**: `1 | 2 | 3 => println!("Menor o igual a 3")`.
3. **Guardas de Coincidencia (*Match Guards*)**: Permiten añadir condiciones booleanas extra al brazo:
   ```rust
   let par = (2, -2);
   match par {
       (x, y) if x == y => println!("Son iguales"),
       (x, y) if x + y == 0 => println!("Son opuestos"),
       _ => println!("Sin relación especial"),
   }
   ```
