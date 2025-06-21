// NOTAS: {
//    1.-
// }
//
// TAREAS:
//    A: Trabajar en los endpoints de la app.
//
//
use negocio::AppError;

fn main() -> Result<(), crate::negocio::AppError> {
    use crate::cli;
    use crate::repositorio;
    use crate::servicio;

    let mut almacen = match repositorio::AlmacenEnMemoria::nuevo("cafeteria") {
        Ok(almacen) => almacen,
        Err(e) => {
            println!("Error al abrir la base de datos porque: {}", e);
            return Err(e);
        }
    };

    let mut recetario = match repositorio::RecetarioEnMemoria::nuevo("cafeteria") {
        Ok(recetario) => recetario,
        Err(e) => {
            println!("Error al abrir la base de datos con el recetario: {}", e);
            return Err(e);
        }
    };

    println!("almacen cargado");
    let mut servicio_de_almacen = servicio::ServicioDeAlmacen::nuevo(Box::new(almacen));
    let mut servicio_de_recetas = servicio::ServicioDeRecetas::nuevo(Box::new(recetario));

    println!(
        "Hola :) \n Bienvenid@ a tu siste de Inventario demo: 1
             \nYa se ha creado el servicio de almacen y recetas."
    );

    //Creamos una funcion predeterminada que permita al usuario
    fn reintentar_o_salir<F>(mut funcion: F) -> ()
    where
        F: FnMut() -> bool,
    {
        loop {
            if funcion() {
                break;
            }
            if cli::reintentar() {
                continue;
            }
            break;
        }
    }

    loop {
        let res = cli::menu();
        match res {
            1 => break,
            2 => reintentar_o_salir(|| cli::crear_insumo(&mut servicio_de_almacen)),
            3 => reintentar_o_salir(|| {
                cli::crear_receta(&mut servicio_de_recetas, &servicio_de_almacen)
            }),
            4 => reintentar_o_salir(|| cli::buscar_insumo(&servicio_de_almacen)),
            5 => reintentar_o_salir(|| cli::buscar_receta(&servicio_de_recetas)),
            6 => cli::ver_insumos(&servicio_de_almacen),
            7 => cli::ver_recetas(&servicio_de_recetas),
            8 => reintentar_o_salir(|| cli::valor_de_insumo(&servicio_de_almacen)),
            9 => {
                reintentar_o_salir(|| cli::receta_valor(&servicio_de_recetas, &servicio_de_almacen))
            }
            10 => reintentar_o_salir(|| cli::eliminar_insumo(&mut servicio_de_almacen)),
            11 => reintentar_o_salir(|| cli::eliminar_receta(&mut servicio_de_recetas)),
            12 => reintentar_o_salir(|| {
                cli::producir_receta(&mut servicio_de_almacen, &servicio_de_recetas)
            }),
            13 => reintentar_o_salir(|| {
                cli::ingredientes_en_recetas(&servicio_de_recetas, &servicio_de_almacen)
            }),
            14 => reintentar_o_salir(|| cli::editar_insumo(&mut servicio_de_almacen)),
            15 => reintentar_o_salir(|| {
                cli::editar_receta(&mut servicio_de_recetas, &servicio_de_almacen)
            }),
            _ => continue,
        }
    }
    return Ok(());
}

pub mod cli {
    use crate::auxiliares::{no_es_cero, solicitar_texto};
    use crate::comandos;
    use crate::servicio::{ServicioDeAlmacen, ServicioDeRecetas};

    //Una pequeña funcion para imprimir el menu.
    pub fn menu() -> u32 {
        loop {
            println!(
                "Elije una opcion:


                 \n                1) Salir del programa.
                 \n2) Crear Un Insumo.                3) Crear unafn obten        \n4) Buscar un insumo.               5) Buscar una receta.
                 \n6) Ver todos los insumos.          7) Ver todas las recetas.
                 \n8) Ver el valor de un Insumo.      9) Ver el valor de una Receta.
                 \n10) Eliminar Insumo.              11) Eliminar Receta.
                 \n12) Producir Receta.              13) Ingredientes en recetas.

