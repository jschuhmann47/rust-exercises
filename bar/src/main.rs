use std::io;

struct Plato{
    id: u32,
    precio_unitario: f32,
    nombre: String
}
impl Plato {
    fn nuevo_plato(id: u32, precio_unitario: f32, nombre: String) -> Plato{
        Plato {id, precio_unitario, nombre}
    }
}

struct Pedido {
    cantidad: u32,
    plato: Plato
}
impl Pedido {
    fn nuevo_pedido(plato: Plato,cantidad: u32) -> Pedido{
        Pedido {cantidad, plato}
    }
}

struct Mesa {
    nro_mesa: u32,
    pedidos: Vec<Pedido>,
    esta_ocupada: bool
}

impl Mesa {
    fn nueva_mesa(nro_mesa: u32) -> Mesa{
        Mesa {
            nro_mesa,
            pedidos: Vec::new(),
            esta_ocupada: false
        }
    }
}


fn main() {
    const CANTIDAD_MESAS: u32 = 16;
    println!("opciones...");
    mostrar_opciones();       
    let opcion_elegida = obtener_opcion_elegida();
    
    println!("{}",opcion_elegida);

    match opcion_elegida {
        1 => print!("el uno"),
        2 => println!("el dos"),
        _ => ()
        
    }

}

fn mostrar_opciones(){
    println!("1. Ver estado de las mesas\n");
    println!("2. Asignar mesas\n");
    println!("3. Ingresar pedido de una mesa\n");
    println!("4. Cerrar mesa\n");
    println!("5. Finalizar por el dia de hoy\n");
    println!("6. Ver el menu\n");
    println!("Elija opcion: ");
}

fn obtener_opcion_elegida() -> u32 {
    loop {
        let mut opcion_elegida: String = String::new();
        io::stdin()
            .read_line(&mut opcion_elegida)
            .expect("Failed to read line");

        let opcion_elegida: u32 = 
        match opcion_elegida.trim().parse(){
            Result::Ok(n) => n,
            Result::Err(_) => {
                println!("opcion invalida ingrese otra!!!");
                continue;
            } 
        };
        return opcion_elegida;
    }
}