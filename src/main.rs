fn main() {
    println!("Hello, world!");
}

mod entidades {
    use chrono::DateTime;
    pub struct Producto {
        id: String,
        nombre: String,
        cantidad: u32,
        cantidad_minima: u32,
    }
    pub struct IngredienteReceta {
        producto_id: String,
        cantidad: u32,
    }

    pub struct Receta {
        id: String,
        nombre: String,
        ingredientes: Vec<IngredienteReceta>,
    }

    pub struct Venta {
        fecha: DateTime,
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
