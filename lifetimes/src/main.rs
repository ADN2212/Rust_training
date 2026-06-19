fn main() {
    println!("Lifetimes anotations ...");

    let string1 = String::from("a");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
    //let _x= string2;
    //drop(string1);//Notece como no puedo dropear string1 aqui porque su valor fue tomado prestado por longest
    println!("{}", result);

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
