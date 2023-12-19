# Bucles condicionales con while

- Un programa necesitará a menudo evaluar una condición dentro de un bucle. Mientras la condición es verdadera, el bucle se ejecuta. Cuando la condición deja de ser cierta, el programa llama a break, deteniendo el bucle. Es posible implementar un comportamiento como éste utilizando una combinación de bucle, if, else y break; si lo desea, puede intentarlo ahora en un programa. Sin embargo, este patrón es tan común que Rust tiene una construcción de lenguaje incorporada para él, llamada bucle while. usamos while para hacer un bucle en el programa tres veces, contando hacia atrás cada vez, y luego, después del bucle, imprimir un mensaje y salir.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

- Esta construcción elimina una gran cantidad de anidamiento que sería necesario si utilizara bucle, if, else y break, y es más clara. Mientras una condición se evalúa como verdadera, el código se ejecuta; de lo contrario, sale del bucle.