fn obtener_nombre_con_id(&self, id: &String) -> AppResult<String>;                 \n14) Editar Insumo.                15) Editar Receta."
            );
            let res = no_es_cero();
            if res > 15 {
                println!("por favor elije una respuesta dentro de las opciones.");
                continue;
            }
            return res;
        }
    }
    pub fn reintentar() -> bool {
        println!("¿Deseas volver a intentar? \n1) Si. \n2) No, volver al menú.");
        loop {
            let res = no_es_cero();
            match res {
                1 => return true,
                2 => return false,
                _ => {
                    println!("por favor responde 1: para salir o 2: para volver a intentar. ");
                    continue;
                }
            }
        }
    }

    pub fn ingredientes_en_recetas(libro: &ServicioDeRecetas, almacen: &ServicioDeAlmacen) -> bool {
        println!("Cual insumo gustas buscar en las recetas?");
        let insumo = solicitar_texto();
        match comandos::insumo_en_recetas(libro, almacen, &insumo) {
            Ok(res) => {
                if res.is_empty() {
                    println!("No se encontraron recetas con el insumo: {}", insumo);
                    return false;
                }
                for receta in res {
                    println!("Receta: {}", receta);
                }
                return true;
            }
            Err(e) => {
                println!("{}", e);
                return false;
            }
        }
    }

    pub fn describir_insumo() -> (String, u32, u32, u32) {
        println!("Hola! que nombre quieres para tu insumo?:");
        let nombre = solicitar_texto();
        println!("cuantos gramos tienes de {}?:", &nombre);
        let cantidad = no_es_cero();
        println!("cual es el costo de '{}' por kilo?:", &nombre);
        let costo = no_es_cero();
        println!(
            "Cual es la cantidad minima que esperas tener en tu inventario del insumo: '{}'",
            &nombre
        );
        let cantidad_minima = no_es_cero();
        return (nombre, cantidad, cantidad_minima, costo);
    }
    pub fn crear_insumo(almacen: &mut ServicioDeAlmacen) -> bool {
        let insumo = describir_insumo();
        match comandos::crear_insumo(insumo, almacen) {
            Ok(respuesta) => {
                println!("{}", respuesta);
                return true;
            }
            Err(e) => {
                println!("Error: {}\nAl crear el insumo.", e);
                return false;
            }
        }
    }
    pub fn describir_receta(almacen: &ServicioDeAlmacen) -> (String, Vec<(String, u32)>) {
        println!("Como quieres que se llame la receta?");
        let nombre = solicitar_texto();
        let mut ingredientes: Vec<(String, u32)> = Vec::new();
        loop {
            println!("Que ingrediente quieres usar?");
            let insumo = solicitar_texto();
            println!("cuantos gramos quieres usar de: {}", &insumo);
            let cantidad = no_es_cero();
            let conjunto = (insumo.clone(), cantidad);
            ingredientes.push(conjunto);
            println!("se usara el insumo: {}, con: {} grs. \nQuieres añadir mas ingredientes a la receta?
            \n 1) si. \n2) no.", &insumo, cantidad);
            let respuesta = no_es_cero();
            match respuesta {
                1 => continue,
                2 => break,
                _ => break,
            }
        }

        return (nombre, ingredientes);
    }
    pub fn crear_receta(libro: &mut ServicioDeRecetas, almacen: &ServicioDeAlmacen) -> bool {
        let receta = describir_receta(almacen);
        return match comandos::crear_receta(receta, almacen, libro) {
            Ok(info) => {
                println!("{}", info);
                true
            }
            Err(e) => {
                println!("{}", e);
                false
            }
        };
    }

    pub fn buscar_insumo(almacen: &ServicioDeAlmacen) -> bool {
        println!("Que insumo gustas buscar?");
        let busqueda = solicitar_texto();
        let resultados = comandos::buscar_insumo(almacen, &busqueda);
        if resultados.is_empty() {
            println!("El insumo: {}, no se ha encontrado.", busqueda);
            return false;
        }
        println!("Resultados: ");
        for resultado in resultados {
            println!("{}", resultado);
        }
        return true;
    }

    pub fn buscar_receta(libro: &ServicioDeRecetas) -> bool {
        println!("Que receta quieres buscar?");
        let busqueda = solicitar_texto();
        let resultados = comandos::buscar_receta(libro, &busqueda);
        if resultados.is_empty() {
            println!("No se encontraron coincidencias.");
            return false;
        }
        println!("Imprimiendo resultados...:");
        for resultado in resultados {
            println!("{}", resultado);
        }
        return true;
    }

    pub fn ver_insumos(almacen: &ServicioDeAlmacen) {
        println!("Buscando insumos. .. . ... .. .");
        let resultados = comandos::ver_todos_los_insumos(almacen);
        if resultados.is_empty() {
            println!("No hay insumos en el almacen.");
        } else {
            for resultado in resultados {
                println!("{}", resultado);
            }
        }
    }

    pub fn ver_recetas(libro: &ServicioDeRecetas) {
        println!("Buscando recetas. .. ... . . . .. .");
        let resultados = comandos::ver_todos_las_recetas(libro);
        if resultados.is_empty() {
            println!("El libro de recetas esta vacio.");
        } else {
            for resultado in resultados {
                println!("{}", resultado);
            }
        }
    }

    pub fn valor_de_insumo(almacen: &ServicioDeAlmacen) -> bool {
        println!("Que insumo gustas buscar?");
        let insumo = solicitar_texto();
        match comandos::valor_de_insumo(&insumo, almacen) {
            Ok(ins) => {
                println!(
                    "Nombre: {}, \nCantidad: {},\nCantidad minima: {}, \nPrecio por kilo: ${}",
                    ins.0, ins.1, ins.2, ins.3
                );
                return true;
            }
            Err(e) => {
                println!("{}", e);
                return false;
            }
        }
    }
    pub fn receta_valor(libro: &ServicioDeRecetas, almacen: &ServicioDeAlmacen) -> bool {
        println!("Que receta gustas buscar?");
        let busqueda = solicitar_texto();
        match comandos::receta_valor(&busqueda, libro, almacen) {
            Ok(receta) => {
                println!("Nombre: {}", receta.0);
                for (ingrediente, cantidad) in receta.1 {
                    println!("Ingrediente: {}\nCantidad: {}", ingrediente, cantidad);
                }
                println!("El costo es de: {}", receta.2);
                return true;
            }
            Err(e) => {
                println!("{}", e);
                return false;
            }
        }
    }

    pub fn eliminar_receta(libro: &mut ServicioDeRecetas) -> bool {
        println!("que receta quieres eliminar?");
        let receta = solicitar_texto();
        match comandos::eliminar_receta(libro, &receta) {
            Ok(_) => {
                return true;
            }
            Err(e) => {
                println!("{}", e);
                return false;
            }
        }
    }

    pub fn eliminar_insumo(almacen: &mut ServicioDeAlmacen) -> bool {
        println!("Que insumo quieres eliminar?");
        let insumo = solicitar_texto();
        match comandos::eliminar_insumo(almacen, &insumo) {
            Ok(_) => {
                return true;
            }
            Err(e) => {
                println!("{}", e);
                return false;
            }
        }
    }

    pub fn producir_receta(almacen: &mut ServicioDeAlmacen, libro: &ServicioDeRecetas) -> bool {
        println!("Que receta quieres producir?");
        let receta = solicitar_texto();
        println!("cuantas unidades quieres producir?");
        let cantidad = no_es_cero();
        match comandos::producir_recetas(almacen, libro, &receta, cantidad) {
            Ok(_) => {
                println!("se han creado: {} {}", receta, cantidad);
                return true;
            }
            Err(e) => {
                println!("{}", e);
                return false;
            }
        }
    }

    pub fn editar_insumo(almacen: &mut ServicioDeAlmacen) -> bool {
        println!("Que insumo quieres editar?");
        let res = solicitar_texto();
        println!("Quieres editar el nombre? \n1) Si. \n2) No.");
        let mut respuesta = no_es_cero();
        let mut nombre: Option<String>;
        match respuesta {
            1 => {
                println!("Que nombre quieres?");
                let nom = solicitar_texto();
                nombre = Some(nom);
            }
            _ => nombre = None,
        }
        let mut cantidad: Option<u32>;
        println!("Quieres editar la cantidad? \n1) Si. \n2) No.");
        respuesta = no_es_cero();
        match respuesta {
            1 => {
                println!("Cual es la nueva cantidad?");
                let cant = no_es_cero();
                cantidad = Some(cant);
            }
            _ => cantidad = None,
        }
        let mut cantidad_minima: Option<u32>;
        println!("Deseas editar la cantidad minima? \n1) Si. \n2) No.");
        respuesta = no_es_cero();
        match respuesta {
            1 => {
                println!("Cual es la nueva cantidad minima?");
                let cant = no_es_cero();
                cantidad_minima = Some(cant);
            }
            _ => cantidad_minima = None,
        }
        let mut precio: Option<u32>;
        println!("Deseas editar el precio? \n1) Si. \n2) No.");
        respuesta = no_es_cero();
        match respuesta {
            1 => {
                println!("Cual es el nuevo precio?");
                let cant = no_es_cero();
                precio = Some(cant);
            }
            _ => precio = None,
        }

        match comandos::editar_insumo(almacen, &res, nombre, cantidad, cantidad_minima, precio) {
            Ok(_) => {
                println!("se ha editado el insumo: {} correctamente.", res);
                return true;
            }
            Err(e) => {
                println!("Error:{}, al editar el insumo: {}", e, res);
                return false;
            }
        }
    }

    pub fn editar_receta(libro: &mut ServicioDeRecetas, almacen: &ServicioDeAlmacen) -> bool {
        println!("Que receta quieres editar?");
        let receta = solicitar_texto();
        let mut nombre: Option<String> = None;
        println!("Deseas cambiar el nombre de la receta? \n1) Si. \n2) No.");
        let mut res = no_es_cero();
        if res == 1 {
            nombre = Some(solicitar_texto());
        }
        println!("Deseas cambiar los ingredientes de la receta? \n1) Si. \n2) No.");
        res = no_es_cero();
        let mut ingredientes: Option<Vec<(String, u32)>> = None;
        if res == 1 {
            let mut n_ingredientes: Vec<(String, u32)> = Vec::new();
            loop {
                println!("Que ingrediente quieres usar?");
                let ingrediente = solicitar_texto();
                println!("Qué cantidad de gramos usaras?");
                let cantidad = no_es_cero();
                let conjunto = (ingrediente, cantidad);
                n_ingredientes.push(conjunto);
                println!("Gustas añadir mas ingredientes? \n1) si, \n2) No.");
                res = no_es_cero();
                if res == 1 {
                    continue;
                }
                ingredientes = Some(n_ingredientes);
                break;
            }
        }
        match libro.editar_receta(almacen, &receta, nombre, ingredientes) {
            Ok(_) => {
                println!("Se ha editado la receta exitosamente.");
                return true;
            }
            Err(e) => {
                println!("{}", e);
                return false;
            }
        }
    }
}

