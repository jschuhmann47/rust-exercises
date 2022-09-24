/*
Convert temperatures between Fahrenheit and Celsius.
Generate the nth Fibonacci number.
Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
*/

//use std::default;


fn main() {

    let celsius = 10;
    println!("{celsius}°C to fahrenheit: {}",celsius_to_fahrenheit(celsius));
    let fa = 50;
    println!("{fa}°F to celsius: {}",fahrenheit_to_celsius(fa));
    println!("fibonacci 0th: {}",n_fibonacci(0));
    println!("fibonacci 1st: {}",n_fibonacci(1));
    println!("fibonacci 3rd: {}",n_fibonacci(3));
    println!("fibonacci 7th: {}",n_fibonacci(7));
    println!("fibonacci 10th: {}",n_fibonacci(10));
    //println!("fibonacci 10th: {}",n_fibonacci(-10));
    println!("Get ready for christmas\n");
    the_twelve_days_of_christmas();
}

fn celsius_to_fahrenheit(celsius: i32) -> i32 {
    9/5 * celsius + 32
}
fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5/9
}

// fn n_fibonacci(n: i32) -> i32 {
//     let mut valor_a_sumar = 1;
//     let mut valor_anterior = 0;
//     let mut contador = 0;
//     let mut valor_acum = 0;

//     while contador <= n {
//         let aux;
//         valor_acum = valor_a_sumar + valor_anterior;
//         aux = valor_a_sumar;
//         valor_a_sumar = valor_acum;
//         valor_anterior = aux;
//         contador += 1;
//     }
//     valor_acum
// }

fn fibonacci_rec(n1: i32, n2: i32,n: i32) -> i32 {
    if n<0 {panic!("Cannot get negative index");}
    if n==0 {return 0;}
    if n==1 {return n1;}
    else {
        return fibonacci_rec(n1+n2, n1,n-1);
    }   
}

fn n_fibonacci(n: i32) -> i32 {
    return fibonacci_rec(1, 0, n);
}

fn the_twelve_days_of_christmas() {
    let gifts = ["A partridge in a pear tree","Two turtle doves, and","Three french hens",
    "Four calling birds","Five golden rings","Six geese a-laying","Seven swans a-swimming", "Eight maids a-milking",
    "Nine ladies dancing","Ten lords a-leaping","Eleven pipers piping","Twelve drummers drumming"];

    let start = "On the X day of Christmas, my true love sent to me";
    let cardinals = ["first","second","third","fourth","fifth","sixth","seventh","eighth","ninth","tenth","eleventh","twelfth"];

    for n in 1..13 {
        println!("{}",start.replace("X", cardinals[n-1]));
        for num in (1..n+1).rev() {
            println!("{}",gifts[num-1]);
        }
        print!("\n");
    }

    
}

// fn number_to_cardinal_string(n: i32) -> &str{

//     let s = match n {
//         1 => "first",
//         2 => "second",
//         3 => "third",
//         4 => "fourth",
//         5 => "fifth",
//         6 => "sixth",
//         7 => "seventh",
//         8 => "eighth",
//         9 => "ninth",
//         10 => "tenth",
//         11 => "eleventh",
//         12 => "twelfth",
//         _ => panic!("nro invalido")

//     };
//     s
// }