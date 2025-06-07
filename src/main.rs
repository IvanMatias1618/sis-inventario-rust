//Hola :3 Cualquier nota sera bien recibida por acá.
//

fn main() {
    //Es buena practica dejar esta weada aquí para saber que todo esta al cien 7u7
    loops::principal();
}

pub mod loops {
    //1

    use crate::auxiliares;
    use crate::negocio;
    use crate::servicio;
    use std::io;

    pub fn principal() {
        //2

        let mut almacen = servicio::Almacen::nuevo();
        let mut recetario = servicio::Recetario::nuevo();

        loop {
            //3
            println!("Hola! Bienvenid@ a tu inventario :3");
            println!(
                "que queres hacer pequeñ@ amig@?  \n
                1) Salir del programa. \n
                2) Crear un insumo.  \n
                3) Crear una receta. "
            );
            let numero: u32 = auxiliares::no_es_cero();
            match numero {
                //4
                1 => break,
                2 => {
                    //5
                    println!("Que nombre quieres para tu Insumo?");
                    let nombre: String = auxiliares::solicitar_texto();
                    let mut insumo = crear_insumo(&nombre);
                    almacen.añadir(&nombre, insumo);
                } //5
                3 => {
                    //5
                    let mut nombre: String = String::new();
                    println!("Que nombre quieres para tu receta?=");
                    nombre = auxiliares::solicitar_texto();
                    let mut ingredientes: Vec<(String, f64)> = solicitar_ingredientes(&almacen);
                    let mut insumos: Vec<(&negocio::Insumo, f64)> = Vec::new();
                    for (ingrediente, cantidad) in &ingredientes {
                        match almacen.clave_insumo(ingrediente.as_str()) {
                            Ok(insumo) => {
                                let conjunto = (insumo, *cantidad);
                                insumos.push(conjunto);
                            }
                            Err(e) => {
                                println!("Ocurrio un error: {}", e);
                                continue;
                            }
                        }
                    }

                    match negocio::Receta::nuevo(nombre.clone(), insumos) {
                        Ok(receta) => {
                            recetario.añadir(&nombre, receta);
                            println!("la receta {} se ha agregado al recetario", nombre);
                            continue;
                        }
                        Err(e) => {
                            println!("hubo un error al crear la receta {}, Error: {}", nombre, e);
                            continue;
                        }
                    }
                } //5
                _ => continue,
            } //4
        } //3
    } //2
    pub fn crear_insumo(nombre: &String) -> negocio::Insumo {
        //2
        loop {
            //3
            println!("Cual es la cantidad en gramos del insumo");
            let cantidad: u32 = auxiliares::no_es_cero();
            println!("Cual es la cantidad minima de gramos");
            let cantidad_minima: u32 = auxiliares::no_es_cero();
            println!("Cual es el precio del insumo");
            let precio: u32 = auxiliares::no_es_cero();
            println!("creando insumo..");
            match negocio::Insumo::nuevo(nombre.clone(), cantidad, precio, cantidad_minima) {
                //4
                Ok(insumo) => return insumo,
                Err(e) => {
                    //5
                    println!("ocurrio un error al crear el insumo: {}", e);
                    continue;
                }
            } //4
        } //3
    } //2
    pub fn solicitar_ingredientes(almacen: &servicio::Almacen) -> Vec<(String, f64)> {
        //2
        let mut ingredientes: Vec<(String, f64)> = Vec::new();
        loop {
            //3
            println!("que ingrediente quieres usar para tu receta?");
            let mut ingrediente: String = auxiliares::solicitar_texto();
            almacen.buscar_clave(&ingrediente.as_str());
            println!("ingresa nuevamente el nombre del insumo:");
            match almacen.clave_insumo(&ingrediente.as_str()) {
                //4
                Ok(_) => (),
                Err(e) => {
                    println!("ocurrio un error al buscar el insumo: {}", e);
                    continue;
                }
            } //4
            println!(
                "Quieres escribir nuevamente el nombre de tu insumo? \n
                1) si \n
                 2) no, seguir con este nombre"
            );
            let mut res = auxiliares::no_es_cero();
            match res {
                //4
                1 => continue,
                2 => (),
                3 => continue,
                _ => continue,
            } //4 
            match almacen.clave_insumo(&ingrediente.as_str()) {
                //4
                Ok(_) => (),
                Err(e) => {
                    println!(
                        "ocurrio un error al buscar el insumo {}: {}",
                        ingrediente, e
                    );
                    continue;
                }
            } //4
            println!("cuantos gramos queres usar?");
            res = auxiliares::no_es_cero();
            let res: f64 = res as f64;
            let conjunto = (ingrediente, res);
            ingredientes.push(conjunto);
            println!(
                "Quieres añadir mas ingredientes a esta receta?\n
                1) si. \n
                2) no, es todo."
            );
            let res = auxiliares::no_es_cero();
            match res {
                //4
                1 => continue,
                2 => return ingredientes,
                _ => continue,
            } //4
        } //3
    } //2
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
            return buffer;
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
    use chrono::{DateTime, TimeZone};
    use serde::{Deserialize, Serialize};
    //Esto de acá es para la fecha.
    use uuid::Uuid; // Esta libreria nos viene bien para id, se usan structs de tipo uuid
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
        pub fn nuevo(nombre: String, insumos: Vec<(&Insumo, f64)>) -> AppResult<Receta> {
            if nombre.is_empty() {
                return Err(AppError::DatoInvalido(
                    "el nombre no deberia estar vacio".to_string(),
                ));
            };
            if insumos.is_empty() {
                return Err(AppError::DatoInvalido(
                    "el ingrediente: '{}' no existe".to_string(),
                ));
            }
            let costo = Receta::calcular_costo(&insumos);

            let mut ingredientes: Vec<(String, f64)> = Vec::new();
            for (insumo, cantidad) in &insumos {
                let conjunto = (insumo.nombre().clone(), *cantidad);
                ingredientes.push(conjunto);
            }
            let mut receta = Receta {
                id: Uuid::new_v4().to_string(),
                nombre,
                ingredientes,
                costo,
            };
            Ok(receta)
        }

        fn calcular_costo(ingredientes: &Vec<(&Insumo, f64)>) -> f64 {
            let mut costo: f64 = 0.0;
            for (insumo, cantidad) in ingredientes {
                costo += insumo.costo_por_gramos(*cantidad);
                //espera, como llamamos al almacen :u
            }
            costo
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

pub mod servicio {
    use crate::auxiliares::{AppError, AppResult, no_es_cero, solicitar_texto};
    use crate::negocio::{Insumo, Receta};
    use std::collections::HashMap;

    pub struct Almacen {
        bodega: HashMap<String, Insumo>,
    }

    impl Almacen {
        pub fn nuevo() -> Self {
            Almacen {
                bodega: HashMap::new(),
            }
        }
        pub fn añadir(&mut self, clave: &String, insumo: Insumo) {
            self.bodega.insert(clave.clone(), insumo);
        }
        pub fn buscar_clave(&self, busqueda: &str) {
            use strsim::levenshtein;
            let probables: Vec<_> = self
                .bodega
                .keys()
                .filter(|nombre| nombre.contains(busqueda))
                .collect();
            println!("Coincidencias: {:?}", probables);

            let probables = self
                .bodega
                .keys()
                .min_by_key(|nombre| levenshtein(nombre, busqueda));
            match probables {
                Some(nombre) => println!("O quisiste decir: {}?", nombre),
                None => println!("No se encontraron coincidencias. "),
            };
        }
        pub fn clave_insumo(&self, nombre: &str) -> AppResult<&Insumo> {
            match self.bodega.get(nombre) {
                Some(insumo) => return Ok(&insumo),
                None => {
                    return Err(AppError::DatoInvalido(format!(
                        "no se encontro el insumo {}",
                        nombre
                    )));
                }
            }
        }
    }
    pub struct Recetario {
        recetas: HashMap<String, Receta>,
    }

    impl Recetario {
        pub fn nuevo() -> Self {
            Recetario {
                recetas: HashMap::new(),
            }
        }

        pub fn añadir(&mut self, nombre: &String, receta: Receta) -> AppResult<()> {
            if *nombre != receta.nombre().clone() {
                return Err(AppError::DatoInvalido(format!(
                    "la receta: {} no existe. te refieres a: {}?",
                    nombre,
                    receta.nombre()
                )));
            }
            self.recetas.insert(nombre.clone(), receta);
            Ok(())
        }
        pub fn buscar_clave(&self, busqueda: &str) {
            use strsim::levenshtein;
            let probables: Vec<_> = self
                .recetas
                .keys()
                .filter(|nombre| nombre.contains(busqueda))
                .collect();
            println!("Coincidencias: {:?}", probables);
            let probables = self
                .recetas
                .keys()
                .min_by_key(|nombre| levenshtein(nombre, busqueda));
            match probables {
                Some(nombre) => println!("O quisiste decir: {}?", nombre),
                None => println!("No se encontraron coincidencias. "),
            }
        }
    }
}

pub mod repositorio {
    use crate::auxiliares::{AppError, AppResult};
    use crate::negocio::{self, Insumo};
    use std::collections::HashMap;
    use strsim::levenshtein;

    pub trait Bodega {
        fn cargar(&mut self);
        fn añadir(&mut self, nombre: &str, insumo: negocio::Insumo);
        fn eliminar(&mut self, nombre: &str);
        fn buscar(&self, busqueda: &str) -> Vec<&String>;
        fn obtener(&mut self, busqueda: &str) -> AppResult<&negocio::Insumo>;
        fn mostrar_todos(&self) -> Vec<String>; //realmente sera un insumo pero hay que ver como)> ;
    }

    pub struct AlmacenEnMemoria {
        bodega: HashMap<String, negocio::Insumo>,
    }

    impl AlmacenEnMemoria {
        fn nuevo() -> Self {
            AlmacenEnMemoria {
                bodega: HashMap::new(),
            }
        }
    }
    impl Bodega for AlmacenEnMemoria {
        fn cargar(&mut self) {
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
        fn obtener(&mut self, busqueda: &str) -> AppResult<&Insumo> {
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
        fn nuevo() -> Self {
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
pub mod servicios {

    use crate::auxiliares::{AppError, AppResult};
    use crate::negocio;
    use crate::repositorio::Bodega;
    use rusqlite::ffi::sqlite3changeset_concat_strm;
    use strsim::levenshtein;

    pub struct ServicioDeAlmacen {
        repositorio: Box<dyn Bodega>,
    }

    impl ServicioDeAlmacen {
        fn nuevo(
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
        fn buscar(&self, busqueda: &str) -> Vec<String> {
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

        fn existe(&self, busqueda: &String) -> bool {
            let lista = self.repositorio.mostrar_todos();
            if lista.contains(busqueda) {
                return true;
            } else {
                return false;
            }
        }

        fn eliminar(&mut self, insumo: &str) -> AppResult<()> {
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
    }
}