pub mod comandos {
    use crate::negocio::{AppError, AppResult};
    use crate::servicio::{ServicioDeAlmacen, ServicioDeRecetas};

    pub fn crear_insumo(
        insumo: (String, u32, u32, u32),
        almacen: &mut ServicioDeAlmacen,
    ) -> AppResult<String> {
        return match almacen.añadir(insumo.0.clone(), insumo.1, insumo.2, insumo.3) {
            Ok(_) => Ok(format!("se ha creado el insumo: {}", insumo.0)),
            Err(e) => Err(AppError::ErrorPersonal(format!(
                "Error al crear el insumo: {}, error: {}",
                insumo.0, e
            ))),
        };
    }

    pub fn crear_receta(
        receta: (String, Vec<(String, u32)>),
        almacen: &ServicioDeAlmacen,
        libro: &mut ServicioDeRecetas,
    ) -> AppResult<String> {
        return match libro.añadir(receta.0.clone(), receta.1, almacen) {
            Ok(_) => Ok(format!("se ha creado la receta: {}", receta.0)),
            Err(e) => Err(AppError::ErrorPersonal(format!(
                "hubo un error al crear la receta: {}, error: {}",
                receta.0, e
            ))),
        };
    }

    pub fn buscar_insumo(almacen: &ServicioDeAlmacen, busqueda: &String) -> Vec<String> {
        return match almacen.buscar(busqueda) {
            Ok(resultados) => resultados,
            Err(e) => {
                let mut resultados: Vec<String> = Vec::new();
                resultados.push(e.to_string());
                resultados
            }
        };
    }

    pub fn buscar_receta(libro: &ServicioDeRecetas, busqueda: &String) -> Vec<String> {
        return match libro.buscar(busqueda) {
            Ok(res) => res,
            Err(e) => {
                let mut res = Vec::new();
                res.push(e.to_string());
                return res;
            }
        };
    }

    pub fn ver_todos_los_insumos(almacen: &ServicioDeAlmacen) -> Vec<String> {
        return match almacen.mostrar_todos() {
            Ok(resultados) => resultados,
            Err(e) => {
                let mut resultados: Vec<String> = Vec::new();
                resultados.push(e.to_string());
                resultados
            }
        };
    }
    pub fn ver_todos_las_recetas(libro: &ServicioDeRecetas) -> Vec<String> {
        return match libro.mostrar_todos() {
            Ok(res) => res,
            Err(e) => {
                let mut resultado = Vec::new();
                resultado.push(e.to_string());
                return resultado;
            }
        };
    }

    pub fn valor_de_insumo(
        busqueda: &String,
        almacen: &ServicioDeAlmacen,
    ) -> AppResult<(String, u32, u32, u32)> {
        return almacen.mostrar_insumo(busqueda);
    }
    pub fn receta_valor(
        busqueda: &String,
        libro: &ServicioDeRecetas,
        almacen: &ServicioDeAlmacen,
    ) -> AppResult<(String, Vec<(String, u32)>, f64)> {
        return libro.mostrar_receta(busqueda, almacen);
    }

    pub fn eliminar_receta(libro: &mut ServicioDeRecetas, busqueda: &String) -> AppResult<()> {
        return libro.eliminar(busqueda);
    }

    pub fn editar_insumo(
        almacen: &mut ServicioDeAlmacen,
        insumo: &String,
        nombre: Option<String>,
        cantidad: Option<u32>,
        cantidad_minima: Option<u32>,
        costo: Option<u32>,
    ) -> AppResult<()> {
        match almacen.editar_insumo(insumo, nombre, cantidad, cantidad_minima, costo) {
            Ok(_) => return Ok(()),
            Err(e) => Err(AppError::ErrorPersonal(format!(
                "Error: {}. \nAl editar el insumo: {}",
                e, insumo
            ))),
        }
    }

    pub fn editar_receta(
        libro: &mut ServicioDeRecetas,
        receta: &String,
        nombre: Option<String>,
        ingredientes: Option<Vec<(String, u32)>>,
        almacen: &ServicioDeAlmacen,
    ) -> AppResult<()> {
        return match libro.editar_receta(almacen, receta, nombre, ingredientes) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }

    pub fn producir_recetas(
        almacen: &mut ServicioDeAlmacen,
        libro: &ServicioDeRecetas,
        receta: &String,
        cantidad: u32,
    ) -> AppResult<()> {
        return match libro.producir_receta(almacen, receta, cantidad) {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::ErrorPersonal(format!(
                "Error: {}, al producir la receta: {}",
                e, receta
            ))),
        };
    }

    pub fn eliminar_insumo(almacen: &mut ServicioDeAlmacen, busqueda: &String) -> AppResult<()> {
        almacen.eliminar(busqueda)?;
        Ok(())
    }

    pub fn insumo_en_recetas(
        libro: &ServicioDeRecetas,
        almacen: &ServicioDeAlmacen,
        insumo: &String,
    ) -> AppResult<Vec<String>> {
        almacen.existe(insumo)?;
        let id = almacen.obtener_id_con_nombre(insumo)?;
        return libro.insumo_en_recetas(insumo);
    }
}

pub mod auxiliares {
    //1
    use crate::negocio;
    use crate::servicio;
    use std::io;

    pub fn solicitar_texto() -> String {
        //2
        loop {
            //3
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Error al leer el teclado.");
            if buffer.trim().is_empty() {
                //4
                println!("el texto no deberia estar vacio, preuba nuevamente:");
                continue;
            } //4
            return buffer.trim().to_string();
        } //3
    } //2
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
}

