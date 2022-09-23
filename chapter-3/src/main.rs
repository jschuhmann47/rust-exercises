/*
Convert temperatures between Fahrenheit and Celsius.
Generate the nth Fibonacci number.
Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
*/

//use std::default;


fn main() {

    let celsius = 10;
    println!("{celsius}°C en fahrenheit: {}",celsius_a_fahrenheit(celsius));
    let fa = 50;
    println!("{fa}°F en celsius: {}",fahrenheit_a_celsius(fa));
    println!("fibonacci rec: {}",n_fibonacci(0));
    println!("fibonacci rec: {}",n_fibonacci(1));
    println!("fibonacci rec: {}",n_fibonacci(3));
    println!("fibonacci rec: {}",n_fibonacci(7));
    println!("fibonacci rec: {}",n_fibonacci(10));
}

fn celsius_a_fahrenheit(celsius: i32) -> i32 {
    9/5 * celsius + 32
}
fn fahrenheit_a_celsius(fahrenheit: i32) -> i32 {
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
    if n==0 {return 0;}
    if n==1 {
        return n1;
    }else {
        return fibonacci_rec(n1+n2, n1,n-1);
    }   
}

fn n_fibonacci(n: i32) -> i32 {
    return fibonacci_rec(1, 0, n);
}

// fn the_twelve_days_of_christmas() {
//     let gifts = ["A partridge in a pear tree\n","Two turtle doves, and\n","Three french hens\n",
//     "Four calling birds\n","Five golden rings\n","Six geese a-laying\n","Seven swans a-swimming\n", "Eight maids a-milking\n",
//     "Nine ladies dancing\n","Ten lords a-leaping\n","Eleven pipers piping\n","Twelve drummers drumming\n"];
//     let start = "On the X day of Christmas, my true love sent to me\n";

//     for n in 1..12 {
        
//     }

    
// }

// fn number_to_cardinal_string(n: i32) -> String{

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
//         default => panic!("nro invalido")

//     };
//     s.to_string()
// }