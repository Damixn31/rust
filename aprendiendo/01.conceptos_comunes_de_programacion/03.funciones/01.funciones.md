# Funciones

- Las funciones son frecuentes en el código Rust. Ya has visto una de las funciones más importantes del lenguaje: la función main, que es el punto de entrada de muchos programas. También has visto la palabra clave fn, que te permite declarar nuevas funciones.

El código Rust utiliza snake case como el estilo convencional para los nombres de funciones y variables, en el que todas las letras son minúsculas y losguiones bajos separan las palabras. Aquí tienes un programa que contiene un ejemplo de definición de función:

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

- Definimos una función en Rust introduciendo fn seguido de un nombre de función y un conjunto de paréntesis. Las llaves indican al compilador dónde empieza y termina el cuerpo de la función.

Podemos llamar a cualquier función que hayamos definido introduciendo su nombre seguido de un conjunto de paréntesis. Como otra_función está definida en el programa, puede ser llamada desde dentro de la función principal. Observe que definimos otra_función después de la función principal en el código fuente; también podríamos haberla definido antes. A Rust no le importa dónde definas tus funciones, sólo que estén definidas en algún lugar en un ámbito que pueda ser visto por el llamador.

Empecemos un nuevo proyecto binario llamado functions para explorar más a fondo las funciones. Coloca el ejemplo otra_funcion en src/main.rs y ejecútalo. Deberías ver la siguiente salida:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/functions`
Hello, world!
Another function.
```

- Las líneas se ejecutan en el orden en que aparecen en la función principal. Primero se imprime el mensaje "¡Hola, mundo!", y después se llama a otra_función y se imprime su mensaje.