pub mod negocio {
    //Esta capa del programa se encargara de la virtualizacion de entidades en memoria y
    //su gestion bajo las reglas logicas del negocio.
    //
    use chrono::{DateTime, TimeZone};
    use serde::{Deserialize, Serialize};
    //Esto de acá es para la fecha.
    use rusqlite::Error as SqlError;
    use thiserror::Error;
    use uuid::Uuid; // Esta libreria nos viene bien para id, se usan structs de tipo uuid

    //ERRORES:

    #[derive(Debug, Error)]
    pub enum AppError {
        // empezamos escribiendo los tipos de errores que tendremos en la app.
        #[error("Error Personal: {0}")]
        ErrorPersonal(String),
        #[error("Dato Invalido: {0}")]
        DatoInvalido(String),
        #[error("Campo Vacio: {0}")]
        CampoVacio(String),
        #[error("Error de Base de datos: {0}")]
        DbError(#[from] SqlError),
    }

    // Encapsulamos la gestion de errores de la aplicacion.
    pub type AppResult<T> = Result<T, AppError>;

    //Estructuras y reglas de datos del negocio

    //INSUMOS:
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Insumo {
        id: String,
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
                id: Uuid::new_v4().to_string(),
                nombre,
                cantidad,
                precio,
                cantidad_minima,
            })
        }

        pub fn crear_desde_db(
            id: String,
            nombre: String,
            cantidad: u32,
            cantidad_minima: u32,
            precio: u32,
        ) -> Result<Self, rusqlite::Error> {
            Ok(Insumo {
                id,
                nombre,
                cantidad,
                cantidad_minima,
                precio,
            })
        }

        pub fn usar(&mut self, cantidad: u32) -> AppResult<()> {
            if cantidad < self.cantidad {
                self.cantidad -= cantidad;
                return Ok(());
            }
            Err(AppError::ErrorPersonal(
                "No hay suficientes gramos para usar".to_string(),
            ))
        }

        pub fn alerta_cantidad_minima(&self) -> bool {
            self.cantidad <= self.cantidad_minima
        }
        pub fn obtener_id(&self) -> String {
            self.id.clone()
        }

        pub fn actualizar_nombre(&mut self, nombre: String) -> AppResult<()> {
            if !nombre.is_empty() {
                self.nombre = nombre;
                Ok(())
            } else {
                return Err(AppError::DatoInvalido(
                    "El nuevo nombre esta vacio".to_string(),
                ));
            }
        }

        pub fn actualizar_cantidad(&mut self, cantidad: u32) -> AppResult<()> {
            if cantidad == 0 {
                return Err(AppError::DatoInvalido(
                    "La cantidad a añadir es 0.".to_string(),
                ));
            }
            self.cantidad = cantidad;
            Ok(())
        }

        pub fn actualizar_cantidad_minima(&mut self, cantidad_minima: u32) -> AppResult<()> {
            if cantidad_minima == 0 {
                return Err(AppError::DatoInvalido(
                    "La cantidad minima no puede ser 0.".to_string(),
                ));
            }
            self.cantidad_minima = cantidad_minima;
            Ok(())
        }

        pub fn actualizar_precio(&mut self, precio: u32) -> AppResult<()> {
            if precio == 0 {
                return Err(AppError::DatoInvalido(
                    "El precio no puede ser 0.".to_string(),
                ));
            }
            self.precio = precio;
            Ok(())
        }

        pub fn obtener_cantidad(&self) -> u32 {
            self.cantidad
        }

        pub fn obtener_cantidad_minima(&self) -> u32 {
            self.cantidad_minima
        }
        pub fn obtener_precio(&self) -> u32 {
            self.precio
        }

        pub fn obtener_costo_por_gramo(&self) -> f64 {
            self.precio as f64 / 1000.00
        }
        pub fn costo_por_gramos(&self, cantidad: f64) -> f64 {
            let gramo_precio = self.obtener_costo_por_gramo();
            gramo_precio * (cantidad)
        }
        pub fn nombre(&self) -> &String {
            &self.nombre
        }
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Receta {
        id: String,
        nombre: String,
        ingredientes: Vec<(String, u32)>,
        costo: f64,
    }

    impl Receta {
        pub fn nuevo(
            nombre: String,
            ingredientes: Vec<(String, u32)>,
            costo: f64,
        ) -> AppResult<Receta> {
            if nombre.is_empty() {
                return Err(AppError::DatoInvalido(
                    "el nombre no deberia estar vacio".to_string(),
                ));
            };
            if ingredientes.is_empty() {
                return Err(AppError::DatoInvalido(
                    "el ingrediente: '{}' no existe".to_string(),
                ));
            }

            let mut receta = Receta {
                id: Uuid::new_v4().to_string(),
                nombre,
                ingredientes,
                costo,
            };
            Ok(receta)
        }
        pub fn desde_db(
            id: String,
            nombre: String,
            ingredientes: Vec<(String, u32)>,
            costo: f64,
        ) -> Self {
            Receta {
                id,
                nombre,
                ingredientes,
                costo,
            }
        }

        pub fn obtener_id(&self) -> String {
            self.id.clone()
        }

        pub fn costo(&self) -> f64 {
            self.costo
        }
        pub fn nombre(&self) -> String {
            self.nombre.clone()
        }
        pub fn ingredientes(&self) -> Vec<(String, u32)> {
            self.ingredientes.clone()
        }
        pub fn actualizar_nombre(&mut self, nombre: String) -> AppResult<()> {
            if nombre.is_empty() {
                return Err(AppError::DatoInvalido(
                    "El nuevo nombre esta vacio".to_string(),
                ));
            }
            self.nombre = nombre;
            Ok(())
        }
        pub fn actualizar_costo(&mut self, costo: f64) -> AppResult<()> {
            if costo == 0.0 {
                return Err(AppError::DatoInvalido(
                    "El nuevo costo esta en 0".to_string(),
                ));
            }
            self.costo = costo;
            Ok(())
        }
        pub fn actualizar_ingredientes(
            &mut self,
            ingredientes: Vec<(String, u32)>,
        ) -> AppResult<()> {
            if ingredientes.is_empty() {
                return Err(AppError::DatoInvalido(
                    "La lista de ingredientes esta vacia".to_string(),
                ));
            }
            self.ingredientes = ingredientes;
            Ok(())
        }
    }

    pub struct Venta<Tz: chrono::TimeZone> {
        fecha: DateTime<Tz>,
        carrito: Vec<(Receta, u32)>,
        //cliente_id: Uuid,
        cliente: String,
        total: f32,
        empleado: Uuid,
    }

    pub struct Empleado {
        id: Uuid,
        nombre: String,
        contra_hash: String,
        rol: String,
    }

    impl Empleado {
        pub fn nuevo(nombre: &String, rol: String, psswd: &str) -> AppResult<Empleado> {
            if nombre.is_empty() {
                return Err(AppError::DatoInvalido(
                    "el nombre no puede estar vacio".to_string(),
                ));
            }
            if rol.is_empty() {
                return Err(AppError::DatoInvalido("El rol esta vacio".to_string()));
            }

            use bcrypt::{hash, verify};
            let contraseña = hash(psswd, 12).expect("Error al encriptar la contraseña");
            Ok(Empleado {
                id: Uuid::new_v4(),
                nombre: nombre.clone(),
                contra_hash: contraseña,
                rol,
            })
        }
    }
}

