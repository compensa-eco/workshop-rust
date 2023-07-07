



fn main() {
    println!("Hello, world!");

    // tipos primitivos
    let mut age: i32 = 24; // i8, i16, i32, i64

    let _salary: f32 = 100000.0; //f32, f64

    // tipo string
    let name: &str = "Giulia";
    let another_name: String = String::from("Rodrigo");

    let color_array = ["amarelo", "vermelho", "azul"];

    let mut dog: &str = "Luke";

    println!("O nome é: {}", name);
    println!("O outro nome é: {} e ele tem: {}", another_name, age);
    println!("Array: {:#?}", color_array);
    greet_dog(dog);

    dog = "Leia";

    greet_dog(dog);

    age = age + 200;

    println!("A nova idade é: {}", age);
    let return_fn = is_underage(age);
    println!("O retorno da função é: {}", return_fn);

    shadowing();


    {
        let apples = "Maçãs";
        println!("Eu tenho {} maçãs. dentro do lifetime criado", apples);
        let apples = 10;
        println!("Eu tenho {} maçãs. dentro do lifetime criado", apples);
    }

    //
    let mut x = 10;
    let y = 20;

    let calculo = soma(x, y);

    println!("Cálculo: {}", calculo);

}

fn soma(x: i32, y: i32) -> i32 {
    x + y
}

// estudando shadowing
fn shadowing() { //
    // shadowing
    let apples = 10;
    println!("Eu tenho {} maçãs.", apples);
    let apples = "apples";
    println!("Eu tenho {} maçãs.", apples);
} //


fn greet_dog(name: &str) {
    println!("O nome do dog é: {}", name);
}

fn is_underage(age: i32) -> i32 {
    if age < 18 {
        println!("Menor de idade");
        5 + 5
    } else if age >= 20 && age <= 30 {
        println!("Tem entre 20 e 30 anos");
        20 + 20
    } else {
        println!("Muito velho");
        10 + 50
    }
}
