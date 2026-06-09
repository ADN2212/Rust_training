#[derive(Debug)] //What is this ?
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)] //What is this ?
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//Los enums pueen ser de valores compuestos como sigue:
#[derive(Debug)]
enum BetterIpAddr {
    V4(String),
    V6(String),
}

//Incluso enums con valores compuestos de varios valores.
#[derive(Debug)]
enum EvenBetterIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

//Incluso mas complejas
//Todos estos valores podrian ser structs inconexas, pero gracias los enums es posible tenerlos bajo el mismo namespace Message::Bar
enum Message {
    Quit,                       //Empty
    Move { x: i32, y: i32 },    //struct
    Write(String),              //Sinble value
    ChangeColor(i32, i32, i32), //tuple
}

enum Dibujo {
    Cara,
    Escudo
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Dibujo),
}

fn value_in_cents(coin: Coin) -> u8 {
    //como en Scala los match han de ser exaustivos
    return match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(dibujo) => {
            //Ejemplo de pattern mathing anidado, aqui el segundo enum tambien tiene que ser exaustivamente analizado.
            match dibujo {
                Dibujo::Cara => println!("Salio cara"),
                Dibujo::Escudo => println!("Salio escudo")
            }
            25
        }
    }
}

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: four,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: six,
        address: String::from("::1"),
    };

    //println!("{}", home.kind);
    //De esta manera se puede declarar valores compuestos de forma mas compacta.
    let home_better = BetterIpAddr::V4(String::from("127.0.0.1"));
    let loopback_better = BetterIpAddr::V4(String::from("::1"));

    println!("{:?}", home_better);

    let home_even_better = EvenBetterIpAddr::V4(127, 0, 0, 1);
    println!("{:?}", home_even_better);

    //About the option type
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    //En Rust el tipo option es un enum con dos valores, Some(T) con T siendo un generic y None representado la nulidad
    //Es casi que el mismo option de Scala, pero en este caso no es un monad
    let some_number = Some(5);
    let some_char = Some('e');
    let a_number_that_could_be_but_is_not: Option<i32> = None;

    //let sum = 5 + some_number;//Esta linea no compila porque some_number podria no estar definido
    //claro que se puden hacer cosas como el getOrElse de scala
    let sum = 5 + some_number.unwrap_or(0);
    println!("sum = {}", sum);

    //Tambien existe el pattern matchin exaustivos como en los enums de Scala

    let fifty_cents = value_in_cents(Coin::Quarter(Dibujo::Cara)) * 2;
    println!("{}, many men, many many many many men", fifty_cents);


    let two = plus_one(Some(1));
    let not_a_num = plus_one(None);

    println!("{}", two.unwrap());
    println!("{}", not_a_num.unwrap_or(0));


    //Esta sintaxis permite desembolber el valor en caso de que este definido
    if let Some(num) = not_a_num {
        println!("Here is the number {}", num);
    } else {
        //Si no esta definido caera en este else
        println!("there was not a number")
    }

    if let Some(num) = two {
        println!("Here is the number {}", num);
    }




}


//Al igaual que en Scala se puede hacer pattern matching al tipo Option
//Esta funcion le suma uno al x si esta definido y retorna None si no lo esta.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}    