pub mod repositorio {

    //REPOSITORIO: Aqui se desglosa la logica para la persistencia de datos.
    // Usamos Traits para abstraer la implimentacion de estas funciones y sea una parte modular.

    use crate::{
        negocio::{self, AppError, AppResult, Insumo, Receta},
        servicio::*,
    };
    use rusqlite::{Connection, Error, params};

    pub struct RecetarioEnMemoria {
        conexion: Connection,
    }

    impl RecetarioEnMemoria {
        pub fn nuevo(ruta: &str) -> AppResult<Self> {
            let conexion = Connection::open(ruta)?;
            conexion.execute(
                "CREATE TABLE IF NOT EXISTS recetas (
                    id TEXT NOT NULL UNIQUE,
                    nombre TEXT NOT NULL UNIQUE,
                    costo REAL NOT NULL
                )",
                [],
            )?;
            conexion.execute(
                "CREATE TABLE IF NOT EXISTS ingredientes_en_receta (
                    receta_id TEXT NOT NULL,
                    ingrediente_id TEXT NOT NULL,
                    cantidad INTEGER NOT NULL,
                    PRIMARY KEY (receta_id, ingrediente_id),
                    FOREIGN KEY (receta_id) REFERENCES recetas(id) ON DELETE CASCADE,
                    FOREIGN KEY (ingrediente_id) REFERENCES insumos(id) ON DELETE CASCADE
                )",
                [],
            )?;
            Ok(RecetarioEnMemoria { conexion })
        }
    }

    impl RecetasEnMemoria for RecetarioEnMemoria {
        fn editar_receta(&mut self, receta: negocio::Receta) -> AppResult<()> {
            let transaccion = self.conexion.transaction()?;
            transaccion.execute(
                "UPDATE recetas SET nombre = ?1, costo = ?2 WHERE id = ?3",
                params![receta.nombre(), receta.costo(), receta.obtener_id()],
            )?;
            for (insumo, cantidad) in &receta.ingredientes() {
                transaccion.execute(
                    "UPDATE ingredientes_en_receta SET ingrediente_id = ?1, cantidad = ?2 WHERE receta_id = ?3",
                    params![insumo, cantidad, receta.obtener_id()],
                )?;
            }
            transaccion.commit()?;
            Ok(())
        }

        fn obtener_nombre_con_id(&self, id: &String) -> AppResult<String> {
            let nombre: String = self
                .conexion
                .query_row(
                    "SELECT nombre FROM recetas WHERE id = ?",
                    params![id],
                    |fila| fila.get(0),
                )
                .map_err(|e| match e {
                    rusqlite::Error::QueryReturnedNoRows => {
                        AppError::DatoInvalido(format!("No se encontro el insumo con id: {}", id))
                    }
                    _ => AppError::DbError(e),
                })?;
            Ok(nombre)
        }
        fn obtener_id_con_nombre(&self, nombre: &str) -> AppResult<String> {
            let id: String = self
                .conexion
                .query_row(
                    "SELECT id FROM recetas WHERE nombre = ?",
                    params![nombre],
                    |fila| fila.get(0),
                )
                .map_err(|e| match e {
                    rusqlite::Error::QueryReturnedNoRows => {
                        AppError::DatoInvalido(format!("No se encontro el insumo: {}", nombre))
                    }
                    _ => AppError::DbError(e),
                })?;
            Ok(id)
        }
        fn añadir(&mut self, receta: Receta) -> AppResult<()> {
            let transaccion = self.conexion.transaction()?;

            transaccion.execute(
                "INSERT INTO recetas (id, nombre, costo) VALUES (?1, ?2, ?3)",
                params![receta.obtener_id(), receta.nombre(), receta.costo(),],
            )?;
            for (insumo, cantidad) in &receta.ingredientes() {
                transaccion.execute(
                    "INSERT INTO ingredientes_en_receta (receta_id, ingrediente_id, cantidad)
                    VALUES (?1, ?2, ?3)",
                    params![receta.obtener_id(), insumo, cantidad,],
                )?;
            }
            transaccion.commit()?;
            Ok(())
        }
        fn eliminar(&self, nombre: &str) -> AppResult<()> {
            let id = self.obtener_id_con_nombre(nombre)?;
            let bandera = self
                .conexion
                .execute("DELETE FROM recetas WHERE id =?", params![id])?;
            if bandera == 0 {
                return Err(AppError::ErrorPersonal(format!(
                    "El insumo: {}, no se pudo eliminar.\nNo fue encontrado",
                    nombre
                )));
            }
            Ok(())
        }
        fn obtener(&self, busqueda: &str) -> AppResult<negocio::Receta> {
            let id = self.obtener_id_con_nombre(busqueda)?;

            let mut accion = self.conexion.prepare(
                "SELECT ingrediente_id, cantidad FROM ingredientes_en_receta WHERE receta_id = ?",
            )?;
            let ingredientes_iter = accion.query_map(params![id], |fila| {
                Ok(((fila.get::<_, String>(0)?), (fila.get::<_, u32>(1)?)))
            })?;
            let mut ingredientes: Vec<(String, u32)> = Vec::new();
            for ingrediente_result in ingredientes_iter {
                let (insumo_id, cantidad) = ingrediente_result?;
                ingredientes.push((insumo_id, cantidad));
            }
            self.conexion
                .query_row(
                    "SELECT id, nombre, costo FROM recetas WHERE id = ?",
                    params![id],
                    |fila| {
                        Ok(Receta::desde_db(
                            fila.get(0)?,
                            fila.get(1)?,
                            ingredientes,
                            fila.get(2)?,
                        ))
                    },
                )
                .map_err(|e| match e {
                    rusqlite::Error::QueryReturnedNoRows => AppError::DatoInvalido(format!(
                        "Error al obtener el insumo desde db.: {}",
                        busqueda
                    )),
                    _ => AppError::DbError(e),
                })
        }

        fn listar(&self) -> AppResult<Vec<String>> {
            let mut accion = self
                .conexion
                .prepare("SELECT nombre FROM recetas ORDER BY nombre")?;
            let nombres_iter = accion.query_map([], |fila| fila.get(0))?;
            let mut nombres = Vec::new();
            for nombre in nombres_iter {
                nombres.push(nombre?);
            }
            Ok(nombres)
        }
        fn obtener_todos(&self) -> AppResult<Vec<Receta>> {
            let recetas_lista = self.listar()?;
            let mut recetas = Vec::new();
            for receta in &recetas_lista {
                let item = self.obtener(receta)?;
                recetas.push(item);
            }
            Ok(recetas)
        }
        fn insumo_en_recetas(&self, insumo_id: &String) -> AppResult<Vec<String>> {
            let mut res: Vec<String> = Vec::new();
            let mut accion = self
                .conexion
                .prepare("SELECT receta_id FROM ingredientes_en_receta WHERE ingrediente_id = ?")?;
            let mut filas = accion.query(params![insumo_id])?;

            while let Some(fila_res) = filas.next()? {
                let fila = fila_res;
                let receta_id: String = fila.get(0)?;
                res.push(receta_id);
            }
            Ok(res)
        }
    }

    pub trait RecetasEnMemoria {
        fn editar_receta(&mut self, receta: negocio::Receta) -> AppResult<()>;
        fn insumo_en_recetas(&self, insumo_id: &String) -> AppResult<Vec<String>>;
        fn obtener_id_con_nombre(&self, nombre: &str) -> AppResult<String>;
        fn obtener_nombre_con_id(&self, id: &String) -> AppResult<String>;
        fn añadir(&mut self, receta: negocio::Receta) -> AppResult<()>;
        fn eliminar(&self, nombre: &str) -> AppResult<()>;
        fn obtener(&self, busqueda: &str) -> AppResult<negocio::Receta>;
        fn obtener_todos(&self) -> AppResult<Vec<Receta>>;
        fn listar(&self) -> AppResult<Vec<String>>;
    }

    pub trait Bodega {
        fn añadir(&self, insumo: negocio::Insumo) -> AppResult<()>;
        fn eliminar(&self, nombre: &String) -> AppResult<()>;
        fn obtener(&self, busqueda: &String) -> AppResult<negocio::Insumo>;
        fn mostrar_todos(&self) -> AppResult<Vec<String>>;
        fn obtener_todos(&self) -> AppResult<Vec<Insumo>>;
        fn obtener_id_con_nombre(&self, nombre: &String) -> AppResult<String>;
        fn obtener_nombre_con_id(&self, nombre: &String) -> AppResult<String>;
        fn usar_insumo(&self, cantidad: u32, id: &String) -> AppResult<()>;
        fn editar_insumo(&self, insumo: negocio::Insumo) -> AppResult<()>;
    }

    pub struct AlmacenEnMemoria {
        conexion: Connection,
    }

    impl AlmacenEnMemoria {
        pub fn nuevo(ruta: &str) -> AppResult<Self> {
            let conn = Connection::open(ruta)?;
            conn.execute(
                "CREATE TABLE IF NOT EXISTS insumos (
                    id TEXT NOT NULL UNIQUE,
                    nombre TEXT NOT NULL UNIQUE,
                    cantidad INTEGER NOT NULL,
                    cantidad_minima INTEGER NOT NULL,
                    precio INTEGER NOT NULL
                )",
                [],
            )?;
            Ok(AlmacenEnMemoria { conexion: conn })
        }
    }

    impl Bodega for AlmacenEnMemoria {
        fn usar_insumo(&self, nueva_cantidad: u32, id: &String) -> AppResult<()> {
            let columnas_afectadas = self.conexion.execute(
                "UPDATE insumos SET cantidad = ?1 WHERE id = ?2",
                params![nueva_cantidad, id],
            )?;
            if columnas_afectadas == 0 {
                return Err(AppError::DatoInvalido(format!(
                    "No se encontro el insumo con ID {} para actualizar la cantidad del insumo.",
                    id
                )));
            }
            Ok(())
        }
        fn obtener_nombre_con_id(&self, id: &String) -> AppResult<String> {
            let nombre = self
                .conexion
                .query_row(
                    "SELECT nombre FROM insumos WHERE id = ?",
                    params![id],
                    |fila| fila.get(0),
                )
                .map_err(|e| match e {
                    rusqlite::Error::QueryReturnedNoRows => {
                        AppError::DatoInvalido(format!("No se encontro el insumo con id: {}", id))
                    }
                    _ => AppError::DbError(e),
                })?;
            Ok(nombre)
        }
        fn obtener_id_con_nombre(&self, nombre: &String) -> AppResult<String> {
            let id: String = self
                .conexion
                .query_row(
                    "SELECT id FROM insumos WHERE nombre = ?",
                    params![nombre],
                    |fila| fila.get(0),
                )
                .map_err(|e| match e {
                    rusqlite::Error::QueryReturnedNoRows => AppError::DatoInvalido(format!(
                        "En obtener id: No se encontro el insumo: {}",
                        nombre
                    )),
                    _ => AppError::DbError(e),
                })?;
            Ok(id)
        }
        fn añadir(&self, insumo: negocio::Insumo) -> AppResult<()> {
            self.conexion.execute(
                "INSERT INTO insumos (id, nombre, cantidad, cantidad_minima, precio)
                VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    insumo.obtener_id(),
                    insumo.nombre(),
                    insumo.obtener_cantidad(),
                    insumo.obtener_cantidad_minima(),
                    insumo.obtener_precio()
                ],
            )?;
            let clave = self.conexion.last_insert_rowid();
            Ok(())
        }

        fn editar_insumo(&self, insumo: negocio::Insumo) -> AppResult<()> {
            let afectados = self.conexion.execute(
                "UPDATE insumos SET nombre = ?1, cantidad = ?2, cantidad_minima = ?3, precio = ?4 WHERE id = ?5",
                params![insumo.nombre(),
                insumo.obtener_cantidad(),
                insumo.obtener_cantidad_minima(),
                insumo.obtener_precio(),
                insumo.obtener_id()],
            )?;
            if afectados == 0 {
                return Err(AppError::ErrorPersonal(format!(
                    "No se guardaron los cambios en: {}",
                    insumo.nombre()
                )));
            }
            Ok(())
        }
        // cambiar nombre por id, y remover id del cuerpo
        fn eliminar(&self, nombre: &String) -> AppResult<()> {
            let id = self.obtener_id_con_nombre(nombre)?;
            let funciono = self
                .conexion
                .execute("DELETE FROM insumos WHERE id =?", params![id])?;
            if funciono == 0 {
                return Err(AppError::ErrorPersonal(format!(
                    "El insumo: {}, a eliminar.\nNo fue encontrado.",
                    nombre
                )));
            }
            Ok(())
        }

        fn obtener(&self, busqueda: &String) -> AppResult<negocio::Insumo> {
            let id = self.obtener_id_con_nombre(busqueda)?;
            self.conexion
                .query_row(
                    "SELECT id, nombre, cantidad, cantidad_minima, precio
                 FROM insumos WHERE id = ?",
                    params![id],
                    |row| {
                        Ok(Insumo::crear_desde_db(
                            row.get(0)?,
                            row.get(1)?,
                            row.get(2)?,
                            row.get(3)?,
                            row.get(4)?,
                        )?)
                    },
                )
                .map_err(|e| match e {
                    rusqlite::Error::QueryReturnedNoRows => AppError::DatoInvalido(format!(
                        "1407Inusmo: {}, \nNo encontrado.",
                        busqueda
                    )),
                    _ => AppError::DbError(e),
                })
        }

        fn mostrar_todos(&self) -> AppResult<Vec<String>> {
            let mut accion = self
                .conexion
                .prepare("SELECT nombre FROM insumos ORDER BY nombre")?;
            let nombres_iter = accion.query_map([], |fila| fila.get(0))?;
            let mut nombres = Vec::new();
            for nombre in nombres_iter {
                nombres.push(nombre?);
            }
            Ok(nombres)
        }

        fn obtener_todos(&self) -> AppResult<Vec<Insumo>> {
            let mut accion = self.conexion.prepare(
                "SELECT id, nombre, cantidad, cantidad_minima, precio
                FROM insumos ORDER BY nombre",
            )?;
            let insumo_iter = accion.query_map([], |fila| {
                Ok(Insumo::crear_desde_db(
                    fila.get(0)?,
                    fila.get(1)?,
                    fila.get(2)?,
                    fila.get(3)?,
                    fila.get(4)?,
                )?)
            })?;
            let mut insumos = Vec::new();
            for insumo in insumo_iter {
                insumos.push(insumo?);
            }
            Ok(insumos)
        }
    }
}
pub mod servicio {

