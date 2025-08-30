// TAREAS: {
//    A: Eliminar recetas  de los insumos que son eliminados.
//    B: Queda pendiente la consulta de insumo_en_recetas dentro de repositorio.
// }

// FN_MAIN:  => {
// RUN: CLI: was the first aproaching on the project. to interact with the backend while his first weeks.
// RUN: || SERVER: since the main program's goal is be a Server WLAN for an front as web page and Desktop app.
//}
use negocio::AppError; //ERRORS: since Rust providess Result we can handle more propialy where and how to manage this errors.
use std::env;
use tokio::sync::Mutex; // Rust doesn't provides async main functions by default.

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let argumentos: Vec<String> = env::args().collect();
    if argumentos.len() > 1 && argumentos[1] == "server" {
        println!("Iniciando el servidor Http...");
        submainfunctions::correr_servidor().await?
    } else {
        println!("Iniciando la linea de comandos.");
        submainfunctions::correr_cli()?;
    }
    Ok(())
}

// Here we can add more ways to operate this programs.
// FIRST: We'll need to build an struct from the mod 'repositorio' (repository) for each Table.
// SECOND: we need to build an service using this repositorory struct.
// THIRD: use a layer to operate the service.
pub mod submainfunctions {
    use actix_web::guard;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // (fn)RUN_SERVER:
    pub async fn correr_servidor() -> Result<(), crate::negocio::AppError> {
        use crate::actix::{
            buscar_insumo_manejador, crear_insumo_manejador, crear_receta_manejador,
            editar_insumo_manejador, eliminar_insumo_manejador, valor_de_insumo_manejador,
            ver_todos_los_insumos_manejador,
        };
        use crate::repositorio;
        use crate::servicio;
        use actix_cors::Cors;
        use actix_web::{App, HttpResponse, HttpServer, Responder, guard, http, web};

        //Cargamos de repositorio (inyeccion de dependencias).:
        let almacen = match repositorio::AlmacenEnMemoria::nuevo("cafeteria") {
            Ok(almacen) => almacen,
            Err(e) => {
                println!(
                    "Error al abrir la base de datos para el almacen\nError: {}",
                    e
                );
                return Err(e);
            }
        };
        let recetario = match repositorio::RecetarioEnMemoria::nuevo("cafeteria") {
            Ok(recetario) => recetario,
            Err(e) => {
                println!(
                    "Error al abrir la base de datos para el recetario\nError: {}",
                    e
                );
                return Err(e);
            }
        };
        let usuario_repo = match repositorio::UsuariosDb::nuevo("cafeteria") {
            Ok(repo) => repo,
            Err(e) => {
                println!(
                    "Error al abrir la base de datos para Usuarios\nError: {}",
                    e
                );
                return Err(e);
            }
        };

        println!("Almacen, Recetarioy  Usuarios cargados correctamente");

        //Envolvemos almacen en Box para que sea aceptado por Servicio.
        // Envolvemos en Mutex para permitir la mutabilidad segura.
        // Envolvemos en Arc para multihilo.
        let servicio_de_almacen = Arc::new(Mutex::new(servicio::ServicioDeAlmacen::nuevo(
            Box::new(almacen),
        )));
        let servicio_de_recetas = Arc::new(Mutex::new(servicio::ServicioDeRecetas::nuevo(
            Box::new(recetario),
        )));
        let servicio_de_usuarios = Arc::new(Mutex::new(servicio::ServicioDeUsuarios::nuevo(
            Box::new(usuario_repo),
        )));

        //Iniciar el server:

        HttpServer::new(move || {
            let almacen_info = servicio_de_almacen.clone();
            let recetas_info = servicio_de_recetas.clone();
            let usuarios_info = servicio_de_usuarios.clone();

            let cors = Cors::default()
                .allowed_origin_fn(|_origin, _req_head| true)
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
                .supports_credentials()
                .max_age(3600);

            App::new()
                .wrap(cors)
                .app_data(web::Data::new(almacen_info))
                .app_data(web::Data::new(recetas_info))
                .app_data(web::Data::new(usuarios_info))
                .service(
                    web::scope("")
                        .service(
                            web::resource("/insumos/buscar")
                                .route(web::get().to(buscar_insumo_manejador)),
                        )
                        .service(
                            web::resource("/insumos/todos")
                                .route(web::get().to(ver_todos_los_insumos_manejador)),
                        )
                        .service(
                            web::resource("/insumos/valor")
                                .route(web::get().to(valor_de_insumo_manejador)),
                        )
                        .service(
                            web::resource("/recetas/todos")
                                .route(web::get().to(crate::actix::listar_recetas_manejador)),
                        )
                        .service(
                            web::resource("/recetas/buscar")
                                .route(web::get().to(crate::actix::buscar_receta_manejador)),
                        )
                        .service(
                            web::resource("/recetas/valor")
                                .route(web::get().to(crate::actix::valor_receta_manejador)),
                        )
                        .service(
                            web::resource("/usuarios/todos")
                                .route(web::get().to(crate::actix::listar_usuarios_manejador)),
                        )
                        .service(
                            web::resource("/usuarios/buscar")
                                .route(web::get().to(crate::actix::buscar_usuario_manejador)),
                        )
                        .service(
                            web::resource("/usuarios/valor")
                                .route(web::get().to(crate::actix::valor_de_usuario_manejador)),
                        )
                        .service(
                            web::resource("/usuarios/iniciar_sesion")
                                .route(web::post().to(crate::actix::iniciar_sesion_manejador)),
                        ),
                )
                .service(
                    web::scope("")
                        .wrap(crate::actix::middleware::GuardianDeAcceso) //ACTIVAMOS MIDDLEWARE
                        .service(
                            web::resource("/insumos/editar/{nombre}")
                                .route(web::put().to(editar_insumo_manejador))
                                .route(
                                    web::route()
                                        .guard(guard::Method(http::Method::OPTIONS))
                                        .to(|| async { HttpResponse::Ok().finish() }),
                                ),
                        )
                        .service(
                            web::resource("/insumos/{insumo}")
                                .route(web::delete().to(eliminar_insumo_manejador)),
                        )
                        .service(
                            web::resource("/insumos/crear")
                                .route(web::post().to(crear_insumo_manejador)),
                        )
                        .service(
                            web::resource("/recetas/crear")
                                .route(web::post().to(crear_receta_manejador)),
                        )
                        .service(
                            web::resource("/recetas/editar/{nombre}")
                                .route(web::put().to(crate::actix::editar_receta_manejador))
                                .route(
                                    web::route()
                                        .guard(guard::Method(http::Method::OPTIONS))
                                        .to(|| async { HttpResponse::Ok().finish() }),
                                ),
                        )
                        .service(
                            web::resource("/recetas/{receta}")
                                .route(web::delete().to(crate::actix::eliminar_receta_manejador)),
                        )
                        .service(
                            web::resource("/usuarios/crear")
                                .route(web::post().to(crate::actix::crear_usuario_manejador)),
                        )
                        .service(
                            web::resource("/usuarios/{usuario}")
                                .route(web::delete().to(crate::actix::eliminar_usuario_manejador)),
                        ),
                )
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;
        Ok(())
    }

    pub fn correr_cli() -> Result<(), crate::negocio::AppError> {
        use crate::repositorio;
        use crate::servicio;

        let almacen = match repositorio::AlmacenEnMemoria::nuevo("cafeteria") {
            Ok(almacen) => almacen,
            Err(e) => {
                println!("Error al abrir la base de datos porque: {}", e);
                return Err(e);
            }
        };

        let recetario = match repositorio::RecetarioEnMemoria::nuevo("cafeteria") {
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
                if super::cli::reintentar() {
                    continue;
                }
                break;
            }
        }

        loop {
            let res = super::cli::menu();
            match res {
                1 => break,
                2 => reintentar_o_salir(|| super::cli::crear_insumo(&mut servicio_de_almacen)),
                3 => reintentar_o_salir(|| {
                    super::cli::crear_receta(&mut servicio_de_recetas, &servicio_de_almacen)
                }),
                4 => reintentar_o_salir(|| super::cli::buscar_insumo(&servicio_de_almacen)),
                5 => reintentar_o_salir(|| super::cli::buscar_receta(&servicio_de_recetas)),
                6 => super::cli::ver_insumos(&servicio_de_almacen),
                7 => super::cli::ver_recetas(&servicio_de_recetas),
                8 => reintentar_o_salir(|| super::cli::valor_de_insumo(&servicio_de_almacen)),
                9 => reintentar_o_salir(|| {
                    super::cli::receta_valor(&servicio_de_recetas, &servicio_de_almacen)
                }),
                10 => reintentar_o_salir(|| super::cli::eliminar_insumo(&mut servicio_de_almacen)),
                11 => reintentar_o_salir(|| super::cli::eliminar_receta(&mut servicio_de_recetas)),
                12 => reintentar_o_salir(|| {
                    super::cli::producir_receta(&mut servicio_de_almacen, &servicio_de_recetas)
                }),
                13 => reintentar_o_salir(|| {
                    super::cli::ingredientes_en_recetas(&servicio_de_recetas, &servicio_de_almacen)
                }),
                14 => reintentar_o_salir(|| super::cli::editar_insumo(&mut servicio_de_almacen)),
                15 => reintentar_o_salir(|| {
                    super::cli::editar_receta(&mut servicio_de_recetas, &servicio_de_almacen)
                }),
                _ => continue,
            }
        }
        return Ok(());
    }
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
                 \n2) Crear Un Insumo.                3) Crear unafn obten
                 \n4) Buscar un insumo.               5) Buscar una receta.
                 \n6) Ver todos los insumos.          7) Ver todas las recetas.
                 \n8) Ver el valor de un Insumo.      9) Ver el valor de una Receta.
                 \n10) Eliminar Insumo.              11) Eliminar Receta.
                 \n12) Producir Receta.              13) Ingredientes en recetas.
                 \n14) Editar Insumo.                15) Editar Receta."
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
                    "id: {}, \nNombre: {}, \nCantidad: {},\nCantidad minima: {}, \nPrecio por kilo: ${}",
                    ins.0, ins.1, ins.2, ins.3, ins.4
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

