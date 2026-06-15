use std::fs::File;
//use std::io::ErrorKind;

fn main() {
    println!("Error Handling Anotations ...");

    //Panic se usa cuando no se puede hacer nada para manejar el error
    //Por ejemplo, una llamada a una funcion que no ha sido implementada.
    //Para ver todas las llamadas que llevaron al error correr el programa como > RUST_BACKTRACE=1 cargo run
    //panic!("There is nothing to do here")    

    //Otro ejemplo:
    //let v = vec![1, 2, 3];
    //println!("{}", v[100]);

    //Para ocaciones en la cuales los error son manejables existe el enum Result, que funciona como el monad Try de Scala 
    //Como se pude ver aqui file es un Result<File, Error>
    // let fileResult = File::open("hello.txt");
    // //We can do patter maching here
    // let file = match fileResult {
    //   Ok(file ) => file,
    //   //Esto te permite decidir que hacer en cada caso, no es obligatorio hacer que el programa se paniquee aqui XD
    //   Err(error ) => panic!("Thre was a bobo while trying to open de file {error:?}"),  
    // };

    //En este punto file debe estar definido si o si.
    //println!("{file:?}")

    //Tambien es posible catchear errores espesificos:
    // let file_result = File::open("hello.txt");
    // let file = match file_result {
    //     Ok(file) => file,//Si existe el archivo lo toma
    //     Err(error ) => match error.kind() {
    //         //Si el error que ocurrio es not found, es decir no encontrado, lo intenta crear
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("File could not be created: {e:?}")//si no puede crearlo, panic!!!                
    //         },
    //         _ => {
    //             panic!("{error:?}")
    //         }
    //     }
    // };
    // println!("{file:?}");

    //Existen formas de hacer este proceso menos verboso:
    //En este caso si no se halla en archivo, the aplication will panic
    // let file = File::open("hello.txt").unwrap();
    // println!("{file:?}");
    
    //Expect pasara su argumento a panic!
    let file = File::open("hello.txt").expect("File did not exists, aborting the mission");
    println!("{file:?}");

}


use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),//El uso del return aqui significa que el error sera retornado por la funcion
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


fn magic_read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;//el "?"" hace que si no se logra abrir el archivo, retorne el error 
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;//lo mismo aqui, es como un early return para errores
    Ok(username)
}


//Nota: para que una funcion pueda user ? debe retorna el enum Option of Result, de lo contrario no compila
fn even_more_magical_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    //Se pueden concatenar, es decir, lo que esta a la derecha de ? solo se ejecutara si no ocurrio un error.
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

}

