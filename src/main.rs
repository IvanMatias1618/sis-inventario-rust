//Hola :3 Cualquier nota sera bien recibida por acá.
//
//      PENDIENTES:
//           fn Usar en ServicioALmacen: guardar los cambios LINEA: 1536
//      SOLUCIONAR: Al modificar un insumo las recetas se rompen.
//             CAMBIAR: de busqueda por nombre a uuid.
//      ) refinar pequeños ajustes varios: {
//
//               Agregar "campò vacio a la lista de errores de Aplicacion"
//      }

use auxiliares::{no_es_cero, solicitar_texto};
use loops::{reintentar, ui_buscar_insumo, ui_editar_insumo};

fn main() {
    use crate::auxiliares;
    use crate::repositorio;
    use crate::servicio;

    let mut almacen = match repositorio::AlmacenEnMemoria::nuevo("insumos") {
        Ok(almacen) => almacen,
        Err(e) => panic!("Error al abrir la base de datos porque: {}", e),
    };
    let mut recetario = repositorio::RecetarioEnMemoria::nuevo();

    println!("almacen cargado");
    let mut servicio_de_almacen = servicio::ServicioDeAlmacen::nuevo(Box::new(almacen));
    let mut servicio_de_recetas = servicio::ServicioDeRecetas::nuevo(Box::new(recetario));

    println!(
        "Hola :) \n Bienvenid@ a tu siste de Inventario demo: 1
             \nYa se ha creado el servicio de almacen y recetas."
    );

    loop {
        let res = loops::menu();
        match res {
            1 => break,
            2 => loop {
                if loops::ui_crear_insumo(&mut servicio_de_almacen) {
                    break;
                }
                if reintentar() {
                    continue;
                }
                break;
            },
            3 => loop {
                if loops::ui_crear_receta(&mut servicio_de_recetas, &servicio_de_almacen) {
                    break;
                }
                if reintentar() {
                    continue;
                }
                break;
            },
            4 => loop {
                if loops::ui_buscar_insumo(&servicio_de_almacen) {
                    break;
                }
                if reintentar() {
                    continue;
                }
                break;
            },
            5 => loop {
                if loops::ui_buscar_receta(&servicio_de_recetas) {
                    break;
                }
                if reintentar() {
                    continue;
                }
                break;
            },
            6 => loops::ver_insumos(&servicio_de_almacen),
            7 => loops::ver_recetas(&servicio_de_recetas),
            8 => loop {
                if loops::ui_insumo_valor(&servicio_de_almacen) {
                    break;
                }
                if reintentar() {
                    continue;
                }
                break;
            },
            9 => loop {
                if loops::ui_receta_valor(&servicio_de_recetas) {
                    break;
                }
                if reintentar() {
                    continue;
                }
                break;
            },
            10 => loop {
                if loops::ui_eliminar_insumo(&mut servicio_de_almacen) {
                    break;
                }
                if loops::reintentar() {
                    continue;
                }
                break;
            },
            11 => loop {
                if loops::ui_eliminar_receta(&mut servicio_de_recetas) {
                    break;
                }
                if reintentar() {
                    continue;
                }
                break;
            },
            12 => loop {
                if loops::ui_producir_receta(&mut servicio_de_almacen, &servicio_de_recetas) {
                    break;
                }
                if reintentar() {
                    continue;
                }
                break;
            },
            13 => loop {
                if loops::ui_editar_insumo(&mut servicio_de_almacen) {
                    break;
                }
                if reintentar() {
                    continue;
                }
                break;
            },
            14 => loop {
                if loops::ui_editar_receta(&mut servicio_de_recetas, &servicio_de_almacen) {
                    break;
                }
                if reintentar() {
                    continue;
                }
                break;
            },
            _ => loop {
                println!("No soy un chihuahua ! \n si soy un chihuahua");
            },
        }
    }
}

pub mod loops {
    //1

    use rusqlite::ffi::SQLITE_SYNC_DATAONLY;

    use crate::auxiliares;
    use crate::auxiliares::no_es_cero;
    use crate::auxiliares::solicitar_texto;
    use crate::negocio::*;
    use crate::repositorio;
    use crate::servicio::{ServicioDeAlmacen, ServicioDeRecetas};
    use std::arch::x86_64::_XCR_XFEATURE_ENABLED_MASK;
    use std::io;