pub mod actix {
    use crate::comandos;
    use crate::servicio::{ServicioDeAlmacen, ServicioDeRecetas, ServicioDeUsuarios};
    use actix_web::{HttpResponse, Responder, web};
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub mod middleware {

        use crate::negocio::{Acciones, Entidad};
        use actix_service::{Service, Transform};
        use actix_web::{
            Error, HttpResponse,
            body::EitherBody,
            dev::{ServiceRequest, ServiceResponse},
        };
        use futures::future::{LocalBoxFuture, Ready, ok};
        use std::{
            pin::Pin,
            rc::Rc,
            task::{Context, Poll},
        };

        pub struct GuardianDeAcceso;

        impl<S, B> Transform<S, ServiceRequest> for GuardianDeAcceso
        where
            S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
            B: 'static,
        {
            type Response = ServiceResponse<EitherBody<B>>;
            type Error = Error;
            type InitError = ();
            type Transform = GuardianMiddleware<S>;
            type Future = Ready<Result<Self::Transform, Self::InitError>>;

            fn new_transform(&self, service: S) -> Self::Future {
                ok(GuardianMiddleware {
                    service: Rc::new(service),
                })
            }
        }

        pub struct GuardianMiddleware<S> {
            service: Rc<S>,
        }

        impl<S, B> Service<ServiceRequest> for GuardianMiddleware<S>
        where
            S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
            B: 'static,
        {
            type Response = ServiceResponse<EitherBody<B>>;
            type Error = Error;
            type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

            fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
                self.service.poll_ready(cx)
            }

            fn call(&self, req: ServiceRequest) -> Self::Future {
                let svc = Rc::clone(&self.service);
                let path = req.path().to_string();
                let token = req
                    .headers()
                    .get("Authorization")
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or_default()
                    .to_string();

                Box::pin(async move {
                    match crate::actix::middleware::verificar_permiso(&path, &token) {
                        Ok(_) => {
                            // Respuesta exitosa mapeada al Left de EitherBody
                            let res: ServiceResponse<B> = svc.call(req).await?;
                            Ok(res.map_into_left_body())
                        }
                        Err(e) => {
                            // Error convertido al Right de EitherBody
                            let res = req.into_response(HttpResponse::Forbidden().body(e));
                            Ok(res.map_into_right_body())
                        }
                    }
                })
            }
        }

        fn parsear_ruta(ruta: &str) -> Result<(Entidad, Acciones), String> {
            let partes: Vec<&str> = ruta.trim_start_matches('/').split('/').collect();

            if partes.len() != 2 {
                return Err("Ruta inválida: se esperaba /entidad/accion".to_string());
            }

            let entidad = match partes[0] {
                "receta" => Entidad::Receta,
                "insumo" => Entidad::Insumo,
                "usuario" => Entidad::Usuario,
                _ => return Err(format!("Entidad desconocida: {}", partes[0])),
            };

            let accion = match partes[1] {
                "crear" => Acciones::Crear,
                "editar" => Acciones::Editar,
                "eliminar" => Acciones::Eliminar,
                _ => return Err(format!("Acción desconocida: {}", partes[1])),
            };

            Ok((entidad, accion))
        }

