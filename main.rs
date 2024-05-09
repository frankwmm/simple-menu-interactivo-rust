use std::process::Command;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        // Mostrar el menú de opciones
        println!("Selecciona una opción:");
        println!("1. Mostrar directorio actual");
        println!("2. Listar archivos en el directorio actual");
        println!("3. Salir");

        // Obtener la entrada del usuario
        print!("Opción: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Convertir la entrada a un número entero
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Ejecutar el comando correspondiente según la opción seleccionada
        match choice {
            1 => {
                let output = Command::new("pwd")
                    .output()
                    .expect("Fallo al ejecutar el comando 'pwd'");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            },
            2 => {
                let output = Command::new("ls")
                    .output()
                    .expect("Fallo al ejecutar el comando 'ls'");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            },
            3 => break,
            _ => continue,
        }
    }
}
