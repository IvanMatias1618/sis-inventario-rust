//Hola :3 Cualquier nota sera bien recibida por acá.
//      SIGUIENTES TAREAS ANTES DE INICIAR EL MODULO DE LOOPS:
// //
//      ) refinar pequeños ajustes varios: {
//
//      }

use auxiliares::solicitar_texto;

fn main() {
    use crate::auxiliares;
    use crate::repositorio;
    use crate::servicio;

    let mut almacen = repositorio::AlmacenEnMemoria::nuevo();
    let mut recetario = repositorio::RecetarioEnMemoria::nuevo();

    almacen.cargar();
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
                let insumo = loops::describir_insumo();
                match loops::crear_insumo(insumo, &mut servicio_de_almacen) {
                    Ok(respuesta) => {
                        println!("{}", respuesta);
                        break;
                    }
                    Err(e) => {
                        println!(
                            "error al crear el insumo: {}, deseas volver a intentar?
                            \n 1) volver a intentar  \n2)Volver al menu principal",
                            e
                        );
                        let respuesta = auxiliares::no_es_cero();
                        match respuesta {
                            1 => break,
                            2 => continue,
                            _ => continue,
                        }
                    }
                }
            },
            3 => loop {
                let receta = loops::describir_receta(&servicio_de_almacen);
                match loops::crear_receta(receta, &servicio_de_almacen, &mut servicio_de_recetas) {
                    Ok(info) => {
                        println!("{}", info);
                        break;
                    }
                    Err(e) => {
                        println!(" {}", e);
                        println!("Deseas volver a intentarlo? \n 1) si. \n 2) no, volver al menu.");
                        let res = auxiliares::no_es_cero();
                        match res {
                            1 => continue,
                            2 => break,
                            _ => break,
                        }
                    }
                }
            },
            4 => {
                println!("Que insumo gustas buscar?");
                let busqueda = auxiliares::solicitar_texto();
                let resultados = loops::buscar_insumo(&servicio_de_almacen, &busqueda);
                if resultados.is_empty() {
                    println!(
                        "el insumo: {}, no se ha encontrado en el sistema.",
                        busqueda
                    );
                } else {
                    for resultado in resultados {
                        println!("{}", resultado);
                    }
                }
            }
            5 => {
                println!("Que receta quieres buscar?");
                let busqueda = solicitar_texto();
                let resultados = loops::buscar_receta(&servicio_de_recetas, &busqueda);
                if resultados.is_empty() {
                    println!(
                        "la receta: {}, no se ha encontrado en el sistema.",
                        busqueda
                    );
                } else {
                    for resultado in resultados {
                        println!("{}", resultado);
                    }
                }
            }
            _ => break,
        }
    }
}

pub mod loops {
    //1

    use crate::auxiliares;
    use crate::negocio::*;
    use crate::repositorio;
    use crate::servicio::{ServicioDeAlmacen, ServicioDeRecetas};
    use std::io;

    // Dado que estamos en una cli, estaran separadas las funciones de ui, y las de cli.
    //
    // FUNCIONES DE UI
    pub fn menu() -> u32 {
        loop {
            println!(
                "Elije una opcion:
                 \n1) Salir del programa.
                 \n2) Crear Un Insumo.
                 \n3) Crear una Receta.
                 \n4) Buscar un insumo.
                 \n5) Buscar una receta."
            );
            let res = auxiliares::no_es_cero();
            if res > 5 {
                println!("por favor elije una respuesta dentro de las opciones.");
                continue;
            }
            return res;
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
    pub fn describir_receta(almacen: &ServicioDeAlmacen) -> (String, Vec<(String, f64)>) {
        println!("Como quieres que se llame la receta?");
        let nombre = auxiliares::solicitar_texto();
        let mut ingredientes: Vec<(String, f64)> = Vec::new();
        loop {
            println!("Que ingrediente quieres usar?");
            let insumo = auxiliares::solicitar_texto();
            if almacen.existe(&insumo) {
                println!("cuantos gramos quieres usar de: {}", &insumo);
                let cantidad = auxiliares::no_es_cero();
                let fcantidad: f64 = cantidad as f64;
                let conjunto = (insumo.clone(), fcantidad.clone());
                ingredientes.push(conjunto);
                println!("se usara el insumo: {}, con: {} grs. \n quieres añadir mas ingredientes a la receta?
                \n 1) si. \n2) no.", &insumo, &fcantidad);
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
        receta: (String, Vec<(String, f64)>),
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
        return almacen.buscar(busqueda);
    }

    pub fn buscar_receta(libro: &ServicioDeRecetas, busqueda: &String) -> Vec<String> {
        return libro.buscar(busqueda);
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
    use uuid::Uuid; // Esta libreria nos viene bien para id, se usan structs de tipo uuid

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

    //Estructuras de datos que se usaran en Virtualizacion.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Insumo {
        //Simulacion de un insumo
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
            self.cantidad <= self.cantidad_minima
        }
        pub fn obtener_id(&self) -> &String {
            &self.id
        }
        pub fn obtener_cantidad(&self) -> u32 {
            self.cantidad
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
        ingredientes: Vec<(String, f64)>,
        costo: f64,
    }

    impl Receta {
        pub fn nuevo(
            nombre: String,
            ingredientes: Vec<(String, f64)>,
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
    use std::collections::HashMap;
    use strsim::levenshtein;

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

    pub trait RecetasEnMemoria {
        fn añadir(&mut self, receta: negocio::Receta);
        fn eliminar(&mut self, nombre: &str);
        fn obtener(&mut self, busqueda: &str) -> AppResult<&negocio::Receta>;
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

        fn obtener(&mut self, busqueda: &str) -> AppResult<&negocio::Receta> {
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
            if cantidad == 0 {
                return Err(AppError::DatoInvalido(
                    "la cantidad no puede estar en cero".to_string(),
                ));
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
                    self.repositorio.añadir(nombre.as_str(), nuevo_insumo);
                    Ok(())
                }
                Err(e) => Err(AppError::ErrorPersonal(format!(
                    "ocurrio un problema al intentar crear el insumo: {}",
                    e
                ))),
            }
        }
        pub fn buscar(&self, busqueda: &str) -> Vec<String> {
            let lista = self.repositorio.mostrar_todos();
            let mut resultados = Vec::new();
            resultados = lista
                .clone()
                .into_iter()
                .filter(|nombre| nombre.contains(busqueda))
                .collect();
            let probables = lista
                .into_iter()
                .min_by_key(|insumo| levenshtein(insumo, busqueda));
            match probables {
                Some(opcion) => {
                    resultados.push(opcion.clone());
                    return resultados;
                }
                None => return resultados,
            }
        }

        pub fn existe(&self, busqueda: &String) -> bool {
            let lista = self.repositorio.mostrar_todos();
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
        pub fn obtener(&self, busqueda: &String) -> AppResult<&negocio::Insumo> {
            if self.existe(busqueda) {
                return match self.repositorio.obtener(busqueda) {
                    Ok(insumo) => Ok(insumo),
                    Err(e) => {
                        return Err(AppError::ErrorPersonal(format!(
                            "error al obtener el insumo: {}",
                            busqueda
                        )));
                    }
                };
            }
            return Err(AppError::ErrorPersonal(format!(
                "No existe el insumo: {}",
                busqueda
            )));
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
            ingredientes: Vec<(String, f64)>,
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
                if *cantidad <= 0.0 {
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
                    Ok(insumo) => costo += insumo.costo_por_gramos(*cantidad),
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

        pub fn obtener(&mut self, busqueda: &str) -> AppResult<&negocio::Receta> {
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
    }
}