        pub fn verificar_permiso(ruta: &str, token: &str) -> Result<(), String> {
            let (entidad, accion) = parsear_ruta(ruta)?;
            let rol = match crate::negocio::verificar_token(token) {
                Ok(r) => r,
                Err(e) => return Err(format!("Acceso denegado: {}", e)),
            };

            if crate::negocio::puede_operar(entidad, rol, accion) {
                Ok(())
            } else {
                Err(format!(
                    "El rol {:?} no tiene permiso para realizar {:?} sobre {:?}",
                    rol, accion, entidad
                ))
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct CrearUsuarioPeticion {
        pub nombre: String,
        pub rol: String,
        pub contra: String,
    }

    #[derive(Deserialize, Serialize)]
    pub struct VerificarUsuario {
        nombre: String,
        contra: String,
    }

    pub async fn crear_usuario_manejador(
        app_info_repositorio: web::Data<std::sync::Arc<tokio::sync::Mutex<ServicioDeUsuarios>>>,
        peticion: web::Json<CrearUsuarioPeticion>,
    ) -> impl Responder {
        println!(
            "nombre:  {}, rol: {}, admin: {}",
            peticion.nombre, peticion.rol, peticion.contra
        );
        let mut repositorio = app_info_repositorio.lock().await;
        match comandos::crear_usuario(
            (&peticion.nombre, &peticion.contra, &peticion.rol),
            &mut repositorio,
        ) {
            Ok(_) => HttpResponse::Ok().json(MensajeRespuesta {
                mensaje: format!("usuario: {}, creado exitosamente", peticion.nombre),
            }),
            Err(e) => {
                println!("{}: 652", e);
                return HttpResponse::InternalServerError().json(MensajeRespuesta {
                    mensaje: format!("Error al crear usuario: {}", e),
                });
            }
        }
    }

    pub async fn iniciar_sesion_manejador(
        app_info_repositorio: web::Data<std::sync::Arc<tokio::sync::Mutex<ServicioDeUsuarios>>>,
        query: web::Json<VerificarUsuario>,
    ) -> impl Responder {
        let mut repo = app_info_repositorio.lock().await;
        match comandos::iniciar_sesion(&mut repo, &query.nombre, &query.contra) {
            Ok(token) => HttpResponse::Ok().json(token),
            Err(e) => HttpResponse::BadRequest().json(MensajeRespuesta {
                mensaje: format!("Error de validacion, {}", e),
            }),
        }
    }

    pub async fn buscar_usuario_manejador(
        app_info_repositorio: web::Data<std::sync::Arc<tokio::sync::Mutex<ServicioDeUsuarios>>>,
        query: web::Query<ParametrosConsulta>,
    ) -> impl Responder {
        let nombre_usuario = match &query.consulta {
            Some(nombre) => nombre.clone(),
            None => {
                return HttpResponse::BadRequest().json(MensajeRespuesta {
                    mensaje: "Falta el parametro de busqueda".to_string(),
                });
            }
        };
        let repositorio = app_info_repositorio.lock().await;
        return match comandos::buscar_usuario(&repositorio, &nombre_usuario) {
            Ok(usuarios) => HttpResponse::Ok().json(usuarios),
            Err(e) => HttpResponse::BadRequest().json(MensajeRespuesta {
                mensaje: format!("Error al listar los usarios: {}", e),
            }),
        };
    }

    pub async fn listar_usuarios_manejador(
        app_info_repositorio: web::Data<Arc<Mutex<ServicioDeUsuarios>>>,
    ) -> impl Responder {
        let repo = app_info_repositorio.lock().await;
        return match comandos::listar_usuarios(&repo) {
            Ok(resultados) => HttpResponse::Ok().json(resultados),
            Err(e) => HttpResponse::BadRequest().json(MensajeRespuesta {
                mensaje: format!("Error al listar los usuarios: {}", e),
            }),
        };
    }

    pub async fn valor_de_usuario_manejador(
        app_info_repositorio: web::Data<Arc<Mutex<ServicioDeUsuarios>>>,
        query: web::Query<ParametrosConsulta>,
    ) -> impl Responder {
        let nombre = match &query.consulta {
            Some(nombre) => nombre.clone(),
            None => {
                return HttpResponse::BadRequest().json(MensajeRespuesta {
                    mensaje: "Falta el parametro 'consulta' ".to_string(),
                });
            }
        };
        let repo = app_info_repositorio.lock().await;
        match comandos::valor_de_usuario(&repo, &nombre) {
            Ok((id, nombre, rol)) => HttpResponse::Ok().json(serde_json::json!({
                "id": id,
                "nombre": nombre,
                "rol": rol,
            })),
            Err(e) => HttpResponse::NotFound().json(MensajeRespuesta {
                mensaje: format!("No se encontro el usuario '{}', \nError: '{}'", nombre, e),
            }),
        }
    }

    pub async fn eliminar_usuario_manejador(
        app_info_repositorio: web::Data<Arc<Mutex<ServicioDeUsuarios>>>,
        ruta: web::Path<String>,
    ) -> impl Responder {
        let nombre_insumo = ruta.into_inner();
        let mut repo = app_info_repositorio.lock().await;

        match comandos::eliminar_usuario(&mut repo, &nombre_insumo) {
            Ok(_) => HttpResponse::Ok().json(MensajeRespuesta {
                mensaje: format!("Insumo '{}' eliminado correctamente.", nombre_insumo),
            }),
            Err(e) => HttpResponse::InternalServerError().json(MensajeRespuesta {
                mensaje: format!("Error al eliminar el insumo '{}': {}", nombre_insumo, e),
            }),
        }
    }

    #[derive(Deserialize)]
    pub struct DatosReceta {
        nombre: String,
        ingredientes: Vec<Ingrediente>,
    }

    #[derive(Deserialize)]
    pub struct Ingrediente {
        nombre: String,
        cantidad: u32,
    }

    #[derive(Deserialize)]
    pub struct ParametrosConsulta {
        consulta: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CrearInsumoPeticion {
        pub nombre: String,
        pub cantidad: u32,
        pub cantidad_minima: u32,
        pub precio: u32,
    }

    #[derive(Deserialize, Serialize)]
    pub struct MensajeRespuesta {
        pub mensaje: String,
    }
    pub fn extraer_nombre_insumo(
        ruta: Option<web::Path<String>>,
        query: Option<web::Query<ParametrosConsulta>>,
    ) -> Option<String> {
        if let Some(consulta) = query {
            consulta.consulta.clone()
        } else if let Some(ruta_valor) = ruta {
            Some(ruta_valor.into_inner())
        } else {
            None
        }
    }

    pub async fn crear_insumo_manejador(
        app_info_almacen: web::Data<std::sync::Arc<tokio::sync::Mutex<ServicioDeAlmacen>>>,
        peticion: web::Json<CrearInsumoPeticion>,
    ) -> impl Responder {
        let mut almacen = app_info_almacen.lock().await;
        match almacen.añadir(
            peticion.nombre.clone(),
            peticion.cantidad,
            peticion.cantidad_minima,
            peticion.precio,
        ) {
            Ok(_) => HttpResponse::Ok().json(MensajeRespuesta {
                mensaje: format!("Insumo: {}, creado exitosamente", peticion.nombre),
            }),
            Err(e) => HttpResponse::InternalServerError().json(MensajeRespuesta {
                mensaje: format!("Error al crear insumo: {}", e),
            }),
        }
    }

    pub async fn buscar_insumo_manejador(
        app_info_almacen: web::Data<std::sync::Arc<tokio::sync::Mutex<ServicioDeAlmacen>>>,
        query: web::Query<ParametrosConsulta>,
    ) -> impl Responder {
        let nombre_insumo = match &query.consulta {
            Some(nombre) => nombre.clone(),
            None => {
                return HttpResponse::BadRequest().json(MensajeRespuesta {
                    mensaje: "Falta el parametro de busqueda".to_string(),
                });
            }
        };
        let almacen = app_info_almacen.lock().await;
        match almacen.buscar(&nombre_insumo) {
            Ok(resultados) => {
                if resultados.is_empty() {
                    HttpResponse::NotFound().json(MensajeRespuesta {
                        mensaje: format!(
                            "El insumo de nombre: {}\n No fue encontrado",
                            nombre_insumo
                        ),
                    })
                } else {
                    HttpResponse::Ok().json(resultados)
                }
            }
            Err(e) => HttpResponse::InternalServerError().json(MensajeRespuesta {
                mensaje: format!("Error al buscar el insumo: {}\nError: {}", nombre_insumo, e),
            }),
        }
    }

    pub async fn ver_todos_los_insumos_manejador(
        app_info_almacen: web::Data<Arc<Mutex<ServicioDeAlmacen>>>,
    ) -> impl Responder {
        println!("Espero hayas tenido un bonito dia");
        let almacen = app_info_almacen.lock().await;
        let resultados = comandos::ver_todos_los_insumos(&almacen);
        HttpResponse::Ok().json(resultados)
    }

    pub async fn valor_de_insumo_manejador(
        app_info_almacen: web::Data<Arc<Mutex<ServicioDeAlmacen>>>,
        query: web::Query<ParametrosConsulta>,
    ) -> impl Responder {
        let nombre = match &query.consulta {
            Some(nombre) => nombre.clone(),
            None => {
                return HttpResponse::BadRequest().json(MensajeRespuesta {
                    mensaje: "Falta el parametro 'consulta' ".to_string(),
                });
            }
        };
        let almacen = app_info_almacen.lock().await;
        match comandos::valor_de_insumo(&nombre, &almacen) {
            Ok((id, nombre, cantidad, cantidad_minima, costo)) => {
                HttpResponse::Ok().json(serde_json::json!({
                    "id": id,
                    "nombre": nombre,
                    "cantidad": cantidad,
                    "cantidad_minima": cantidad_minima,
                    "precio": costo,
                }))
            }
            Err(e) => HttpResponse::NotFound().json(MensajeRespuesta {
                mensaje: format!("No se encontro el insumo '{}', \nError: '{}'", nombre, e),
            }),
        }
    }

    #[derive(Deserialize)]
    pub struct EditarInsumoPayload {
        nombre: Option<String>,
        cantidad: Option<u32>,
        cantidad_minima: Option<u32>,
        costo: Option<u32>,
    }

    pub async fn editar_insumo_manejador(
        app_info_almacen: web::Data<Arc<Mutex<ServicioDeAlmacen>>>,
        path: web::Path<String>,
        datos: web::Json<EditarInsumoPayload>,
    ) -> impl Responder {
        println!("y que GG te siga despertando como solo ella sabe");
        let nombre_actual = path.into_inner();
        let body = datos.into_inner();

        let mut almacen = app_info_almacen.lock().await;
        match comandos::editar_insumo(
            &mut almacen,
            &nombre_actual,
            body.nombre,
            body.cantidad,
            body.cantidad_minima,
            body.costo,
        ) {
            Ok(_) => HttpResponse::Ok().json(MensajeRespuesta {
                mensaje: format!("Insumo '{}' actualizado correctamente.", nombre_actual),
            }),
            Err(e) => HttpResponse::InternalServerError().json(MensajeRespuesta {
                mensaje: format!("Error al actualizar el insumo '{}': {}", nombre_actual, e),
            }),
        }
    }

    pub async fn eliminar_insumo_manejador(
        app_info_almacen: web::Data<Arc<Mutex<ServicioDeAlmacen>>>,
        ruta: web::Path<String>,
    ) -> impl Responder {
        let nombre_insumo = ruta.into_inner();
        let mut almacen = app_info_almacen.lock().await;

        match comandos::eliminar_insumo(&mut almacen, &nombre_insumo) {
            Ok(_) => HttpResponse::Ok().json(MensajeRespuesta {
                mensaje: format!("Insumo '{}' eliminado correctamente.", nombre_insumo),
            }),
            Err(e) => HttpResponse::InternalServerError().json(MensajeRespuesta {
                mensaje: format!("Error al eliminar el insumo '{}': {}", nombre_insumo, e),
            }),
        }
    }

    //    RECETAS:

    pub async fn crear_receta_manejador(
        datos: web::Json<DatosReceta>,
        datos_almacen: web::Data<Arc<Mutex<ServicioDeAlmacen>>>,
        datos_libro: web::Data<Arc<Mutex<ServicioDeRecetas>>>,
    ) -> impl Responder {
        let entrada = datos.into_inner();

        let almacen = datos_almacen.lock().await;
        let mut libro = datos_libro.lock().await;

        let receta_como_tupla = (
            entrada.nombre.clone(),
            entrada
                .ingredientes
                .into_iter()
                .map(|i| (i.nombre, i.cantidad))
                .collect(),
        );

        match comandos::crear_receta(receta_como_tupla, &almacen, &mut libro) {
            Ok(mensaje) => HttpResponse::Ok().json(MensajeRespuesta { mensaje }),
            Err(e) => HttpResponse::BadRequest().json(MensajeRespuesta {
                mensaje: format!("No se pudo crear la receta: {}", e),
            }),
        }
    }

    pub async fn listar_recetas_manejador(
        app_info_libro: web::Data<Arc<Mutex<ServicioDeRecetas>>>,
    ) -> impl Responder {
        let libro = app_info_libro.lock().await;
        let resultados = comandos::ver_todos_las_recetas(&libro);
        resultados.clone().into_iter().map(|e| print!("{}", e));
        HttpResponse::Ok().json(resultados)
    }

    pub async fn buscar_receta_manejador(
        app_info_libro: web::Data<Arc<Mutex<ServicioDeRecetas>>>,
        query: web::Query<ParametrosConsulta>,
    ) -> impl Responder {
        let nombre = match &query.consulta {
            Some(nom) => nom.clone(),
            None => {
                return HttpResponse::BadRequest().json(MensajeRespuesta {
                    mensaje: "Falta el parametro de busqueda".to_string(),
                });
            }
        };
        let libro = app_info_libro.lock().await;
        let resultados = comandos::buscar_receta(&libro, &nombre);
        HttpResponse::Ok().json(resultados)
    }

    pub async fn valor_receta_manejador(
        app_info_libro: web::Data<Arc<Mutex<ServicioDeRecetas>>>,
        query: web::Query<ParametrosConsulta>,
        app_info_almacen: web::Data<Arc<Mutex<ServicioDeAlmacen>>>,
    ) -> impl Responder {
        let nombre = match &query.consulta {
            Some(nombre) => nombre.clone(),
            None => {
                return HttpResponse::BadRequest().json(MensajeRespuesta {
                    mensaje: "Falta el parametro 'consulta' ".to_string(),
                });
            }
        };
        let libro = app_info_libro.lock().await;
        let almacen = app_info_almacen.lock().await;
        match comandos::receta_valor(&nombre, &libro, &almacen) {
            Ok((nombre, ingredientes, costo)) => {
                let receta = Receta {
                    nombre,
                    ingredientes,
                    costo,
                };
                HttpResponse::Ok().json(receta)
            }
            Err(e) => HttpResponse::BadRequest().json(MensajeRespuesta {
                mensaje: format!("no se encontro la receta: {}\n Error: {}", nombre, e),
            }),
        }
    }

    pub async fn editar_receta_manejador(
        app_info_almacen: web::Data<Arc<Mutex<ServicioDeAlmacen>>>,
        path: web::Path<String>,
        datos: web::Json<EditarRecetaPayLoad>,
        app_info_libro: web::Data<Arc<Mutex<ServicioDeRecetas>>>,
    ) -> impl Responder {
        let nombre = path.into_inner();
        let body = datos.into_inner();

        //Aqui puse el dereferenciador '*' porque me decia que se estaban recibiendo GaurMutex.
        //
        let almacen = app_info_almacen.lock().await;
        let mut libro = app_info_libro.lock().await;
        let servicio: &mut ServicioDeRecetas = &mut *libro;
        match comandos::editar_receta(servicio, &nombre, body.nombre, body.ingredientes, &almacen) {
            Ok(_) => HttpResponse::Ok().json(MensajeRespuesta {
                mensaje: format!("Receta: {}, Actualizada correctamente", nombre),
            }),
            Err(e) => HttpResponse::InternalServerError().json(MensajeRespuesta {
                mensaje: format!("Error al actualizar la receta: {}.\nError: {}", nombre, e),
            }),
        }
    }

    pub async fn eliminar_receta_manejador(
        app_info_libro: web::Data<Arc<Mutex<ServicioDeRecetas>>>,
        receta: web::Path<String>,
    ) -> impl Responder {
        let mut libro = app_info_libro.lock().await;
        let servicio: &mut ServicioDeRecetas = &mut *libro;
        let nombre = receta.into_inner();

        match comandos::eliminar_receta(servicio, &nombre) {
            Ok(_) => HttpResponse::Ok().json(MensajeRespuesta {
                mensaje: format!("Receta: {}, Eliminada correctamente.", nombre),
            }),
            Err(e) => HttpResponse::InternalServerError().json(MensajeRespuesta {
                mensaje: format!("Receta: {}, no eliminada \nError: {}", nombre, e),
            }),
        }
    }

    #[derive(Deserialize)]
    pub struct EditarRecetaPayLoad {
        nombre: Option<String>,
        ingredientes: Option<Vec<(String, u32)>>,
    }

    #[derive(Serialize)]
    pub struct Receta {
        nombre: String,
        ingredientes: Vec<(String, u32)>,
        costo: f64,
    }
}

pub mod comandos {
    use crate::negocio::{AppError, AppResult};
    use crate::servicio::{ServicioDeAlmacen, ServicioDeRecetas, ServicioDeUsuarios};

    pub fn crear_usuario(
        usuario: (&str, &str, &str),
        repositorio: &mut ServicioDeUsuarios,
    ) -> AppResult<String> {
        return match repositorio.agregar(
            usuario.0.to_string(),
            usuario.1.to_string(),
            usuario.2.to_string(),
        ) {
            Ok(_) => Ok(format!("Se ha creado el usuario: {}", usuario.0)),
            Err(e) => {
                println!("{}1074", e);
                return Err(AppError::ErrorPersonal(format!(
                    "Error al crear el usuario: {}\n Error {}",
                    usuario.0, e
                )));
            }
        };
    }

    pub fn iniciar_sesion(
        repositorio: &mut ServicioDeUsuarios,
        usuario: &str,
        contra: &str,
    ) -> AppResult<String> {
        return match repositorio.verificar_usuario(contra, usuario) {
            Ok(token) => Ok(token),
            Err(_) => Err(AppError::DatoInvalido(format!(
                "Querid@: {}, contra incorrecta, por favor vuelve a intentar.",
                usuario
            ))),
        };
    }

    pub fn buscar_usuario(
        repositorio: &ServicioDeUsuarios,
        busqueda: &String,
    ) -> AppResult<Vec<String>> {
        return match repositorio.buscar(busqueda) {
            Ok(resultados) => Ok(resultados),
            Err(e) => Err(AppError::DatoInvalido(format!(
                "No se encontro el usuario: {}\nError: {}",
                busqueda, e
            ))),
        };
    }

    pub fn listar_usuarios(repositorio: &ServicioDeUsuarios) -> AppResult<Vec<String>> {
        return match repositorio.listar() {
            Ok(resultados) => Ok(resultados),
            Err(e) => Err(AppError::ErrorPersonal(format!(
                "Ocurrio un error al listar los usuarios: {}",
                e
            ))),
        };
    }

    pub fn valor_de_usuario(
        repo: &ServicioDeUsuarios,
        usuario: &str,
    ) -> AppResult<(String, String, String)> {
        return match repo.obtener(usuario) {
            Ok((id, nombre, rol)) => Ok((id, nombre, rol)),
            Err(e) => Err(AppError::ErrorPersonal(format!(
                "Error al encontrar el usuario: {}, error: {}",
                usuario, e
            ))),
        };
    }

    pub fn eliminar_usuario(
        repositorio: &mut ServicioDeUsuarios,
        busqueda: &String,
    ) -> AppResult<()> {
        repositorio.eliminar(busqueda)?;
        Ok(())
    }

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
    ) -> AppResult<(String, String, u32, u32, u32)> {
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
        almacen.obtener_id_con_nombre(insumo)?;
        return libro.insumo_en_recetas(insumo);
    }
}

pub mod auxiliares {
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
    use bcrypt::hash;

    //Esta capa del programa se encargara de la virtualizacion de entidades en memoria y
    //su gestion bajo las reglas logicas del negocio.
    //
    use chrono::DateTime;
    use serde::{Deserialize, Serialize};
    //Esto de acá es para la fecha.
    use actix_web;
    use rusqlite::Error as SqlError;
    use std::io;
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
        #[error("Error de I/O: {0} ")]
        IoError(#[from] io::Error),
        #[error("Error de Actix_web: {0}")]
        ActixError(#[from] actix_web::Error),
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

            let receta = Receta {
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

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Proveedor {
        id: String,
        marca: String,
        numero_contacto: String,
        producto: String,
    }

    impl Proveedor {
        pub fn nuevo(
            marca: String,
            numero_contacto: String,
            producto: String,
        ) -> AppResult<Proveedor> {
            if marca.is_empty() {
                return Err(AppError::DatoInvalido(
                    "El nombre de la marca esta vacio.".to_string(),
                ));
            }
            if numero_contacto.is_empty() {
                return Err(AppError::DatoInvalido(
                    "El nombre de la marca esta vacio.".to_string(),
                ));
            }
            if producto.is_empty() {
                return Err(AppError::DatoInvalido(
                    "El nombre de la marca esta vacio.".to_string(),
                ));
            }
            Ok(Proveedor {
                id: Uuid::new_v4().to_string(),
                marca,
                numero_contacto,
                producto,
            })
        }

        pub fn crear_desde_db(
            id: String,
            marca: String,
            numero_contacto: String,
            producto: String,
        ) -> Proveedor {
            Proveedor {
                id,
                marca,
                numero_contacto,
                producto,
            }
        }

        pub fn obtener_id(&self) -> String {
            self.id.clone()
        }

        pub fn obtener_marca(&self) -> String {
            self.marca.clone()
        }

        pub fn obtener_numero(&self) -> String {
            self.numero_contacto.clone()
        }

        pub fn obtener_producto(&self) -> String {
            self.producto.clone()
        }
    }

    // GASTOS: Quiza despues podamos pensar en agregar Servicios como Gas, Agua o Luz.

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Gasto {
        id: String,
        insumo_id: String,
        proveedor_id: String,
        gasto_pesos: f64,
    }

    impl Gasto {
        pub fn nuevo(
            insumo_id: String,
            proveedor_id: String,
            gasto_pesos: f64,
        ) -> AppResult<Gasto> {
            //comparo a 1.1 para evitar "trampas de redondeo" aunque no se si tenga sentido
            if gasto_pesos <= 1.1 {
                return Err(AppError::DatoInvalido(format!(
                    "El costo del producto no puede ser 0 o menor a 1.1"
                )));
            }
            Ok(Gasto {
                id: Uuid::new_v4().to_string(),
                insumo_id,
                proveedor_id,
                gasto_pesos,
            })
        }

        pub fn crear_desde_db(
            id: String,
            insumo_id: String,
            proveedor_id: String,
            gasto_pesos: f64,
        ) -> Gasto {
            Gasto {
                id,
                insumo_id,
                proveedor_id,
                gasto_pesos,
            }
        }

        pub fn id(&self) -> String {
            self.id.clone()
        }

        pub fn insumo_id(&self) -> String {
            self.insumo_id.clone()
        }

        pub fn proveedor_id(&self) -> String {
            self.proveedor_id.clone()
        }

        pub fn gasto_pesos(&self) -> f64 {
            self.gasto_pesos.clone()
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

    #[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
    pub enum Entidad {
        Insumo,
        Receta,
        Usuario,
    }

    #[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
    pub enum Acciones {
        Crear,
        Eliminar,
        Editar,
    }

    #[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
    pub enum Rol {
        Admin,
        Invitado,
        Usuario,
    }

    fn listar_permisos() -> HashMap<Entidad, HashMap<Rol, Vec<Acciones>>> {
        let mut permisos = HashMap::new();
        let mut receta_permisos = HashMap::new();
        receta_permisos.insert(
            Rol::Admin,
            vec![Acciones::Crear, Acciones::Editar, Acciones::Eliminar],
        );
        receta_permisos.insert(Rol::Usuario, vec![Acciones::Crear, Acciones::Editar]);
        permisos.insert(Entidad::Receta, receta_permisos);
        let mut permisos_insumos = HashMap::new();
        permisos_insumos.insert(
            Rol::Admin,
            vec![Acciones::Crear, Acciones::Editar, Acciones::Eliminar],
        );
        permisos_insumos.insert(Rol::Usuario, vec![Acciones::Crear, Acciones::Editar]);
        permisos.insert(Entidad::Insumo, permisos_insumos);
        let mut permisos_usuarios = HashMap::new();
        permisos_usuarios.insert(
            Rol::Admin,
            vec![Acciones::Crear, Acciones::Editar, Acciones::Eliminar],
        );
        permisos
    }

    pub fn puede_operar(entidad: Entidad, rol: Rol, accion: Acciones) -> bool {
        PERMISOS
            .get(&entidad)
            .and_then(|roles| roles.get(&rol))
            .map_or(false, |acciones| acciones.contains(&accion))
    }

    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    pub static PERMISOS: Lazy<HashMap<Entidad, HashMap<Rol, Vec<Acciones>>>> =
        Lazy::new(|| listar_permisos());

    pub struct Usuario {
        id: String,
        nombre: String,
        contra_token: String,
        rol: String,
    }

    impl Usuario {
        pub fn nuevo(nombre: &String, rol: String, psswd: &str) -> AppResult<Usuario> {
            if nombre.is_empty() {
                return Err(AppError::DatoInvalido(
                    "el nombre no puede estar vacio".to_string(),
                ));
            }
            if rol.is_empty() {
                return Err(AppError::DatoInvalido("El rol esta vacio".to_string()));
            }

            let contraseña = hash(psswd, 12).expect("Error al encriptar la contraseña");
            Ok(Usuario {
                id: Uuid::new_v4().to_string(),
                nombre: nombre.clone(),
                contra_token: contraseña,
                rol,
            })
        }

        pub fn crear_desde_db(
            id: String,
            nombre: String,
            contra_token: String,
            rol: String,
        ) -> Usuario {
            Usuario {
                id,
                nombre,
                contra_token,
                rol,
            }
        }
        pub fn obtener_id(&self) -> String {
            self.id.clone()
        }

        pub fn obtener_nombre(&self) -> String {
            self.nombre.clone()
        }

        pub fn obtener_hash(&self) -> String {
            self.contra_token.clone()
        }

        pub fn obtener_rol(&self) -> String {
            self.rol.clone()
        }

        pub fn verificar_hash(&self, contra: &str) -> AppResult<String> {
            match bcrypt::verify(contra, &self.contra_token) {
                Ok(true) => Ok(self.generar_token()),
                Ok(false) => Err(AppError::DatoInvalido("Contraseña incorrecta.".to_string())),
                Err(e) => Err(AppError::DatoInvalido(format!(
                    "Error al verificar hash: {}",
                    e
                ))),
            }
        }

        pub fn generar_token(&self) -> String {
            let nombre = self.nombre.chars().take(2);
            let rol = self.rol.chars().take(2);
            let id = self.id.chars().take(2);
            let hash = self.contra_token.chars().take(2);

            let mut token = String::new();

            for c in nombre.chain(rol).chain(id).chain(hash) {
                token.push(c);
            }

            token
        }
    }

    pub fn verificar_token(token: &str) -> Result<Rol, String> {
        if token.len() != 8 {
            return Err("Token inválido: longitud incorrecta".to_string());
        }

        let rol_str = &token[2..4];

        let rol = match rol_str {
            "ad" => Rol::Admin,
            "us" => Rol::Usuario,
            "in" => Rol::Invitado,
            _ => return Err("Rol desconocido en el token".to_string()),
        };
        Ok(rol)
    }
}
pub mod repositorio {

    //REPOSITORIO: Aqui se desglosa la logica para la persistencia de datos.
    // Usamos Traits para abstraer la implimentacion de estas funciones y sea una parte modular.

    use crate::{
        negocio::{self, AppError, AppResult, Insumo, Proveedor, Receta},
        servicio::*,
    };
    use rusqlite::{Connection, Error, params};
    use std::sync::{Arc, Mutex};

    pub struct RecetarioEnMemoria {
        conexion: Arc<Mutex<Connection>>,
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
            Ok(RecetarioEnMemoria {
                conexion: Arc::new(Mutex::new(conexion)),
            })
        }
    }

    impl RecetasEnMemoria for RecetarioEnMemoria {
        fn editar_receta(&mut self, receta: negocio::Receta) -> AppResult<()> {
            let mut conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear el mutex de la conexion. {}", e))
            })?;

            let transaccion = conexion_segura.transaction()?;
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
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion. {}", e))
            })?;
            let nombre: String = conexion_segura
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
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion. {}", e))
            })?;
            let id: String = conexion_segura
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
            let mut conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            let transaccion = conexion_segura.transaction()?;

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
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            let bandera =
                conexion_segura.execute("DELETE FROM recetas WHERE id =?", params![id])?;
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
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            let mut accion = conexion_segura.prepare(
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
            conexion_segura
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
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion.: {}", e))
            })?;
            let mut accion =
                conexion_segura.prepare("SELECT nombre FROM recetas ORDER BY nombre")?;
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
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion. {}", e))
            })?;
            let mut res: Vec<String> = Vec::new();
            let mut accion = conexion_segura
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

    pub trait RecetasEnMemoria: Send + Sync {
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

    pub trait Bodega: Send + Sync {
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
        conexion: Arc<Mutex<Connection>>,
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
            Ok(AlmacenEnMemoria {
                conexion: Arc::new(Mutex::new(conn)),
            })
        }
    }

    impl Bodega for AlmacenEnMemoria {
        fn usar_insumo(&self, nueva_cantidad: u32, id: &String) -> AppResult<()> {
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion. : {}", e))
            })?;
            let columnas_afectadas = conexion_segura.execute(
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
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            let nombre = conexion_segura
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
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            let id: String = conexion_segura
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
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            conexion_segura.execute(
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
            Ok(())
        }

        fn editar_insumo(&self, insumo: negocio::Insumo) -> AppResult<()> {
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion. {}", e))
            })?;
            let afectados = conexion_segura.execute(
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
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            let funciono =
                conexion_segura.execute("DELETE FROM insumos WHERE id =?", params![id])?;
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
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            conexion_segura
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
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            let mut accion =
                conexion_segura.prepare("SELECT nombre FROM insumos ORDER BY nombre")?;
            let nombres_iter = accion.query_map([], |fila| fila.get(0))?;
            let mut nombres = Vec::new();
            for nombre in nombres_iter {
                nombres.push(nombre?);
            }
            Ok(nombres)
        }

        fn obtener_todos(&self) -> AppResult<Vec<Insumo>> {
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            let mut accion = conexion_segura.prepare(
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
    // Esta base de datos es para entidades que no deben mutar. podriamos agregar una funcion de una consulta SQL?
    pub trait BaseDatosNoModificable<T> {
        fn crear(&mut self, entidad: T) -> AppResult<()>;
    }

    pub trait BaseDatos<T>: Send + Sync {
        fn crear(&mut self, entidad: T) -> AppResult<()>;
        fn editar(&mut self, entidad: T) -> AppResult<()>;
        fn eliminar(&self, entidad_id: &str) -> AppResult<()>;
        fn listar(&self) -> AppResult<Vec<String>>;
        fn obtener(&self, busqueda: &str) -> AppResult<T>;
        fn id_con_nombre(&self, nombre: &str) -> AppResult<String>;
        fn nombre_con_id(&self, id: &str) -> AppResult<String>;
    }

    pub struct UsuariosDb {
        conexion: Arc<Mutex<Connection>>,
    }

    impl UsuariosDb {
        pub fn nuevo(ruta: &str) -> AppResult<UsuariosDb> {
            let conexion = Connection::open(ruta)?;
            conexion.execute(
                "CREATE TABLE IF NOT EXISTS usuarios (
                    id TEXT NOT NULL,
                    nombre TEXT NOT NULL,
                    hash TEXT NOT NULL,
                    rol TEXT NOT NULL
                    
                )",
                [],
            )?;
            Ok(UsuariosDb {
                conexion: Arc::new(Mutex::new(conexion)),
            })
        }
    }

    impl BaseDatos<negocio::Usuario> for UsuariosDb {
        fn crear(&mut self, datos: negocio::Usuario) -> AppResult<()> {
            let con = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            con.execute(
                "INSERT INTO usuarios (id, nombre, hash, rol)
                VALUES (?1, ?2, ?3, ?4)",
                params![
                    datos.obtener_id(),
                    datos.obtener_nombre(),
                    datos.obtener_hash(),
                    datos.obtener_rol()
                ],
            )?;
            Ok(())
        }
        fn editar(&mut self, datos: negocio::Usuario) -> AppResult<()> {
            let con = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion {}", e))
            })?;
            let afectados = con.execute(
                "UPDATE usuarios SET nombre = ?1, hash = ?2, rol =?3 WHERE id = ?4",
                params![
                    datos.obtener_nombre(),
                    datos.obtener_hash(),
                    datos.obtener_rol(),
                    datos.obtener_id()
                ],
            )?;
            if afectados == 0 {
                return Err(AppError::ErrorPersonal(format!(
                    "No se guardaron los cambios en: {}",
                    datos.obtener_nombre()
                )));
            }
            Ok(())
        }

        fn eliminar(&self, nombre: &str) -> AppResult<()> {
            let id = self.id_con_nombre(nombre)?;
            let con = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion:{}", e))
            })?;
            let funciono = con.execute("DELETE FROM usuarios WHERE id =?", params![id])?;
            if funciono == 0 {
                return Err(AppError::ErrorPersonal(format!(
                    "El Usuario: {}\n no fue modificado.",
                    nombre
                )));
            }
            Ok(())
        }

        fn id_con_nombre(&self, nombre: &str) -> AppResult<String> {
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            let id: String = conexion_segura
                .query_row(
                    "SELECT id FROM usuarios WHERE nombre = ?",
                    params![nombre],
                    |fila| fila.get(0),
                )
                .map_err(|e| match e {
                    rusqlite::Error::QueryReturnedNoRows => AppError::DatoInvalido(format!(
                        "En obtener id: No se encontro el usuario : {}",
                        nombre
                    )),
                    _ => AppError::DbError(e),
                })?;
            Ok(id)
        }

        fn nombre_con_id(&self, id: &str) -> AppResult<String> {
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            let nombre = conexion_segura
                .query_row(
                    "SELECT nombre FROM usuarios WHERE id = ?",
                    params![id],
                    |fila| fila.get(0),
                )
                .map_err(|e| match e {
                    rusqlite::Error::QueryReturnedNoRows => {
                        AppError::DatoInvalido(format!("No se encontro el usuario con id: {}", id))
                    }
                    _ => AppError::DbError(e),
                })?;
            Ok(nombre)
        }

        fn obtener(&self, busqueda: &str) -> AppResult<negocio::Usuario> {
            let id = self.id_con_nombre(busqueda)?;
            let con = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            con.query_row(
                "SELECT id, nombre, hash, rol FROM usuarios WHERE id = ?",
                params![id],
                |fila| {
                    Ok(negocio::Usuario::crear_desde_db(
                        fila.get(0)?,
                        fila.get(1)?,
                        fila.get(2)?,
                        fila.get(3)?,
                    ))
                },
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    AppError::DatoInvalido(format!("usuario: {}, no existe", busqueda))
                }
                _ => AppError::DbError(e),
            })
        }

        fn listar(&self) -> AppResult<Vec<String>> {
            let con = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("error al bloquear la conexion: {}", e))
            })?;
            let mut accion = con.prepare("SELECT nombre FROM usuarios ORDER BY nombre")?;
            let nombres_iter = accion.query_map([], |fila| fila.get(0))?;
            let mut nombres = Vec::new();
            for nombre in nombres_iter {
                nombres.push(nombre?);
            }
            Ok(nombres)
        }
    }

    pub struct GastosDB {
        conexion: Arc<Mutex<Connection>>,
    }

    impl GastosDB {
        pub fn nuevo(ruta: &str) -> AppResult<GastosDB> {
            let conexion = Connection::open(ruta)?;
            conexion.execute(
                "CREATE TEBLE IF NOT EXISTS gastos (
                    id TEXT NOT NULL,
                    insumo_id TEXT NOT NULL,
                    proveedor_id TEXT NOT NULL,
                    gasto_pesos REAL NOT NULL,
                    PRIMARY KEY (insumo_id),
                    PRIMARY KEY (proveedor_id),
                    FOREIGN KEY (insumo_id) REFERENCES insumos(id) ON DELETE CASCADE,
                    FOREIGN KEY (producto_id) REFERENCES proveedores(id) ON DELETE CASCADE                  
                )",
                [],
            )?;
            Ok(GastosDB {
                conexion: Arc::new(Mutex::new(conexion)),
            })
        }
    }

    impl BaseDatosNoModificable<negocio::Gasto> for GastosDB {
        fn crear(&mut self, datos: negocio::Gasto) -> AppResult<()> {
            let con = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            con.execute(
                "INSERT INTO gastos (id, insumo_id, proveedor_id, gasto_pesos)
                VALUES (?1, ?2, ?3, ?4)",
                params![
                    datos.id(),
                    datos.insumo_id(),
                    datos.proveedor_id(),
                    datos.gasto_pesos()
                ],
            )?;
            Ok(())
        }
    }

    pub struct ProveedoresDB {
        conexion: Arc<Mutex<Connection>>,
    }

    impl ProveedoresDB {
        pub fn nuevo(ruta: &str) -> AppResult<ProveedoresDB> {
            let conexion = Connection::open(ruta)?;
            conexion.execute(
                "CREATE TEBLE IF NOT EXISTS proveedores (
                    id TEXT NOT NULL,
                    marca TEXT NOT NULL,
                    numero TEXT NOT NULL,
                    producto_id TEXT NOT NULL,
                    PRIMARY KEY (producto_id),
                    FOREIGN KEY (producto_id) REFERENCES insumos(id) ON DELETE CASCADE
                )",
                [],
            )?;
            Ok(ProveedoresDB {
                conexion: Arc::new(Mutex::new(conexion)),
            })
        }
    }

    impl BaseDatos<Proveedor> for ProveedoresDB {
        fn crear(&mut self, datos: Proveedor) -> AppResult<()> {
            let con = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            con.execute(
                "INSERT INTO proveedores (id, marca, numero, producto_id)
                VALUES (?1, ?2, ?3, ?4)",
                params![
                    datos.obtener_id(),
                    datos.obtener_marca(),
                    datos.obtener_numero(),
                    datos.obtener_producto()
                ],
            )?;
            Ok(())
        }
        fn editar(&mut self, datos: Proveedor) -> AppResult<()> {
            let con = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion {}", e))
            })?;
            let afectados = con.execute(
                "UPDATE proveedores SET marca = ?1, numero = ?2, producto =?3 WHERE id = ?4",
                params![
                    datos.obtener_marca(),
                    datos.obtener_numero(),
                    datos.obtener_producto(),
                    datos.obtener_id()
                ],
            )?;
            if afectados == 0 {
                return Err(AppError::ErrorPersonal(format!(
                    "No se guardaron los cambios en: {}",
                    datos.obtener_marca()
                )));
            }
            Ok(())
        }

        fn eliminar(&self, nombre: &str) -> AppResult<()> {
            let id = self.id_con_nombre(nombre)?;
            let con = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion:{}", e))
            })?;
            let funciono = con.execute("DELETE FROM proveedores WHERE id =?", params![id])?;
            if funciono == 0 {
                return Err(AppError::ErrorPersonal(format!(
                    "El proveedor: {}\n no fue modificado.",
                    nombre
                )));
            }
            Ok(())
        }

        fn id_con_nombre(&self, nombre: &str) -> AppResult<String> {
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            let id: String = conexion_segura
                .query_row(
                    "SELECT id FROM proveedores WHERE nombre = ?",
                    params![nombre],
                    |fila| fila.get(0),
                )
                .map_err(|e| match e {
                    rusqlite::Error::QueryReturnedNoRows => AppError::DatoInvalido(format!(
                        "En obtener id: No se encontro el provedor : {}",
                        nombre
                    )),
                    _ => AppError::DbError(e),
                })?;
            Ok(id)
        }

        fn nombre_con_id(&self, id: &str) -> AppResult<String> {
            let conexion_segura = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            let nombre = conexion_segura
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

        fn obtener(&self, busqueda: &str) -> AppResult<Proveedor> {
            let id = self.id_con_nombre(busqueda)?;
            let con = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("Error al bloquear la conexion: {}", e))
            })?;
            con.query_row(
                "SELECT id, marca, numero, producto FROM proveedores WHERE id = ?",
                params![id],
                |fila| {
                    Ok(Proveedor::crear_desde_db(
                        fila.get(0)?,
                        fila.get(1)?,
                        fila.get(2)?,
                        fila.get(3)?,
                    ))
                },
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    AppError::DatoInvalido(format!("Proveedor: {}", busqueda))
                }
                _ => AppError::DbError(e),
            })
        }
        fn listar(&self) -> AppResult<Vec<String>> {
            let con = self.conexion.lock().map_err(|e| {
                AppError::ErrorPersonal(format!("error al bloquear la conexion: {}", e))
            })?;
            let mut accion = con.prepare("SELECT marca FROM proveedores ORDER BY nombre")?;
            let nombres_iter = accion.query_map([], |fila| fila.get(0))?;
            let mut nombres = Vec::new();
            for nombre in nombres_iter {
                nombres.push(nombre?);
            }
            Ok(nombres)
        }
    }
}

