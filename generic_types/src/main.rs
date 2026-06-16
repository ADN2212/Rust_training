use std::cmp::PartialOrd;

fn main() {
    println!("Generic types anotations ...");
    //Como otros lenguages de programacion, una me las formas mas basicas de no repetir codigo es unsand funcions
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    //let mut largest = &number_list[0];
    let result = largest(&number_list);
    //Para no repetir esta logica declaramos una funcion, basic!!
    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    println!("The largest number is {result}");
    //Pero este solo fue un primer layer de abstraccion, supongamos que ahora queremos una funcion que haga lo mismo pero con un vector de chars
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {result}");
    //notece como el tipo es tambien un argumento de la funcion
    let l1 = generic_largest::<i32>(&number_list);
    let l2 = generic_largest::<char>(&char_list);
    println!("The largest are {} and {}", l1, l2);

    //Notece como el compilador puede inferir los tipos:
    let point1 = Point{x: 10,  y: 5};
    let point2 = Point{x: 1.5, y:2.4};
    //even more:
    //Does that makes sence? Para eso hay que ser mas espesificos en la definicion del tipo T 
    let point3 = Point{x: "Hola", y: "Marola"};

    println!("x: {}, y: {}", point1.x, point1.y);
    println!("x: {}, y: {}", point2.x, point2.y);
    println!("x: {}, y: {}", point3.x, point3.y);

    //Una limitacion:
    //Si la struct esta declerada como Point<T>, x e y han de ser del mismo tipo
    //cuando es Point<T, U> no hay problemas de compilacion, porque T y U puedan ser distintos y el mismo 
    let point4 = PointTwo{x: 4, y: 5.5};//Vease como el rust-analyzar infiere los tipos de ganster
    let point5 = PointTwo{x: 10, y:10};
    println!("x: {}, y: {}", point4.x, point4.y);
    println!("x: {}, y: {}", point5.x, point5.y);
    
    println!("pointq x = {}", point1.x());
    println!("distance((0, 0), ({}, {}) = {}", point2.x, point2.y, point2.distance_from_origin());
    //Notece como distance from origin no esta definido, ya que point1 es Point<i32> no f32:
    //println!("distance((0, 0), ({}, {}) = {}", point1.x, point1.y, point1.distance_from_origin()); 




}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//Esta funcion poseee la misma logica que largets, la unica diferencia el el tipo de vector que resive el el tipo que retorna
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//Para agregar una nueva capa de abstraccion se usan los tipos genericos
//Aqui T es un tipo que comple con poseer el trait PartalOrd, es decir que puede ser ordenado(I guess)
//Esto puede verce desde una perspectiva de teoria de conjuntos,
//digamos que nuestro T es el conjunto de todos los tipos que puede ser ordenados,
//entonces tando i32 como char son elementos de dicho conjunto, y por tanto pueden ser pasados como argumentos de esta funcion.
fn generic_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//Another example:
struct PointTwo<T, U> {
    x: T,
    y: U,
}

struct Point<T> {
    x: T,
    y: T,
}

//En este caso el bloque impl funciona para puntos de tipo T
//Osea, si T estubiera limitado a suptimos de nums
//un punto con chars, no funcionaria
impl<T> Point<T> {
    //by the way this is a getter
    fn x(&self) -> &T {
        &self.x
    }
}

//En este caso los metodos declarados dentro del bloque solo se veran disponibles para puntos de valores de tipo f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