    //SERVICIO: proporciona funciones usables por los comandos para conectarse a repositorio y verificar las existencias de productos antes de la creacion de una.
    // Además provee informacion de consulta para los comandos.
    //
    use crate::negocio::{self, AppError, AppResult, Insumo, Receta};
    use crate::repositorio::{Bodega, RecetasEnMemoria};
    use strsim::levenshtein;

    pub struct ServicioDeAlmacen {
        repositorio: Box<dyn Bodega>,
    }

    impl ServicioDeAlmacen {
        pub fn nuevo(repo: Box<dyn Bodega>) -> Self {
            ServicioDeAlmacen { repositorio: repo }
        }

        pub fn reinsertar(
            &mut self,
            id: String,
            nombre: String,
            cantidad: u32,
            cantidad_minima: u32,
            precio: u32,
        ) -> AppResult<()> {
            let insumo =
                negocio::Insumo::crear_desde_db(id, nombre, cantidad, cantidad_minima, precio);
            self.repositorio.añadir(insumo?)?;
            Ok(())
        }

        pub fn añadir(
            &mut self,
            nombre: String,
            cantidad: u32,
            cantidad_minima: u32,
            precio: u32,
        ) -> AppResult<()> {
            if nombre.is_empty() {
                return Err(AppError::DatoInvalido("El nombre esta vacio".to_string()));
            }
            match self.existe(&nombre) {
                Ok(_) => {
                    return Err(AppError::DatoInvalido(format!(
                        "El insumo: {}, ya existe.",
                        nombre
                    )));
                }
                Err(_) => (),
            }
            let insumo = negocio::Insumo::nuevo(nombre.clone(), cantidad, precio, cantidad_minima)?;
            self.repositorio.añadir(insumo)?;
            Ok(())
        }

