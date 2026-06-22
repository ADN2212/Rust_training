fn main() {
    println!("Lifetimes anotations ...");

    let string1 = String::from("aaaaa");
    let string2 = String::from("xyz");
    //let result: &str;
    // {
    //     let string2 = String::from("xyz");
    //     //Aqui el compilador dice "string2 does not live long enough"
    //     //lo que quiere deceir que, como string2 fue declarada dentro de este scope
    //     //al final de este la variable es dropeada
    //     //lo que implica que, cuando se intente acceder a result desde fuera de este scope
    //     //si se da el caso de que string2 es la cadena mas larga result estara apuntado a una dangling refrerence
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    //Con esta version el problema se elimina
    let result = longest(&string1, &string2);
    println!("The longest string is {result}");
    
    let mut vec_one = vec![10, 10, 10, 10];
    let mut vec_two = vec![10, 10, 10, 10, 10];
    //drop(vec_one);

    let largest_vec = shorter(&mut vec_one, &mut vec_two);

    //Estas lineas no se pueden ejecutar por que largest_vec puede ser owner de calquiara de los dos valores.
    //drop(vec_two);
    //drop(vec_one);

    for i in largest_vec {
        println!("{}", i);
    }

    {
        let x: i32 = 10;
        println!("{}", x);
    }

    let novel = String::from("What you see is all there is. Then, bla bla bla bla");
    let fs = novel.split(".").next().unwrap();
    let i = ImportantExcerpt{
        part: fs
    };
    //drop(novel);//Notece como el compilador no permite que novel sea dropeada,
    //porque existe una referencia a un substring de ella en fs y luego en part
    println!("{}", i.part)

}

//En este caso el hecho de que 'a y este tanto en la declaracion de la strutc como en part
//significa que sus lifetimes deben ser iguales
//notece que str es una referencia a un str, no una copia del valor de esa referencia.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

//Esta funcion no compila debido a que le falta un "named lifetime parameter"
//Lo que sucede aqui es que el compilador no es capaz de determinar a que referencia estara
//enlazado el valor retornado cuando sea retornado
//puede ser tanto x como y
// fn longest(x: &str, y: &str) -> &str {
//     // no compila
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

//Lo que dice la firma de esta funcion es:
//para algun lifetime 'a tanto x como y viven almenos 'a y la referencia retornada tambie vive 'a
//En espaniol
//Ahora el compilado sabe que el tiempo de vida de &str durara al menos lo que dura x o y
//por lo que, para el caso, sabra que si x o y son dropeados ya no es posible acceder a lo que retorna esta funcion
//evitando asi que se hagan accesos a "dangling references".
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

//Esta es mas de los mismo
// fn greates<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
//     if a > b { a } else { b }
// }

fn shorter<'a>(v1: &'a mut Vec<i32>, v2: &'a mut Vec<i32>) -> &'a mut Vec<i32> {
    if v1.len() > v2.len() { v1 } else { v2 }
}

fn foo() -> String {
    let str = String::from("Marola");
    println!("{}", str);
    drop(str);
    //println!("{str}");
    let str = String::from("Marola");
    return str;
}

//Some annotations about the Lifetime Elision Rules.
//Estas son reglas que el compilador usa para determinar los lifetimes sin que el programador tenga que intervenir

//1-Cada parámetro que es una referencia recibe su propio lifetime distinto.
//Ej: fn foo(x: &i32) pasa a ser fn foo<'a>(x: &'a i32), foo(x: &i32, y: &i32) pasa a ser fn foo<'a, 'b>(x: &'a i32, y: &'b i32)

//2-Si hay exactamente un input lifetime, este se asigna a todos los output lifetimes
//Ej: fn foo(x: &str) -> &str, pasa a ser fn foo<'a>(x: &'a str) -> &'a str
//O sea, el compilador asume que si se recibe una sola referencia como argumento, lo que sea que se retorne tiene que tener el mismo tiempo de vida.

//3-Si hay varios input lifetimes pero uno de ellos es &self o &mut self, el lifetime de self es asignado a todos los output lifetimes
//Ej:
// impl Parser {
//     fn siguiente_token(&self, entrada: &str) -> &str {
//         // ...
//     }
// }
// // se expande a:
// fn siguiente_token<'a, 'b>(&'a self, entrada: &'b str) -> &'a str
//El compilador asume que si es un método, lo que se está retornando proviene del propio objeto (self), de ahí que 'a sea el lifetime asignado a lo que retorna la función.
//Ojo: en caso de que ese no sea el caso, el programador tendrá que asignar los lifetimes de forma manual.

//El compilador aplica estas tres reglas en orden, y en caso de no poder determinar la firma de la función, pues .. no compila.
//Ejemplo de por qué la función longest no compila sin anotar los lifetimes:
//we have got fn longest(x: &str, y: &str) -> &str
//1- se le asigna un lifetime a cada parámetro: fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str
//2- como no hay exactamente un input lifetime, la segunda regla no aplica.
//3- tampoco es un método, por lo que la tercera regla no aplica tampoco

//Entonces el compilador no puede determinar la firma de la función en forma automática y .... puff, falla, es tu turno como programador de anotar los lifetimes de tu función.