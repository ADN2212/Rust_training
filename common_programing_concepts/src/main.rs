fn main() {
    println!("Common programing concepts");
    //En Rust la variables son inmutables por dafault
    let mut x = 5;
    println!("x is {}", x);
    x = 10; //Esto genera un error de compilacion si x no fue declarado como mutable, "mut"
    println!("x has chaged, now is {}", x);

    //there are constants
    //const mut BAR: u32 = 1000;//esta linea genera un error de compilacion, cuz constants can not be mutable
    //Usar mayuscular es una convection.
    //Ojo: Nada que sea calcudo en runtime pueder ser una constante.
    const BAR: u32 = 1500;
    println!("{} is a constant meaning that it cannot be changed", BAR);

    //Shadowing
    //You can do something like:
    let y = 10;
    println!("y is {}", y);
    //Aqui se esta redeclarando y, cosa que no es posible en otros lenguages tipados, de esta linea para abajo y pasa a ser el nuevo valor.
    //Esto no es una reasignacion.
    //Todo esto tiene algo que ver con el ownership, stay tuned.
    let y = "foo";
    println!("Now y is {}", y);

    //Data types:
    //Intergers
    // let a: i32 = 98_222;//Decimal
    // let b: i32 = 0xff;//Hexadecimal
    // let c: i32 = 0o77;//Octal
    // let d: i32 = 0b1111_0000;//Banirio
    // let e: u8 = b'A';//Byte (u8 only)

    //Integer overflow, si se asigna un valor mayor que el maxiomo soportado por un tipo el compilador will panic
    //let f: u8 = 255;//panics wiht n >= 256

    //Floating-point numbers
    //let f = 2.0; //por default son f64

    //El compilador es capaz de inferir los tipos de todas estas operaciones
    // let sum =5 + 10;
    // let diff = 95.5 - 4.3;//
    // let prod = 4 * 30;
    // let quotient = 56.7 / 32.2;
    // let resto = 43 % 5;

    //there are booleans:
    //let t = true;
    //let f = false;

    //there are character
    // let c = 'Z';
    // let z = 'b';

    //There are also compount types:
    //tuples
    // let tup = ("Yo", 31);
    // //they can be destrutured
    // let (yo, age) = tup;
    // //you can use the dot notation
    // let yolo = tup.0;

    // //arrays
    // let nums = [10, 11, 12];
    // let once = nums[1]; //Si se intene in index fuera del array el compilador panics.

    // //what ???
    // let byte = [0; 8];

    //Functions
    // let s = foo(10, 10); //look, foo is hoisted.

    //Flow control shit
    //let condition = true;
    //let num = if condition { 5 } else { 6 };//Los if retornan como en Scala.

    //there are loop
    let mut counter = 0;

    //Este es como un while true
    let result = loop {
        println!("{}", counter);
        counter += 1;
        if counter == 10 {
            break counter; //Esto hace que el loop retorne
            //return counter;//esto seria un rturn para la funcion (main), no para el loop.
        }
    };

    println!("El loop termino con el valor {}", result);

    //El clasico while
    let mut num = 3;

    while num != 0 {
        println!("{}", num);
        num -= 1;
    }

    //y el for normalito:
    let arr = [10, 11, 12];
    for el in arr.iter() {
        println!("{}", el);
    }

    //Tambien existe un tipo range:
    let range = 1..100;
    for i in range {
        println!("{}", i);
    }

    /*
        Dont forget about multy line comments ...
     */

}

//there are functions
// fn foo(a: i32, b: i32) -> i32 {
//     println!("Fooooooooooooooo {}", a + b);
//     return a + b;
// }