        pub fn obtener_nombre_con_id(&self, id: &String) -> AppResult<String> {
            return self.repositorio.obtener_nombre_con_id(id);
        }

        pub fn obtener_id_con_nombre(&self, nombre: &String) -> AppResult<String> {
            return self.repositorio.obtener_id_con_nombre(nombre);
        }

        pub fn buscar(&self, busqueda: &String) -> AppResult<Vec<String>> {
            let insumos = self.repositorio.mostrar_todos()?;
            let mut resultados: Vec<String> = Vec::new();
            resultados = insumos
                .clone()
                .into_iter()
                .filter(|receta| receta.contains(busqueda))
                .collect();

            if !resultados.is_empty() {
                return Ok(resultados);
            }
            let probables = insumos
                .into_iter()
                .min_by_key(|receta| levenshtein(receta, busqueda));
            match probables {
                Some(opcion) => {
                    resultados.push(opcion.clone());
                    return Ok(resultados);
                }
                None => return Ok(resultados),
            }
        }

        pub fn existe(&self, busqueda: &String) -> AppResult<()> {
            let lista = self.mostrar_todos()?;
            if lista.contains(busqueda) {
                return Ok(());
            } else {
                return Err(AppError::DatoInvalido(format!(
                    "El insumo: {}, no existe en el almacen.",
                    busqueda
                )));
            }
        }

        pub fn eliminar(&mut self, insumo: &String) -> AppResult<()> {
            self.existe(insumo)?;
            self.repositorio.eliminar(insumo)?;
            Ok(())
        }

        pub fn obtener(&self, busqueda: &String) -> AppResult<negocio::Insumo> {
            self.existe(busqueda)?;
            Ok(self.repositorio.obtener(busqueda)?)
        }

        pub fn usar(&mut self, busqueda: &String, cantidad: u32) -> AppResult<u32> {
            let mut insumo = self.obtener(busqueda)?;

            self.repositorio
                .usar_insumo(cantidad, &insumo.obtener_id())?;
            Ok(insumo.obtener_cantidad())
        }

        pub fn mostrar_todos(&self) -> AppResult<Vec<String>> {
            return self.repositorio.mostrar_todos();
        }

        pub fn mostrar_insumo(&self, busqueda: &String) -> AppResult<(String, u32, u32, u32)> {
            let insumo = self.obtener(busqueda)?;
            let insumo_tupla = (
                insumo.nombre().clone(),
                insumo.obtener_cantidad(),
                insumo.obtener_cantidad_minima(),
                insumo.obtener_precio(),
            );

            Ok(insumo_tupla)
        }

