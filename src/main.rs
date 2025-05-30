//Hola :3 Cualquier nota sera bien recibida por acá.
//

fn main() {
    //Es buena practica dejar esta weada aquí para saber que todo esta al cien 7u7r
    use crate::auxiliares;
    use crate::negocio;
    use std::io;

    println!("Hello, world!");
    loop_principal();
}

fn loop_principal() {
    use crate::auxiliares;
    use std::io;

    let escuchar_teclado = io::stdin();
    loop {
        let mut nombre: String = String::new();
        println!("Cual sera el nombre del insumo?");
        escuchar_teclado
            .read_line(&mut nombre)
            .expect("Error al leer el teclado");
        println!("Cual es la cantidad en gramos del insumo");
        let cantidad: u32 = auxiliares::no_es_cero();
        println!("Cual es la cantidad minima de gramos");
        let cantidad_minima: u32 = auxiliares::no_es_cero();
        println!("Cual es el precio del insumo");
        let precio: u32 = auxiliares::no_es_cero();
        println!("creando insumo..");
        let mut insumo = negocio::Insumo::nuevo(nombre, cantidad, precio, cantidad_minima);
        println!("cuantos gramos queres usar voludo?");
        let cantidad: u32 = auxiliares::no_es_cero();
        insumo.usar(cantidad).unwrap();
        println!(
            "la cantidad actual de tu insumo es {}",
            insumo.obtener_cantidad()
        )
    }
}

pub mod auxiliares {
    use std::io;

    pub fn no_es_cero() -> u32 {
        loop {
            let mut buffer = String::new(); // Create a new, empty string for each iteration
                                            // This clears the previous input

            // 1. Correct syntax for read_line and expect
            io::stdin()
                .read_line(&mut buffer) // Pass a mutable reference to buffer
                .expect("Error al leer el teclado"); // Correct expect syntax

            // 2. Trim whitespace (including newline) before parsing
            let cantidad: u32 = match buffer.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Entrada inválida. Por favor, introduce un número válido.");
                    continue; // Go to the next iteration of the loop
                }
            };

            // 3. Correct logic for checking if the number is greater than zero
            if cantidad > 0 {
                return cantidad; // Return the valid number and exit the loop
            } else {
                println!("El número no debería ser 0 (o menor)."); // More accurate message
            }
        }
    }

    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum AppError {
        // empezamos escribiendo los tipos de errores que tendremos en la app.
        #[error("Error Personal: {0}")]
        ErrorPersonal(String),
        #[error("Dato Invalido: {0}")]
        DatoInvalido(String),
    }

    pub type AppResult<T> = Result<T, AppError>;
}

pub mod negocio {
    //Esta capa del programa se encargara de la virtualizacion de entidades en memoria y
    //su gestion bajo las reglas logicas del negocio.

    //Estoy dudando como haremos la verificacion de los u32 que no sean 0:
    //pero para este negocio los u32 en no denerian de ser 0.
    use crate::auxiliares::{AppError, AppResult};
    use chrono::{DateTime, TimeZone}; //Esto de acá es para la fecha.
    use uuid::Uuid; // Esta libreria nos viene bien para id, se usan structs de tipo uuid

    //Estructuras de datos que se usaran en Virtualizacion.
    pub struct Insumo {
        //Simulacion de un insumo
        id: Uuid,
        nombre: String,
        cantidad: u32,
        precio: u32,
        cantidad_minima: u32,
    }

    impl Insumo {
        pub fn nuevo(nombre: String, cantidad: u32, precio: u32, cantidad_minima: u32) -> Insumo {
            Insumo {
                id: Uuid::new_v4(),
                nombre,
                cantidad,
                precio,
                cantidad_minima,
            }
        }
        pub fn usar(&mut self, cantidad: u32) -> AppResult<()> {
            if cantidad < self.cantidad {
                self.cantidad -= cantidad;
                return Ok(());
            }
            Err(AppError::ErrorPersonal(
                "No hay suficiente Stock para usar".to_string(),
            ))
        }

        pub fn alerta_cantidad_minima(&self) -> bool {
            self.cantidad <= self.cantidad
        }
        pub fn obtener_cantidad(&self) -> u32 {
            self.cantidad
        }
    }
    //Pregunta: Y si usamos un Hash_map para representar los ingredientes en receta?
    pub struct IngredienteReceta {
        producto_id: Uuid,
        cantidad: u32,
    }

    pub struct Receta {
        id: String,
        nombre: String,
        ingredientes: Vec<IngredienteReceta>,
    }

    pub struct Venta<Tz: chrono::TimeZone> {
        fecha: DateTime<Tz>,
        receta_id: String,
        total: f32,
    }

    pub struct Empleado {
        id: String,
        nombre: String,
        contra_hash: String,
        // rol: RolEnum,
    }

    pub struct Reporte; // de momento lo dejamos como struct unitario
}
