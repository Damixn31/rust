# Etiquetas de bucle para distinguir entre varios bucles

- Si tienes bucles dentro de bucles, break y continue se aplican al bucle más interno en ese punto. Opcionalmente, puede especificar una etiqueta de bucle en un bucle que luego puede utilizar con break o continue para especificar que esas palabras clave se aplican al bucle etiquetado en lugar del bucle más interno. Las etiquetas de bucle deben comenzar con una comilla simple. He aquí un ejemplo con dos bucles anidados:

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

- El bucle exterior tiene la etiqueta 'contando_arriba, y contará hacia arriba de 0 a 2. El bucle interior sin etiqueta cuenta hacia abajo de 10 a 9. El primer break que no especifica una etiqueta saldrá del bucle interno solamente. La sentencia break 'contando_arriba; saldrá del bucle exterior. Este código imprime:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/loops`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2

```