    // Dado que estamos en una cli, estaran separadas las funciones de ui, y las de cli.
    //
    // FUNCIONES DE UI
    pub fn menu() -> u32 {
        loop {
            println!(
                "Elije una opcion:
                 \n                1) Salir del programa.
                 \n2) Crear Un Insumo.                3) Crear una Receta.
                 \n4) Buscar un insumo.               5) Buscar una receta.
                 \n6) Ver todos los insumos.          7) Ver todas las recetas.
                 \n8) Ver el valor de un Insumo.      9) Ver el valor de una Receta.
                 \n10) Eliminar Insumo.              11) Eliminar Receta.
                 \n              12) Producir Receta.
                 \n13) Editar Insumo.                14) Editar Receta."
            );
            let res = auxiliares::no_es_cero();
            if res > 30 {
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

    pub fn ui_crear_insumo(almacen: &mut ServicioDeAlmacen) -> bool {
        let insumo = describir_insumo();
        match crear_insumo(insumo, almacen) {
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
    pub fn describir_insumo() -> (String, u32, u32, u32) {
        println!("Hola! que nombre quieres para tu insumo?:");
        let nombre = auxiliares::solicitar_texto();
        println!("cuantos gramos tienes de {}?:", &nombre);
        let cantidad = auxiliares::no_es_cero();
        println!("cual es el costo de '{}' por kilo?:", &nombre);
        let costo = auxiliares::no_es_cero();
        println!(
            "Cual es la cantidad minima que esperas tener en tu inventario del insumo: '{}'",
            &nombre
        );
        let cantidad_minima = auxiliares::no_es_cero();
        return (nombre, cantidad, cantidad_minima, costo);
    }

    pub fn ui_buscar_insumo(almacen: &ServicioDeAlmacen) -> bool {
        println!("Que insumo gustas buscar?");
        let busqueda = solicitar_texto();
        let resultados = buscar_insumo(almacen, &busqueda);
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

    pub fn ver_insumos(almacen: &ServicioDeAlmacen) {
        println!("Buscando insumos. .. . ... .. .");
        let resultados = ver_todos_los_insumos(almacen);
        if resultados.is_empty() {
            println!("No hay insumos en el almacen.");
        } else {
            for resultado in resultados {
                println!("{}", resultado);
            }
        }
    }

    pub fn ui_insumo_valor(almacen: &ServicioDeAlmacen) -> bool {
        println!("Que insumo buscas?");
        let busqueda = solicitar_texto();
        if !almacen.existe(&busqueda) {
            println!(
                "No se encontró el insumo: {}. \n Buscando coincidencias...",
                &busqueda
            );
            let mut resultados = match almacen.buscar(&busqueda) {
                Ok(insumos) => insumos,
                Err(e) => {
                    println!(
                        "El insumo: {}, No ha sido encontrado en el almacen.",
                        busqueda
                    );
                    return false;
                }
            };
            if resultados.is_empty() {
                println!("No se encontraron coincidencias.");
                return false;
            }
            for resultado in resultados {
                println!("{}", resultado);
            }
            return false;
        }
        return match almacen.mostrar_insumo(&busqueda) {
            Ok(insumo) => {
                println!(
                    "Insumo: {};\n
                    Nombre: {}.\n
                    Cantidad actual: {}.\n
                    Cantidad minima: {}. \n
                    Precio por kilo: {}.",
                    &busqueda, insumo.0, insumo.1, insumo.2, insumo.3
                );
                true
            }
            Err(e) => {
                println!("Error: {}.\nal buscar el insumo: {}", e, busqueda);
                false
            }
        };
    }

    pub fn describir_receta(almacen: &ServicioDeAlmacen) -> (String, Vec<(String, u32)>) {
        println!("Como quieres que se llame la receta?");
        let nombre = auxiliares::solicitar_texto();
        let mut ingredientes: Vec<(String, u32)> = Vec::new();
        loop {
            println!("Que ingrediente quieres usar?");
            let insumo = auxiliares::solicitar_texto();
            if almacen.existe(&insumo) {
                println!("cuantos gramos quieres usar de: {}", &insumo);
                let cantidad = auxiliares::no_es_cero();
                let conjunto = (insumo.clone(), cantidad);
                ingredientes.push(conjunto);
                println!("se usara el insumo: {}, con: {} grs. \nQuieres añadir mas ingredientes a la receta?
                \n 1) si. \n2) no.", &insumo, cantidad);
                let respuesta = auxiliares::no_es_cero();
                match respuesta {
                    1 => continue,
                    2 => break,
                    _ => break,
                }
            } else {
                println!("no se encontro el insumo {}", insumo)
            }
        }

        return (nombre, ingredientes);
    }

    pub fn ui_crear_receta(libro: &mut ServicioDeRecetas, almacen: &ServicioDeAlmacen) -> bool {
        let receta = describir_receta(almacen);
        return match crear_receta(receta, almacen, libro) {
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

    pub fn ui_buscar_receta(libro: &ServicioDeRecetas) -> bool {
        println!("Que receta quieres buscar?");
        let busqueda = &solicitar_texto();
        let resultados = buscar_receta(libro, busqueda);
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

    pub fn ver_recetas(libro: &ServicioDeRecetas) {
        println!("Buscando recetas. .. ... . . . .. .");
        let resultados = ver_todos_las_recetas(libro);
        if resultados.is_empty() {
            println!("El libro de recetas esta vacio.");
        } else {
            for resultado in resultados {
                println!("{}", resultado);
            }
        }
    }

    pub fn ui_receta_valor(libro: &ServicioDeRecetas) -> bool {
        println!("Que receta gustas buscar?");
        let busqueda = solicitar_texto();
        if !libro.existe(&busqueda) {
            println!(
                "No se han encontrado la receta: {}.\nBuscando coincidencias...",
                &busqueda
            );
            let resultados = libro.buscar(&busqueda);
            if resultados.is_empty() {
                println!("No se encontraron coincidencias.");
                return false;
            } else {
                for resultado in resultados {
                    println!("{}", resultado);
                }
            }
            return false;
        }
        return match libro.mostrar_receta(&busqueda) {
            Ok(receta) => {
                println!("Receta: {}\nNombre: {}", busqueda, receta.0);
                for (insumo, cantidad) in receta.1 {
                    println!("Insumo: {} \nCantidad: {}", insumo, cantidad);
                }
                println!("Costo: {}", receta.2);
                true
            }
            Err(e) => {
                println!("Error: {}\nAl obtener la receta: {}", e, busqueda);
                false
            }
        };
    }

    pub fn ui_eliminar_receta(libro: &mut ServicioDeRecetas) -> bool {
        println!("que receta quieres eliminar?");
        let receta = solicitar_texto();
        if !libro.existe(&receta) {
            println!("No existe la receta: {}, en el libro", receta);
            return false;
        }
        match eliminar_receta(libro, &receta) {
            Ok(i) => {
                println!("{}", i);
                return true;
            }
            Err(e) => {
                println!("{}", e);
                return false;
            }
        }
    }

    pub fn ui_eliminar_insumo(almacen: &mut ServicioDeAlmacen) -> bool {
        println!("Que insumo quieres eliminar?");
        let insumo = solicitar_texto();
        if !almacen.existe(&insumo) {
            println!("No existe el insumo: {}, en el almacen.", insumo);
            return false;
        }
        match eliminar_insumo(almacen, &insumo) {
            Ok(i) => {
                println!("{}", i);
                return true;
            }
            Err(e) => {
                println!("{}", e);
                return false;
            }
        }
    }

    pub fn ui_producir_receta(almacen: &mut ServicioDeAlmacen, libro: &ServicioDeRecetas) -> bool {
        println!("Que receta quieres producir?");
        let receta = solicitar_texto();
        if !libro.existe(&receta) {
            println!(
                "No se encontro la receta: {}.\nBuscando similitudes...",
                &receta
            );
            let resultados = libro.buscar(&receta);
            if resultados.is_empty() {
                println!("No se encontraron similitudes con: {}", receta);
                return false;
            } else {
                for resultado in resultados {
                    println!("{}", resultado);
                }
            }
            return false;
        } else {
            println!("cuantas unidades quieres producir?");
            let cantidad = no_es_cero();
            match producir_recetas(almacen, libro, &receta, cantidad) {
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
    }

    pub fn ui_editar_insumo(almacen: &mut ServicioDeAlmacen) -> bool {
        println!("Que insumo quieres editar?");
        let res = solicitar_texto();
        if !almacen.existe(&res) {
            println!("no existe el insumo: {}", res);
            return false;
        }
        println!("Quieres editar el nombre? \n1) Si. \n2) No.");
        let mut respuesta = no_es_cero();
        let mut nombre: Option<String>;
        match respuesta {
            1 => {
                println!("Que nombre quieres?");
                let nom = solicitar_texto();
                if almacen.existe(&nom) {
                    println!("Ya hay un insumo con ese nombre.");
                    return false;
                }
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

        match editar_insumo(almacen, &res, nombre, cantidad, cantidad_minima, precio) {
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

    pub fn ui_editar_receta(libro: &mut ServicioDeRecetas, almacen: &ServicioDeAlmacen) -> bool {
        println!("Que receta quieres editar?");
        let receta = solicitar_texto();
        if !libro.existe(&receta) {
            println!("No se encontro la receta: {}", receta);
            return false;
        }
        let mut nombre: Option<String> = None;
        println!("Deseas cambiar el nombre de la receta? \n1) Si. \n2) No.");
        let mut res = no_es_cero();
        if res == 1 {
            let nombre = Some(solicitar_texto());
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

    //   FUNCIONES DE CLI
    //
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
        return libro.buscar(busqueda);
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
        return libro.mostrar_todos();
    }

    pub fn mostrar_insumo(
        almacen: &ServicioDeAlmacen,
        busqueda: &String,
    ) -> AppResult<(String, u32, u32, u32)> {
        almacen.mostrar_insumo(busqueda)
    }

    pub fn mostrar_receta(
        libro: &ServicioDeRecetas,
        busqueda: &String,
    ) -> AppResult<(String, Vec<(String, u32)>, f64)> {
        libro.mostrar_receta(busqueda)
    }
    pub fn eliminar_receta(libro: &mut ServicioDeRecetas, busqueda: &String) -> AppResult<String> {
        match libro.eliminar(busqueda) {
            Ok(_) => {
                return Ok(format!(
                    "Se ha eliminado la receta: {}, del libro.",
                    busqueda
                ));
            }
            Err(e) => {
                return Err(AppError::ErrorPersonal(format!(
                    "Error al eliminar la receta: {}. \nError: {}",
                    busqueda, e
                )));
            }
        }
    }

    pub fn eliminar_insumo(
        almacen: &mut ServicioDeAlmacen,
        busqueda: &String,
    ) -> AppResult<String> {
        if !almacen.existe(busqueda) {
            return Err(AppError::DatoInvalido(format!(
                "No se encontro el insumo: {}, en el almacen.",
                busqueda
            )));
        }
        match almacen.eliminar(busqueda) {
            Ok(_) => {
                return Ok(format!(
                    "Se ha eliminado la receta: {}, del libro.",
                    busqueda
                ));
            }
            Err(e) => {
                return Err(AppError::ErrorPersonal(format!(
                    "Hubo un error al eliminar el insumo: {}. \nError: {}",
                    busqueda, e
                )));
            }
        }
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
} //1

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

    use chrono::{DateTime, TimeZone};
    use serde::{Deserialize, Serialize};
    //Esto de acá es para la fecha.
    use rusqlite::Error as SqlError;
    use thiserror::Error;
    use uuid::Uuid; // Esta libreria nos viene bien para id, se usan structs de tipo uuid

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

    pub type AppResult<T> = Result<T, AppError>;

    //Estructuras de datos que se usaran en Virtualizacion.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Insumo {
        //Simulacion de un insumo
        id: Option<i64>,
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
                id: None,
                nombre,
                cantidad,
                precio,
                cantidad_minima,
            })
        }

        pub fn crear_desde_db(
            id: i64,
            nombre: String,
            cantidad: u32,
            cantidad_minima: u32,
            precio: u32,
        ) -> Result<Self, rusqlite::Error> {
            Ok(Insumo {
                id: Some(id),
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
        pub fn obtener_id(&self) -> Option<i64> {
            self.id
        }

        pub fn actualizar_nombre(&mut self, nombre: &String) -> AppResult<()> {
            if nombre.is_empty() {
                return Err(AppError::DatoInvalido(
                    "El nuevo nombre esta vacio.".to_string(),
                ));
            }
            self.nombre = nombre.clone();
            Ok(())
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

        pub fn costo(&self) -> f64 {
            self.costo
        }
        pub fn nombre(&self) -> &String {
            &self.nombre
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
    use crate::negocio::{self, AppError, AppResult, Insumo, Receta};
    use rusqlite::{Connection, Error, params};
    use std::collections::HashMap;
    use strsim::levenshtein;

    /*
        pub trait Bodega {
            fn añadir(&mut self, nombre: &str, insumo: negocio::Insumo);
            fn eliminar(&mut self, nombre: &str);
            fn buscar(&self, busqueda: &str) -> Vec<&String>;
            fn obtener(&self, busqueda: &str) -> AppResult<&negocio::Insumo>;
            fn mostrar_todos(&self) -> Vec<String>; //realmente sera un insumo pero hay que ver como)> ;
        }

        pub struct AlmacenEnMemoria {
            bodega: HashMap<String, negocio::Insumo>,
        }

        impl AlmacenEnMemoria {
            pub fn nuevo() -> Self {
                AlmacenEnMemoria {
                    bodega: HashMap::new(),
                }
            }
            pub fn cargar(&mut self) {
                match Insumo::nuevo("leche".to_string(), 120, 100, 30) {
                    Ok(insumo) => {
                        let mut nuevo = insumo;
                        self.añadir(&"leche".to_string(), nuevo)
                    }
                    Err(_) => (),
                }
                match Insumo::nuevo("cafe".to_string(), 1000, 123, 40) {
                    Ok(insumo) => {
                        let mut nuevo = insumo;
                        self.añadir(&"cafe".to_string(), nuevo)
                    }
                    Err(_) => (),
                }
                match Insumo::nuevo("chocolate".to_string(), 120, 100, 30) {
                    Ok(insumo) => {
                        let mut nuevo = insumo;
                        self.añadir(&"chocolate".to_string(), nuevo)
                    }
                    Err(_) => (),
                }
                match Insumo::nuevo("cocoa".to_string(), 1000, 123, 40) {
                    Ok(insumo) => {
                        let mut nuevo = insumo;
                        self.añadir(&"cocoa".to_string(), nuevo)
                    }
                    Err(_) => (),
                }

                match Insumo::nuevo("caramelo".to_string(), 1000, 150, 13) {
                    Ok(insumo) => {
                        let mut nuevo = insumo;
                        self.añadir(&"caramelo".to_string(), nuevo)
                    }
                    Err(_) => (),
                }
                match Insumo::nuevo("vainilla".to_string(), 1000, 123, 40) {
                    Ok(insumo) => {
                        let mut nuevo = insumo;
                        self.añadir(&"vainilla", nuevo)
                    }
                    Err(_) => (),
                }

                match Insumo::nuevo("rompope".to_string(), 120, 100, 30) {
                    Ok(insumo) => {
                        let mut nuevo = insumo;
                        self.añadir(&"rompope".to_string(), nuevo)
                    }
                    Err(_) => (),
                }
                match Insumo::nuevo("matcha".to_string(), 1000, 123, 40) {
                    Ok(insumo) => {
                        let mut nuevo = insumo;
                        self.añadir(&"matcha".to_string(), nuevo)
                    }
                    Err(_) => (),
                }
            }
        }
        impl Bodega for AlmacenEnMemoria {
            fn añadir(&mut self, nombre: &str, insumo: negocio::Insumo) {
                self.bodega.insert(nombre.to_string(), insumo);
            }
            fn eliminar(&mut self, nombre: &str) {
                self.bodega.remove(nombre);
            }
            fn buscar(&self, busqueda: &str) -> Vec<&String> {
                let mut resultados: Vec<&String> = Vec::new();
                resultados = self
                    .bodega
                    .keys()
                    .filter(|nombre| nombre.contains(busqueda))
                    .collect();
                let probables = self
                    .bodega
                    .keys()
                    .min_by_key(|insumo| levenshtein(insumo, busqueda));
                match probables {
                    Some(opcion) => {
                        resultados.push(opcion);
                        return resultados;
                    }
                    None => return resultados,
                }
            }

            fn obtener(&self, busqueda: &str) -> AppResult<&Insumo> {
                match self.bodega.get(busqueda) {
                    Some(insumo) => Ok(&insumo),
                    None => Err(AppError::DatoInvalido(format!(
                        "el insumo: {}, no existe",
                        busqueda
                    ))),
                }
            }

            fn mostrar_todos(&self) -> Vec<String> {
                let mut resultados: Vec<String> = Vec::new();
                for (clave, _) in &self.bodega {
                    resultados.push(clave.clone());
                }
                return resultados;
            }
        }
    */
    pub trait RecetasEnMemoria {
        fn añadir(&mut self, receta: negocio::Receta);
        fn eliminar(&mut self, nombre: &str);
        fn obtener(&self, busqueda: &str) -> AppResult<&negocio::Receta>;
        fn buscar(&self, busqueda: &str) -> Vec<String>;
        fn listar(&self) -> Vec<String>;
    }

    pub struct RecetarioEnMemoria {
        libro: HashMap<String, negocio::Receta>,
    }

    impl RecetarioEnMemoria {
        pub fn nuevo() -> Self {
            RecetarioEnMemoria {
                libro: HashMap::new(),
            }
        }
    }

    impl RecetasEnMemoria for RecetarioEnMemoria {
        fn añadir(&mut self, receta: negocio::Receta) {
            self.libro.insert(receta.nombre().clone(), receta);
        }
        fn eliminar(&mut self, nombre: &str) {
            self.libro.remove(nombre);
        }
        fn listar(&self) -> Vec<String> {
            let mut resultado: Vec<String> = Vec::new();
            for (nombre, _) in &self.libro {
                resultado.push(nombre.clone());
            }
            resultado
        }

        fn obtener(&self, busqueda: &str) -> AppResult<&negocio::Receta> {
            match self.libro.get(busqueda) {
                Some(receta) => Ok(&receta),
                None => Err(AppError::DatoInvalido(format!(
                    "no se encontro la receta: {}",
                    busqueda
                ))),
            }
        }

        fn buscar(&self, busqueda: &str) -> Vec<String> {
            let mut resultados = Vec::new();
            resultados = self
                .libro
                .keys()
                .filter(|nombre| nombre.contains(busqueda))
                .cloned()
                .collect();
            let probables = self
                .libro
                .keys()
                .min_by_key(|insumo| levenshtein(insumo, busqueda));
            match probables {
                Some(opcion) => {
                    resultados.push(opcion.clone());
                    return resultados;
                }
                None => return resultados,
            }
        }
    }

    pub trait Bodega {
        fn añadir(&self, insumo: negocio::Insumo) -> AppResult<()>;
        fn eliminar(&self, nombre: &str) -> AppResult<()>;
        fn obtener(&self, busqueda: &str) -> AppResult<negocio::Insumo>;
        fn mostrar_todos(&self) -> AppResult<Vec<String>>;
        fn obtener_todos(&self) -> AppResult<Vec<Insumo>>;
    }

    pub struct AlmacenEnMemoria {
        conexion: Connection,
    }

    impl AlmacenEnMemoria {
        pub fn nuevo(ruta: &str) -> AppResult<Self> {
            println!("LLegue a la linea 1314, nuevo, almacen en memoria.");
            let conn = match Connection::open(ruta) {
                Ok(conex) => conex,
                Err(e) => {
                    println!("Error al abrir la ruta: {}, \nError: {}", ruta, e);
                    panic!("error en la conexion.");
                }
            };
            println!("LLegue a 1322, voy a crear la tabla");
            conn.execute(
                "CREATE TABLE IF NOT EXISTS insumos (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    nombre TEXT NOT NULL,
                    cantidad INTEGER NOT NULL,
                    cantidad_minima INTEGER NOT NULL,
                    precio INTEGER NOT NULL
                )",
                [],
            )?;
            println!("Logre crear la tabla.");
            Ok(AlmacenEnMemoria { conexion: conn })
        }
        fn obtener_id_con_nombre(&self, nombre: &str) -> AppResult<i64> {
            let id: i64 = self
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
            println!("1350");
            Ok(id)
        }
    }

    impl Bodega for AlmacenEnMemoria {
        fn añadir(&self, insumo: negocio::Insumo) -> AppResult<()> {
            self.conexion.execute(
                "INSERT INTO insumos (nombre, cantidad, cantidad_minima, precio)
                VALUES (?1, ?2, ?3, ?4)",
                params![
                    insumo.nombre(),
                    insumo.obtener_cantidad(),
                    insumo.obtener_cantidad_minima(),
                    insumo.obtener_precio()
                ],
            )?;
            let clave = self.conexion.last_insert_rowid();
            Ok(())
        }
        // cambiar nombre por id, y remover id del cuerpo
        fn eliminar(&self, nombre: &str) -> AppResult<()> {
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

        fn obtener(&self, busqueda: &str) -> AppResult<negocio::Insumo> {
            println!("1387");
            let id = self.obtener_id_con_nombre(busqueda)?;
            println!("1389");
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
            println!("1410");
            let mut accion = self
                .conexion
                .prepare("SELECT nombre FROM insumos ORDER BY nombre")?;
            println!("1414");
            let nombres_iter = accion.query_map([], |fila| fila.get(0))?;
            println!("1416");
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
        pub fn añadir(
            &mut self,
            nombre: String,
            cantidad: u32,
            cantidad_minima: u32,
            precio: u32,
        ) -> AppResult<()> {
            if nombre.is_empty() {
                return Err(AppError::DatoInvalido(
                    "el nombre no puede estar vacio".to_string(),
                ));
            }
            if self.existe(&nombre) {
                return Err(AppError::DatoInvalido(format!(
                    "El insumo: {}, ya existe.",
                    nombre
                )));
            }
            if precio == 0 {
                return Err(AppError::DatoInvalido(
                    "el costo no puede ser cero.".to_string(),
                ));
            }
            if cantidad_minima == 0 || cantidad_minima > cantidad {
                return Err(AppError::DatoInvalido(
                    "la cantidad minima no deberia ser ni cero, ni mayor a la cantidad actual."
                        .to_string(),
                ));
            }
            match negocio::Insumo::nuevo(nombre.clone(), cantidad, precio, cantidad_minima) {
                Ok(insumo) => {
                    let mut nuevo_insumo = insumo;
                    println!("Guardando el insumo");
                    self.repositorio.añadir(nuevo_insumo)?;
                    println!("Se guardo el insumo");
                    Ok(())
                }
                Err(e) => Err(AppError::ErrorPersonal(format!(
                    "ocurrio un problema al intentar crear el insumo: {}",
                    e
                ))),
            }
        }
        pub fn buscar(&self, busqueda: &String) -> AppResult<Vec<String>> {
            let insumos = self.repositorio.mostrar_todos()?;
            println!("1505");
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

        pub fn existe(&self, busqueda: &String) -> bool {
            let mut lista: Vec<String>;
            match self.repositorio.mostrar_todos() {
                Ok(i) => lista = i,
                Err(e) => return false,
            }
            if lista.contains(busqueda) {
                return true;
            } else {
                return false;
            }
        }

        pub fn eliminar(&mut self, insumo: &str) -> AppResult<()> {
            if self.existe(&insumo.to_string()) {
                self.repositorio.eliminar(insumo);
                return Ok(());
            } else {
                return Err(AppError::DatoInvalido(format!(
                    "el insumo: {}, no existe.",
                    insumo
                )));
            }
        }
        pub fn obtener(&self, busqueda: &String) -> AppResult<negocio::Insumo> {
            if self.existe(busqueda) {
                println!("1559");
                return match self.repositorio.obtener(busqueda) {
                    Ok(mut insumo) => Ok(insumo),
                    Err(e) => {
                        return Err(AppError::ErrorPersonal(format!(
                            "error 1564 al obtener el insumo: {}",
                            busqueda
                        )));
                    }
                };
            }

            // IMPORTANTE GUARDAR LOS CAMBIOS DEL CLON   }
            return Err(AppError::ErrorPersonal(format!(
                "No existe el insumo: {}",
                busqueda
            )));
        }

        //NTE GUARDAR LOS CAMBIOS DEL s
        pub fn usar(&mut self, busqueda: &String, cantidad: u32) -> AppResult<u32> {
            if !self.existe(busqueda) {
                return Err(AppError::DatoInvalido(format!(
                    "No existe el insumo: {}, en el almacen.",
                    busqueda
                )));
            }
            match self.obtener(busqueda) {
                Ok(mut insumo) => {
                    insumo.usar(cantidad);
                    return Ok(insumo.obtener_cantidad());
                }
                Err(e) => Err(AppError::ErrorPersonal(format!(
                    "Error al obtener el insumo: {}. \nError: {}",
                    busqueda, e
                ))),
            }
        }
        pub fn mostrar_todos(&self) -> AppResult<Vec<String>> {
            return self.repositorio.mostrar_todos();
        }

        pub fn mostrar_insumo(&self, busqueda: &String) -> AppResult<(String, u32, u32, u32)> {
            if self.existe(busqueda) {
                return match self.obtener(busqueda) {
                    Ok(res) => {
                        let insumo = res;
                        let conjunto = (
                            insumo.nombre().clone(),
                            insumo.obtener_cantidad(),
                            insumo.obtener_cantidad_minima(),
                            insumo.obtener_precio(),
                        );
                        return Ok(conjunto);
                    }
                    Err(e) => Err(AppError::ErrorPersonal(format!(
                        "Error1615 al obtener el insumo: {} \nError: {}",
                        busqueda, e
                    ))),
                };
            } else {
                return Err(AppError::DatoInvalido(format!(
                    "no se encontro el insumo: {}",
                    busqueda
                )));
            }
        }
        pub fn editar_insumo(
            &mut self,
            insumo: &String,
            nombre: Option<String>,
            cantidad: Option<u32>,
            cantidad_minima: Option<u32>,
            precio: Option<u32>,
        ) -> AppResult<()> {
            if !self.existe(insumo) {
                return Err(AppError::DatoInvalido(format!(
                    "El insumo: {}, no esta en el almacen.",
                    insumo
                )));
            }
            let mut insumo_a_editar: negocio::Insumo;
            match self.obtener(insumo) {
                Ok(i) => insumo_a_editar = i.clone(),
                Err(e) => {
                    return Err(AppError::ErrorPersonal(format!(
                        "Error: {}\nAl obtener el insumo: {}",
                        e, insumo
                    )));
                }
            }

            let mut clave = insumo.clone();

            if let Some(mut nuevo_nombre) = nombre {
                if nuevo_nombre.is_empty() {
                    return Err(AppError::DatoInvalido(
                        "El nuevo nombre esta vacio.".to_string(),
                    ));
                }
                if nuevo_nombre != *insumo && self.existe(&nuevo_nombre) {
                    return Err(AppError::DatoInvalido(format!(
                        "Ya existe el insumo: {} ",
                        nuevo_nombre
                    )));
                }
                if nuevo_nombre != *insumo {
                    self.repositorio.eliminar(insumo)?;
                }
                insumo_a_editar.actualizar_nombre(&nuevo_nombre);
                clave = nuevo_nombre;
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

            self.repositorio.añadir(insumo_a_editar)?;
            Ok(())
        }
    }

    pub struct ServicioDeRecetas {
        repositorio: Box<dyn RecetasEnMemoria>,
    }

    impl ServicioDeRecetas {
        pub fn nuevo(repositorio: Box<dyn RecetasEnMemoria>) -> Self {
            ServicioDeRecetas {
                repositorio: repositorio,
            }
        }
        pub fn añadir(
            &mut self,
            n_receta: String,
            ingredientes: Vec<(String, u32)>,
            almacen: &ServicioDeAlmacen,
        ) -> AppResult<()> {
            if n_receta.is_empty() {
                return Err(AppError::DatoInvalido(
                    "el nombre de la receta esta vacio".to_string(),
                ));
            }
            let mut costo = 0.0;
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
                if !almacen.existe(nombre) {
                    return Err(AppError::DatoInvalido(format!(
                        "el insumo: {}, no existe.",
                        &nombre
                    )));
                }
                match almacen.obtener(nombre) {
                    Ok(insumo) => costo += insumo.costo_por_gramos((*cantidad).into()),
                    Err(e) => {
                        return Err(AppError::ErrorPersonal(format!(
                            "error: {}, al obtener el insumo: {}",
                            e, nombre
                        )));
                    }
                }
            }

            match negocio::Receta::nuevo(n_receta.clone(), ingredientes, costo) {
                Ok(receta) => {
                    let nuevo = receta;
                    self.repositorio.añadir(nuevo);
                    Ok(())
                }
                Err(e) => {
                    return Err(AppError::ErrorPersonal(format!(
                        "hubo un error al añadir la receta: {}, al repo {}",
                        n_receta, e
                    )));
                }
            }
        }

        pub fn editar_receta(
            &mut self,
            almacen: &ServicioDeAlmacen,
            receta: &String,
            nombre: Option<String>,
            ingredientes: Option<Vec<(String, u32)>>,
        ) -> AppResult<()> {
            if !self.existe(receta) {
                return Err(AppError::DatoInvalido(format!(
                    "No se encontro en el libro, la receta: {}",
                    receta
                )));
            }
            let mut receta_a_editar: negocio::Receta;
            match self.obtener(receta) {
                Ok(i) => receta_a_editar = i.clone(),
                Err(e) => {
                    return Err(AppError::ErrorPersonal(format!(
                        "Error al obtener el insumo: {}",
                        receta
                    )));
                }
            }
            let mut clave = receta.clone();
            if let Some(nuevo_nombre) = nombre {
                if nuevo_nombre.is_empty() {
                    return Err(AppError::DatoInvalido(
                        "El nuevo nombre no puede estar vacio.".to_string(),
                    ));
                }
                if nuevo_nombre != *receta && self.existe(receta) {
                    return Err(AppError::DatoInvalido(format!(
                        "El nuevo nombre coincide con otra receta."
                    )));
                }
                if nuevo_nombre != *receta {
                    self.repositorio.eliminar(&nuevo_nombre);
                }
                receta_a_editar.actualizar_nombre(nuevo_nombre.clone());
                clave = nuevo_nombre;
            }
            let mut costo = 0.0;
            if let Some(ingr) = ingredientes {
                if ingr.is_empty() {
                    return Err(AppError::DatoInvalido(
                        "La nueva lista de ingredientes esta vacia.".to_string(),
                    ));
                }
                for (ingrediente, cantidad) in &ingr {
                    if !almacen.existe(ingrediente) {
                        return Err(AppError::DatoInvalido(format!(
                            "El insumo: {}, no existe.",
                            ingrediente
                        )));
                    }
                    if *cantidad == 0 {
                        return Err(AppError::DatoInvalido(format!(
                            "En el ingrediente: {}.\nLa cantidad no puede ser 0",
                            ingrediente
                        )));
                    }
                    match almacen.obtener(ingrediente) {
                        Ok(insumo) => costo += insumo.costo_por_gramos(*cantidad as f64),
                        Err(e) => return Err(e),
                    }
                }
            }
            self.repositorio.añadir(receta_a_editar);
            Ok(())
        }

        pub fn buscar(&self, busqueda: &str) -> Vec<String> {
            let recetas = self.repositorio.listar();
            let mut resultados: Vec<String> = Vec::new();
            resultados = recetas
                .clone()
                .into_iter()
                .filter(|receta| receta.contains(busqueda))
                .collect();

            if !resultados.is_empty() {
                return resultados;
            }
            let probables = recetas
                .into_iter()
                .min_by_key(|receta| levenshtein(receta, busqueda));
            match probables {
                Some(opcion) => {
                    resultados.push(opcion.clone());
                    return resultados;
                }
                None => return resultados,
            }
        }
        pub fn existe(&self, busqueda: &str) -> bool {
            let recetas = self.repositorio.listar();
            if recetas.contains(&busqueda.to_string()) {
                return true;
            }
            return false;
        }

        pub fn obtener(&self, busqueda: &str) -> AppResult<&negocio::Receta> {
            if self.existe(busqueda) {
                return match self.repositorio.obtener(busqueda) {
                    Ok(receta) => Ok(receta),
                    Err(e) => Err(AppError::ErrorPersonal(format!(
                        "error: {}, \nAl obtener el insumo: {}",
                        e, busqueda
                    ))),
                };
            }
            return Err(AppError::DatoInvalido(format!(
                "no se encontro la receta: {}",
                busqueda
            )));
        }

        pub fn obtener_clon(&self, busqueda: &String) -> AppResult<&mut negocio::Receta> {
            if self.existe(busqueda) {
                match self.repositorio.obtener(busqueda) {
                    Ok(receta) => Ok(receta.clone()),
                    Err(e) => Err(AppError::ErrorPersonal(format!(
                        "Error al obtener la receta: {}. \n {}",
                        busqueda, e
                    ))),
                };
            }
            return Err(AppError::DatoInvalido(format!(
                "le receta: {}, No existe en el libro.",
                busqueda
            )));
        }

        pub fn eliminar(&mut self, busqueda: &str) -> AppResult<()> {
            if self.existe(busqueda) {
                self.repositorio.eliminar(busqueda);
                return Ok(());
            }
            return Err(AppError::DatoInvalido(format!(
                "Error al eliminar.: \nNo se encontro la receta: {}",
                busqueda
            )));
        }
        pub fn mostrar_todos(&self) -> Vec<String> {
            return self.repositorio.listar();
        }

        pub fn mostrar_receta(
            &self,
            busqueda: &String,
        ) -> AppResult<(String, Vec<(String, u32)>, f64)> {
            if self.existe(busqueda) {
                return match self.obtener(busqueda) {
                    Ok(receta) => {
                        let conjunto = (
                            receta.nombre().clone(),
                            receta.ingredientes(),
                            receta.costo(),
                        );
                        return Ok(conjunto);
                    }
                    Err(e) => Err(AppError::ErrorPersonal(format!(
                        "Error al obtener la receta: {}. \nError: {}",
                        busqueda, e
                    ))),
                };
            } else {
                return Err(AppError::DatoInvalido(format!(
                    "no se encontro el insumo: {}",
                    busqueda
                )));
            }
        }
        pub fn producir_receta(
            &self,
            almacen: &mut ServicioDeAlmacen,
            nombre_receta: &String,
            cantidad: u32,
        ) -> AppResult<()> {
            if self.existe(nombre_receta) {
                match self.obtener(nombre_receta) {
                    Ok(receta) => {
                        for producto in 0..cantidad {
                            for (nombre, cant) in receta.ingredientes() {
                                if !almacen.existe(&nombre) {
                                    return Err(AppError::DatoInvalido(format!(
                                        "el insumo: {}, no esta en almacen.",
                                        nombre
                                    )));
                                }
                                match almacen.obtener(&nombre) {
                                    Ok(mut insumo) => match insumo.usar(cant) {
                                        Ok(_) => continue,

                                        Err(e) => {
                                            return Err(AppError::ErrorPersonal(format!(
                                                "Error: {}. \nAl usar el insumo: {}",
                                                e, nombre
                                            )));
                                        }
                                    },
                                    Err(e) => {
                                        return Err(AppError::ErrorPersonal(format!(
                                            "Error al obtener el insumo: {} del almacen.",
                                            nombre
                                        )));
                                    }
                                }
                            }
                        }
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(AppError::ErrorPersonal(format!(
                            "Error al obtener la receta: {}, \nError: {}",
                            nombre_receta, e
                        )));
                    }
                }
            }
            return Err(AppError::DatoInvalido(format!(
                "La receta: {} no existe en el libro.",
                nombre_receta
            )));
        }
    }
}
