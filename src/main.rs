use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    // el primer argumento es la ubicacion del binario compilado, asi que saltar
    let primero: String = args.nth(1).unwrap();

    // Despues de acceder al segundo argumento, el elemento iterador siguiente se convierte en el primero
    let operador: String = args.nth(0).unwrap();
    let segundo: String = args.nth(0).unwrap();

    println!("{} {} {} ", primero, operador, segundo);


}
