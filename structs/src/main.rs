//Similar to C like languages
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//Las structs pueden ser utilizadas para diferenciar un tipo de otro incluso si son isomorficos
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)] //What is this ?
struct Rectangle {
    width: u32,
    height: u32,
}

//Para agregar metodos
impl Rectangle {
    //&self es una referencia a la instancia de struct que esta llamando el metodo, asi que no area no se hace owner de esta.
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    //&self se usa cuando no se requiere mutar la instancia que llama el metodo (como un getter)
    fn has_width(&self) -> bool {
        return self.width > 0;
    }

    //En este contexto se sabe que &self significa &Rectangle, pero si el metod resive otro rectangle abra que espesificarlo
    fn can_hold(&self, other: &Rectangle) -> bool {
        //By the way, notece como no hay que usar * para optener el valor de la ref, esto es porque Rust (like Go) hace automatic dereference.
        //Y tambien se puede obviar el return (not a fan : /)
        self.width > other.width && self.height > other.height
    }
}

//Si, una misma strut puede tener mas de un impl block
impl  Rectangle {
    //En este contexto Self es la struct que corresponde al blocke impl, Rectangle para elc caso
    //notece como esta funcion no depende de ningun Reactangle espesifico, de ahi su nombre Associated Functions
    fn create_square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}


fn main() {
    println!("Structs notes ...");

    //Por default son imutables como otras variables
    //un campo no pueder ser mutable por si solo
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    //Si user1 no fuera mut esto no seria posible
    user1.username = String::from("adn22");

    let user2 = build_user(String::from("adn22@hotmail.com"), String::from("adn22"));

    //Se pueden copiar valores de structs existentes como sigue:
    let user3 = User {
        email: String::from("hood@gmail.com"),
        ..user2
    };

    println!(
        "{}, {} {}, {}",
        user3.username, user3.active, user3.email, user3.sign_in_count
    );

    //These all called tuple struts
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    print_color(black);
    draw_point(origin);
    //Estas dos no compilan
    //print_color(origin);
    //draw_point(black);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area is {}", rect1.area());
    println!("Has non zero width ? {}", rect1.has_width());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));//Passing just the reference not to own the value
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq_one = Rectangle::create_square(100);
    //Como un cuadrado es tambien un rectangulo puede usar sus metodos!
    println!("Can sq_one hold rect3? {}", sq_one.can_hold(&rect3));

}

//Aunque Color y Point tengan los mismos campos no se estan refiriendo a la misma cosa
//de ahi la utilidad, esto es como los opaque types de Scala
fn print_color(color: Color) {
    println!("{}, {}, {}", color.0, color.1, color.2);
}
fn draw_point(point: Point) {
    println!("{}, {}, {}", point.0, point.1, point.2)
}

//This is like a constructor
fn build_user(email: String, username: String) -> User {
    return User {
        active: true,
        username, //Esto es como el spread operator de JS
        email,
        sign_in_count: 1,
    };
}