        pub fn editar_insumo(
            &mut self,
            insumo: &String,
            nombre: Option<String>,
            cantidad: Option<u32>,
            cantidad_minima: Option<u32>,
            precio: Option<u32>,
        ) -> AppResult<()> {
            let mut insumo_a_editar = self.obtener(insumo)?;

            if let Some(mut nuevo_nombre) = nombre {
                if nuevo_nombre.is_empty() {
                    return Err(AppError::DatoInvalido(
                        "El nuevo nombre esta vacio.".to_string(),
                    ));
                }
                if nuevo_nombre != *insumo {
                    match self.existe(&nuevo_nombre) {
                        Ok(_) => {
                            return Err(AppError::DatoInvalido(format!(
                                "Ya existe el insumo: {}",
                                nuevo_nombre
                            )));
                        }
                        Err(_) => (),
                    }
                }
                insumo_a_editar.actualizar_nombre(nuevo_nombre);
            }

            if let Some(cant) = cantidad {
                match insumo_a_editar.actualizar_cantidad(cant) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
            if let Some(cant_mnm) = cantidad_minima {
                match insumo_a_editar.actualizar_cantidad_minima(cant_mnm) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
            if let Some(costo) = precio {
                match insumo_a_editar.actualizar_precio(costo) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }

            self.repositorio.editar_insumo(insumo_a_editar)?;
            Ok(())
        }
    }

    pub struct ServicioDeRecetas {
        repositorio: Box<dyn RecetasEnMemoria>,
    }

    impl ServicioDeRecetas {
        pub fn nuevo(repositorio: Box<dyn RecetasEnMemoria>) -> Self {
            ServicioDeRecetas { repositorio }
        }
        pub fn añadir(
            &mut self,
            n_receta: String,
            ingredientes: Vec<(String, u32)>,
            almacen: &ServicioDeAlmacen,
        ) -> AppResult<()> {
            match self.existe(&n_receta) {
                Ok(_) => {
                    return Err(AppError::DatoInvalido(format!(
                        "La receta: {}, ya existe.",
                        n_receta
                    )));
                }
                Err(_) => (),
            }
            let mut costo = 0.0;
            let mut ingredientes_con_id: Vec<(String, u32)> = Vec::new();
            for (nombre, cantidad) in &ingredientes {
                if nombre.is_empty() {
                    return Err(AppError::DatoInvalido(
                        "el nombre del ingrediente esta vacio".to_string(),
                    ));
                }
                if *cantidad == 0 {
                    return Err(AppError::DatoInvalido(
                        "las cantidades no pueden ser menores a 0".to_string(),
                    ));
                }
                almacen.existe(nombre)?;
                let insumo = almacen.obtener(nombre)?;
                let id = insumo.obtener_id();
                ingredientes_con_id.push((id, cantidad.clone()));
                costo += insumo.costo_por_gramos((*cantidad).into())
            }

            let receta = negocio::Receta::nuevo(n_receta, ingredientes_con_id, costo)?;
            self.repositorio.añadir(receta);
            Ok(())
        }

        pub fn editar_receta(
            &mut self,
            almacen: &ServicioDeAlmacen,
            receta: &String,
            nombre: Option<String>,
            ingredientes: Option<Vec<(String, u32)>>,
        ) -> AppResult<()> {
            self.existe(receta)?;
            let mut receta_a_editar = self.obtener(receta)?;
            if let Some(nuevo_nombre) = nombre {
                if nuevo_nombre.is_empty() {
                    return Err(AppError::DatoInvalido(
                        "El nuevo nombre no puede estar vacio.".to_string(),
                    ));
                }

                if nuevo_nombre != *receta {
                    match self.existe(&nuevo_nombre) {
                        Ok(_) => {
                            return Err(AppError::DatoInvalido(format!(
                                "La receta: {}",
                                nuevo_nombre
                            )));
                        }
                        Err(_) => (),
                    }
                }

                receta_a_editar.actualizar_nombre(nuevo_nombre.clone());
            }
            let mut costo = 0.0;
            if let Some(ingr) = ingredientes {
                if ingr.is_empty() {
                    return Err(AppError::DatoInvalido(
                        "La nueva lista de ingredientes esta vacia.".to_string(),
                    ));
                }
                for (ingrediente, cantidad) in &ingr {
                    almacen.existe(ingrediente)?;
                    if *cantidad == 0 {
                        return Err(AppError::DatoInvalido(format!(
                            "En el ingrediente: {}.\nLa cantidad no puede ser 0",
                            ingrediente
                        )));
                    }
                    let insumo = almacen.obtener(ingrediente)?;
                    costo += insumo.costo_por_gramos(*cantidad as f64);
                }
            }
            self.repositorio.editar_receta(receta_a_editar);
            Ok(())
        }

        pub fn buscar(&self, busqueda: &str) -> AppResult<Vec<String>> {
            let recetas = self.repositorio.listar()?;
            let mut resultados: Vec<String> = Vec::new();
            resultados = recetas
                .clone()
                .into_iter()
                .filter(|receta| receta.contains(busqueda))
                .collect();

            if !resultados.is_empty() {
                return Ok(resultados);
            }
            let probables = recetas
                .into_iter()
                .min_by_key(|receta| levenshtein(receta, busqueda));
            match probables {
                Some(opcion) => {
                    resultados.push(opcion.clone());
                    return Ok(resultados);
                }
                None => return Ok(resultados),
            }
        }
        pub fn existe(&self, busqueda: &str) -> AppResult<()> {
            let recetas = self.repositorio.listar()?;
            if recetas.contains(&busqueda.to_string()) {
                return Ok(());
            }
            return Err(AppError::DatoInvalido(format!(
                "La receta: {}\nNo existe en el libro.",
                busqueda
            )));
        }

        pub fn obtener(&self, busqueda: &str) -> AppResult<negocio::Receta> {
            self.existe(busqueda)?;
            let receta = self.repositorio.obtener(busqueda)?;
            Ok(receta)
        }

        pub fn eliminar(&mut self, busqueda: &str) -> AppResult<()> {
            self.existe(busqueda)?;
            self.repositorio.eliminar(busqueda);
            Ok(())
        }
        pub fn mostrar_todos(&self) -> AppResult<Vec<String>> {
            return self.repositorio.listar();
        }

        pub fn mostrar_receta(
            &self,
            busqueda: &String,
            almacen: &ServicioDeAlmacen,
        ) -> AppResult<(String, Vec<(String, u32)>, f64)> {
            self.existe(busqueda)?;
            let receta = self.obtener(busqueda)?;
            let mut ingredientes: Vec<(String, u32)> = Vec::new();
            let ingredientes_receta = receta.ingredientes();
            for (id, cant) in &ingredientes_receta {
                let nombre = almacen.obtener_nombre_con_id(&id)?;
                ingredientes.push((nombre, *cant));
            }
            let conjunto = (receta.nombre().clone(), ingredientes, receta.costo());
            Ok(conjunto)
        }

        pub fn producir_receta(
            &self,
            almacen: &mut ServicioDeAlmacen,
            nombre_receta: &String,
            cantidad: u32,
        ) -> AppResult<()> {
            self.existe(nombre_receta)?;
            let receta = self.obtener(nombre_receta)?;
            for uno in 0..cantidad {
                for (id, cant) in receta.ingredientes() {
                    let nombre = almacen.obtener_nombre_con_id(&id)?;
                    let mut insumo = almacen.obtener(&nombre)?;
                    insumo.usar(cant)?;
                    let nueva_cant = insumo.obtener_cantidad();
                    almacen.usar(&nombre, nueva_cant)?;
                }
            }
            Ok(())
        }
        pub fn insumo_en_recetas(&self, insumo_id: &String) -> AppResult<Vec<String>> {
            return self.repositorio.insumo_en_recetas(insumo_id);
        }
    }
}
