/*╭🌸╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌✦╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌🌸╮
  │  🧠 nodo central del pensamiento
  │
  │  MODULOS: [ contratos.ts, servicio.ts, modelos.ts]
  │
  │  🐾 seccion abierta: []
  │     ⤷ módulo activo, lógica actual, ritual en curso
  │
  │  🌞 tareas actuales: []
  │     ⤷ qué se está orquestando ahora
  │
  │  🔬 tareas futuras: []
  │     ⤷ cosas que aún duermen pero susurran promesas
  │
  │  🌌 pendientes: []
  │     ⤷ rarezas, condiciones límite, TODOs que acechan
  ╰🌸╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌✦╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌🌸╯*/
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import { servicioDeInsumos } from './servicio.js';
const form = document.getElementById('crear__insumo');
const submitBtn = document.getElementById('btn__agregar_insumo');
form.addEventListener('submit', (event) => __awaiter(void 0, void 0, void 0, function* () {
    event.preventDefault();
    const formInfo = new FormData(form);
    const nombre = formInfo.get('nombre');
    const cantidad = Number(formInfo.get('cantidad'));
    const cantidad_minima = Number(formInfo.get('cantidad_minima'));
    const precio = Number(formInfo.get('precio'));
    if (!nombre)
        alert("El nombre esta vacio.");
    if (cantidad <= 0)
        alert("La cantidad no puede ser negativa");
    if (cantidad_minima <= 0)
        alert("La cantidad minima no puede ser negativa");
    if (precio <= 0)
        alert("El precio no puede ser negativo");
    const nuevoInsumo = {
        nombre,
        cantidad,
        cantidad_minima,
        precio
    };
    try {
        const respuesta = yield servicioDeInsumos.crear(nuevoInsumo);
        if (!respuesta.ok)
            throw new Error('Fallo la creacion');
        console.log('Insumo creado Exitosamente');
    }
    catch (error) {
        console.error('Error al enviar insumo:', error);
    }
}));
const contenedorFormularios = document.querySelector("formularios__insumos_contenedor");
document.querySelectorAll("#menu_formularios li").forEach(item => {
    //OCULTAR: todos los forms
    item.addEventListener("click", () => {
        Array.from(contenedorFormularios.children).forEach(formulario => {
            formulario.classList.remove("activo");
        });
        const id = item.getAttribute("data-formulario");
        if (id) {
            const formAmostrar = document.getElementById(id);
            formAmostrar === null || formAmostrar === void 0 ? void 0 : formAmostrar.classList.add("active");
        }
    });
});
