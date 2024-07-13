use rand::Rng;
use std::io;

fn main() {
    adivina_el_numero();
}

fn adivina_el_numero() {
    println!("########## Bienvenido a adivina el numero ##########");

    let mut rng = rand::thread_rng();
    let numero_random: u32 = rng.gen_range(0..101);

    let mut nombre = String::new();
    println!("Porfavor ingrese su nombre: ");
    io::stdin()
        .read_line(&mut nombre)
        .expect("Ha ocurrido un error al ingresar nombre");

    let mut intentos: u32 = 1;
    let mut acertaste = false;

    println!(
        "{} Recuerda que tienes solo 3 intentos para acertar el numero",
        nombre
    );

    while intentos <= 3 {
        intentos += 1;

        let mut numero = String::new();
        println!("Ingrese un numero del 0-100");
        io::stdin()
            .read_line(&mut numero)
            .expect("Ha ocurrido un error al ingresar el numero");

        let numero_ingresado: u32 = numero
            .trim()
            .parse()
            .expect("Ha ocurrido un error al parsear el entero");

        println!("{}", numero_random);

        if numero_ingresado == numero_random {
            println!("Haz acertado :)");
            acertaste = true;
            break;
        } else {
            println!("No haz acertado :(");
            acertaste = false;
        }
    }

    if acertaste == true {
        println!("¡¡¡Felicidades {} haz ganado!!!", nombre);
    } else {
        println!(
            "Lo sentimos {} pero se te han acabado los intentos :(",
            nombre
        );
    }
}
