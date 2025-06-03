//Hola :3 Cualquier nota sera bien recibida por acá.
//

fn main() {
    //Es buena practica dejar esta weada aquí para saber que todo esta al cien 7u7

    println!("Hello, world!");
    loop_principal::principal();
}

pub mod loop_principal {

    use crate::auxiliares;
    use crate::negocio;
    use std::io;

    pub fn principal() {
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
            let mut insumo = match negocio::Insumo::nuevo(nombre, cantidad, precio, cantidad_minima)
            {
                Ok(insumo) => insumo,
                Err(e) => {
                    println!("ocurrio un error al crear el insumo: {}", e);
                    continue;
                }
            };
            println!("cuantos gramos queres usar voludo?");
            let cantidad: u32 = auxiliares::no_es_cero();
            match insumo.usar(cantidad) {
                Ok(_) => (),
                Err(e) => {
                    println!("ocurrio un error al usar el insumo: {}", e);
                    continue;
                }
            };
            println!(
                "la cantidad actual de tu insumo es {}",
                insumo.obtener_cantidad()
            )
        }
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

    use core::f64;

    //IMPLEMENTAR VALIDACIONES :3
    //
    //podriamos pensar en un validador o algo por el estilo, que pueda manejar datos genericos y
    //compruebe las reglas de negocio.
    //
    //   ERRORES UwU
    //
    //pensemos en como vamos a lidiar con los errores de validacion, podriamos llamar al validador
    //antes que crear la instancia. o devolver AppError para casi todo :u
    //
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
        pub fn nuevo(
            nombre: String,
            cantidad: u32,
            precio: u32,
            cantidad_minima: u32,
        ) -> AppResult<Insumo> {
            let nombre = if !nombre.is_empty() {
                nombre
            } else {
                return Err(AppError::DatoInvalido(
                    "el nombre no puede estar vacio".to_string(),
                ));
            };
            let cantidad = if cantidad > 0 {
                cantidad
            } else {
                return Err(AppError::DatoInvalido(
                    "La cantidad no puede ser 0.".to_string(),
                ));
            };
            let precio = if precio > 0 {
                precio
            } else {
                return Err(AppError::DatoInvalido(
                    "el precio no debe ser 0.".to_string(),
                ));
            };
            let cantidad_minima = if cantidad_minima > 0 {
                cantidad_minima
            } else {
                return Err(AppError::DatoInvalido(
                    " la cantidad_minima no deberia ser 0".to_string(),
                ));
            };

            Ok(Insumo {
                id: Uuid::new_v4(),
                nombre,
                cantidad,
                precio,
                cantidad_minima,
            })
        }
        pub fn usar(&mut self, cantidad: u32) -> AppResult<()> {
            if cantidad < self.cantidad {
                self.cantidad -= cantidad;
                println!("se han usado: {} gramos", cantidad);
                if self.alerta_cantidad_minima() {
                    println!("Alerta! la cantidad del insumo es baja.: {}", self.cantidad);
                }
                return Ok(());
            }
            Err(AppError::ErrorPersonal(
                "No hay suficiente Stock para usar".to_string(),
            ))
        }

        pub fn alerta_cantidad_minima(&self) -> bool {
            self.cantidad <= self.cantidad
        }
        pub fn obtener_id(&self) -> &Uuid {
            &self.id
        }
        pub fn obtener_cantidad(&self) -> u32 {
            self.cantidad
        }
        pub fn obtener_costo_por_gramo(&self) -> f64 {
            self.precio as f64 / 1000.00
        }
    }

    pub struct Receta<'a> {
        id: Uuid,
        nombre: String,
        ingredientes: Vec<(&'a Insumo, u32)>,
        costo: f64,
    }

    impl<'a> Receta<'a> {
        pub fn nuevo(nombre: String, ingredientes: Vec<(&Insumo, u32)>) -> AppResult<Receta> {
            let nombre = if !nombre.is_empty() {
                nombre
            } else {
                return Err(AppError::DatoInvalido(
                    "el nombre no deberia estar vacio".to_string(),
                ));
            };
            let ingredientes = if !ingredientes.is_empty() {
                ingredientes
            } else {
                return Err(AppError::DatoInvalido(
                    "la lista de ingredientes esta vacia".to_string(),
                ));
            };
            let mut receta = Receta {
                id: Uuid::new_v4(),
                nombre,
                ingredientes,
                costo: 0.0,
            };
            receta.calcularcosto();
            Ok(receta)
        }

        fn calcularcosto(&mut self) {
            let mut costo: f64 = 0.0;
            for (insumo, cantidad) in &self.ingredientes {
                costo += insumo.obtener_costo_por_gramo() * (*cantidad as f64);
            }
            self.costo = costo
        }
    }

    pub struct Venta<Tz: chrono::TimeZone> {
        fecha: DateTime<Tz>,
        //carrito: Vec<Receta>,
        //cliente_id: Uuid,
        //cliente: String,
        total: f32,
        empleado: Uuid,
    }

    pub struct Empleado {
        id: Uuid,
        nombre: String,
        contra_hash: String,
        rol: String,
    }

    impl Empleado {}

    pub struct Reporte {
        operador: &'static Uuid,
    } //
}