pub mod servicio {
    //SERVICIO: proporciona funciones usables por los comandos para conectarse a repositorio y verificar las existencias de productos antes de la creacion de una.
    // Además provee informacion de consulta para los comandos.
    //
    use crate::negocio::{self, AppError, AppResult};
    use crate::repositorio::{BaseDatos, Bodega, RecetasEnMemoria, UsuariosDb};
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

        pub fn mostrar_insumo(
            &self,
            busqueda: &String,
        ) -> AppResult<(String, String, u32, u32, u32)> {
            let insumo = self.obtener(busqueda)?;
            let insumo_tupla = (
                insumo.obtener_id().clone(),
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
            for _ in 0..cantidad {
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

    pub struct ServicioDeProveedores {
        repositorio: Box<dyn BaseDatos<negocio::Proveedor>>,
    }

    impl ServicioDeProveedores {
        pub fn nuevo(repositorio: Box<dyn BaseDatos<negocio::Proveedor>>) -> ServicioDeProveedores {
            ServicioDeProveedores { repositorio }
        }

        pub fn reinsertar(
            &mut self,
            id: String,
            marca: String,
            numero: String,
            producto: String,
        ) -> AppResult<()> {
            let proveedor = negocio::Proveedor::crear_desde_db(id, marca, numero, producto);
            self.repositorio.crear(proveedor)?;
            Ok(())
        }

        pub fn agregar(
            &mut self,
            marca: String,
            numero: String,
            producto: String,
        ) -> AppResult<()> {
            if let Ok(_) = self.existe(&marca) {
                return Err(AppError::DatoInvalido(format!(
                    "El proveedor: {}, ya existe",
                    marca
                )));
            }

            let proveedor = negocio::Proveedor::nuevo(marca, numero, producto)?;
            self.repositorio.crear(proveedor)?;
            Ok(())
        }

        pub fn nombre_con_id(&self, id: String) -> AppResult<String> {
            return self.repositorio.nombre_con_id(&id);
        }

        pub fn id_con_nombre(&self, nombre: String) -> AppResult<String> {
            return self.repositorio.id_con_nombre(&nombre);
        }

        pub fn buscar(&self, busqueda: &String) -> AppResult<Vec<String>> {
            let proveedores = self.repositorio.listar()?;
            let mut resultados: Vec<String> = Vec::new();
            //Primera Busqueda por contains:
            resultados = proveedores
                .clone()
                .into_iter()
                .filter(|proveedor| proveedor.contains(busqueda))
                .collect();
            if !resultados.is_empty() {
                return Ok(resultados);
            }
            //Segunda busqueda
            let probables = proveedores
                .into_iter()
                .min_by_key(|proveedor| levenshtein(proveedor, busqueda));
            match probables {
                Some(opcion) => {
                    resultados.push(opcion.clone());
                    Ok(resultados)
                }
                None => {
                    return Err(AppError::DatoInvalido(format!(
                        "No se encontro el proveedor: {}",
                        busqueda
                    )));
                }
            }
        }

        pub fn existe(&self, nombre: &str) -> AppResult<bool> {
            let lista = self.listar()?;
            if lista.contains(&nombre.to_string()) {
                return Ok(true);
            }
            return Ok(false);
        }

        pub fn listar(&self) -> AppResult<Vec<String>> {
            return self.repositorio.listar();
        }

        pub fn eliminar(&mut self, marca: &str) -> AppResult<()> {
            if let Ok(true) = self.existe(marca) {
                return Err(AppError::DatoInvalido(format!(
                    "No existe el proveedor: {}",
                    marca
                )));
            }
            self.repositorio.eliminar(marca)?;
            Ok(())
        }

        pub fn obtener(&self, busqueda: &str) -> AppResult<negocio::Proveedor> {
            if let Ok(true) = self.existe(busqueda) {
                return Err(AppError::DatoInvalido(format!(
                    "El proveedor: {}, no existe",
                    busqueda
                )));
            }
            Ok(self.repositorio.obtener(busqueda)?)
        }
    }

    pub struct ServicioDeUsuarios {
        repositorio: Box<dyn BaseDatos<negocio::Usuario>>,
    }

    impl ServicioDeUsuarios {
        pub fn nuevo(repositorio: Box<dyn BaseDatos<negocio::Usuario>>) -> ServicioDeUsuarios {
            ServicioDeUsuarios { repositorio }
        }

        pub fn verificar_usuario(&self, contra: &str, usuario: &str) -> AppResult<String> {
            let usuario = self.repositorio.obtener(&usuario)?;
            return match usuario.verificar_hash(contra) {
                Ok(token) => Ok(token),
                Err(e) => Err(AppError::DatoInvalido(format!("{}", e))),
            };
        }

        pub fn agregar(&mut self, nombre: String, contra: String, rol: String) -> AppResult<()> {
            if self.existe(&nombre)? {
                println!("Existe, 3170");
                return Err(AppError::DatoInvalido(format!(
                    "El usuario: {}, ya existe",
                    nombre
                )));
            }

            let usuario = negocio::Usuario::nuevo(&nombre, rol, &contra)?;
            match self.repositorio.crear(usuario) {
                Ok(_) => return Ok(()),
                Err(e) => return Err(AppError::ErrorPersonal(e.to_string())),
            }
        }

        pub fn existe(&self, nombre: &str) -> AppResult<bool> {
            let lista = self.repositorio.listar()?;
            if lista.contains(&nombre.to_string()) {
                return Ok(true);
            }
            return Ok(false);
        }

        pub fn eliminar(&mut self, nombre: &str) -> AppResult<()> {
            if !self.existe(nombre)? {
                return Err(AppError::DatoInvalido(format!(
                    "No existe el usuario: {}",
                    nombre
                )));
            }
            self.repositorio.eliminar(nombre)?;
            Ok(())
        }

        pub fn nombre_con_id(&self, id: String) -> AppResult<String> {
            return self.repositorio.nombre_con_id(&id);
        }

        pub fn id_con_nombre(&self, nombre: String) -> AppResult<String> {
            return self.repositorio.id_con_nombre(&nombre);
        }

        pub fn obtener(&self, busqueda: &str) -> AppResult<(String, String, String)> {
            if !self.existe(busqueda)? {
                println!("3213");
                return Err(AppError::DatoInvalido(format!(
                    "El usuario: {}, no existe",
                    busqueda
                )));
            }
            let usuario = self.repositorio.obtener(busqueda)?;
            Ok((
                usuario.obtener_id(),
                usuario.obtener_nombre(),
                usuario.obtener_rol(),
            ))
        }

        pub fn listar(&self) -> AppResult<Vec<String>> {
            return self.repositorio.listar();
        }

        pub fn buscar(&self, busqueda: &String) -> AppResult<Vec<String>> {
            let usuarios = self.repositorio.listar()?;
            let mut resultados: Vec<String> = Vec::new();
            //Primera Busqueda por contains:
            resultados = usuarios
                .clone()
                .into_iter()
                .filter(|usuario| usuario.contains(busqueda))
                .collect();
            if !resultados.is_empty() {
                return Ok(resultados);
            }
            //Segunda busqueda
            let probables = usuarios
                .into_iter()
                .min_by_key(|usuario| levenshtein(usuario, busqueda));
            match probables {
                Some(opcion) => {
                    resultados.push(opcion.clone());
                    Ok(resultados)
                }
                None => {
                    return Err(AppError::DatoInvalido(format!(
                        "No se encontro el usuario: {}",
                        busqueda
                    )));
                }
            }
        }
    }
}
