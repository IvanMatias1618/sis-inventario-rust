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
/* CREAR: Insumo   */
const form = document.getElementById('crear__insumo');
const submitBtn = document.getElementById('btn__agregar_insumo');
form.addEventListener('submit', (event) => __awaiter(void 0, void 0, void 0, function* () {
    event.preventDefault();
    const formInfo = new FormData(form);
    const nombre = formInfo.get('nombre');
    const cantidad = Number(formInfo.get('cantidad'));
    const cantidad_minima = Number(formInfo.get('cantidad_minima'));
    const precio = Number(formInfo.get('precio'));
    if (!nombre) {
        alert("El nombre esta vacio.");
        return;
    }
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
    servicioDeInsumos.crear(nuevoInsumo).then((res) => __awaiter(void 0, void 0, void 0, function* () {
        const data = yield res.json();
        const resultado = formatearRespuesta(data, res.status);
        renderRespuesta("crear__insumo", resultado);
    })).catch(() => {
        const resultado = formatearRespuesta({ error: 'servidor no responde' }, 500);
        renderRespuesta('crear__insumo', resultado);
    });
}));
/*  BUSCAR: insumo */
const formBuscar = document.getElementById("buscar__insumo");
const btn_buscar = document.getElementById("btn_buscar__insumo");
formBuscar.addEventListener('submit', (event) => __awaiter(void 0, void 0, void 0, function* () {
    event.preventDefault();
    const formInfo = new FormData(formBuscar);
    const nombre = formInfo.get('nombre');
    if (!nombre)
        alert("El nombre esta vacio");
    servicioDeInsumos.buscarPorNombre(nombre).then((res) => __awaiter(void 0, void 0, void 0, function* () {
        const data = yield res.json();
        const resultado = formatearRespuesta(data, res.status);
        renderRespuesta("buscar__insumo", resultado);
    })).catch(() => {
        const resultado = formatearRespuesta({ error: 'servidor no responde' }, 500);
        renderRespuesta('buscar__insumo', resultado);
    });
}));
/*  TODOS: los insumos.  */
const formTodos = document.getElementById("insumos_todos");
formTodos.addEventListener('submit', (event) => __awaiter(void 0, void 0, void 0, function* () {
    event.preventDefault();
    servicioDeInsumos.listar().then((res) => __awaiter(void 0, void 0, void 0, function* () {
        const data = yield res.json();
        const resultado = formatearRespuesta(data, res.status);
        renderRespuesta("insumos_todos", resultado);
    })).catch(() => {
        const resultado = formatearRespuesta({ error: 'servidor no responde' }, 500);
        renderRespuesta('insumos_todos', resultado);
    });
}));
/*  VALOR: de un insumo.  */
const formValor = document.getElementById("valor_insumo");
const btn_valor = document.getElementById("btn_valor_insumo");
formValor.addEventListener('submit', (event) => __awaiter(void 0, void 0, void 0, function* () {
    event.preventDefault();
    const formInfo = new FormData(formValor);
    const nombre = formInfo.get("nombre__valor_insumo");
    if (!nombre)
        alert("el nombre esta vacio");
    servicioDeInsumos.valorInsumo(nombre).then((res) => __awaiter(void 0, void 0, void 0, function* () {
        const data = yield res.json();
        const resultado = formatearRespuesta(data, res.status);
        renderRespuesta("valor_insumo", resultado);
    })).catch(() => {
        const resultado = formatearRespuesta({ error: 'servidor no responde' }, 500);
        renderRespuesta('valor_insumo', resultado);
    });
}));
/*  EDITAR: insumo  */
const formEdit = document.getElementById("editar__insumo");
const btn_edit = document.getElementById("btn_editar__insumo");
formEdit.addEventListener('submit', (event) => __awaiter(void 0, void 0, void 0, function* () {
    event.preventDefault();
    const formInfo = new FormData(formEdit);
    const insumo = formInfo.get("insumo");
    const insumoEditado = {};
    const nombre = formInfo.get("nombre");
    if (nombre)
        insumoEditado.nombre = nombre;
    const cantidadStr = formInfo.get("cantidad");
    const cantidad = Number(cantidadStr);
    if (cantidadStr && !isNaN(cantidad))
        insumoEditado.cantidad = cantidad;
    const cantidadMinStr = formInfo.get("cantidad_minima");
    const cantidad_minima = Number(cantidadMinStr);
    if (cantidadMinStr && !isNaN(cantidad_minima))
        insumoEditado.cantidad_minima = cantidad_minima;
    const precioStr = formInfo.get("precio");
    const precio = Number(precioStr);
    if (precioStr && !isNaN(precio))
        insumoEditado.precio = precio;
    servicioDeInsumos.editarInsumo(insumo, insumoEditado).then((res) => __awaiter(void 0, void 0, void 0, function* () {
        const data = yield res.json();
        const resultado = formatearRespuesta(data, res.status);
        renderRespuesta("editar__insumo", resultado);
    })).catch(() => {
        const resultado = formatearRespuesta({ error: 'servidor no responde' }, 500);
        renderRespuesta('editar__insumo', resultado);
    });
}));
/*   ELIMINAR: insumo  */
const formEliminar = document.getElementById("eliminar__insumo");
const btn_eliminar = document.getElementById("btn_eliminar__insumo");
formEliminar.addEventListener('submit', (event) => __awaiter(void 0, void 0, void 0, function* () {
    event.preventDefault();
    const formInfo = new FormData(formEliminar);
    const insumo = formInfo.get("nombre");
    if (!insumo)
        alert("Que insumo quieres eliminar?");
    servicioDeInsumos.eliminarInsumo(insumo).then((res) => __awaiter(void 0, void 0, void 0, function* () {
        const data = yield res.json();
        const resultado = formatearRespuesta(data, res.status);
        renderRespuesta("eliminar__insumo", resultado);
    })).catch(() => {
        const resultado = formatearRespuesta({ error: 'servidor no responde' }, 500);
        renderRespuesta('eliminar__insumo', resultado);
    });
}));
/* AUXILIARES: */
function formatearRespuesta(respuesta, status) {
    if (status >= 200 && status < 300) {
        return {
            mensaje: (respuesta === null || respuesta === void 0 ? void 0 : respuesta.message) || '¡Todo salió bien! 🌟',
            tipo: 'success',
        };
    }
    else {
        const errores = (respuesta === null || respuesta === void 0 ? void 0 : respuesta.fieldErrors) || {};
        const campos = Object.keys(errores).join(', ');
        return {
            mensaje: campos
                ? `Revisa los campos: ${campos} 🧸`
                : (respuesta === null || respuesta === void 0 ? void 0 : respuesta.error) || 'Algo falló 💔',
            tipo: 'error',
            errores,
        };
    }
}
function renderRespuesta(nombreFormulario, resultado) {
    const form = document.getElementById(nombreFormulario);
    const div = form.querySelector('[name="respuesta"]');
    if (div) {
        div.innerHTML = `💬 <strong>${resultado.mensaje}</strong>`;
        div.classList.remove('oculto', 'success', 'error');
        div.classList.add('visible', resultado.tipo);
    }
    // Si hay errores por campo, vamos a pintarlos lindos también
    if (resultado.errores) {
        for (const campo in resultado.errores) {
            const input = form.querySelector(`[name="${campo}"]`);
            if (input) {
                input.classList.add('input-error'); // Estilo visual para el campo
                // Crear tooltip o mensaje visual (aquí puedes personalizar más)
                const msg = document.createElement('small');
                msg.textContent = `⚠️ ${resultado.errores[campo]}`;
                msg.classList.add('mensaje-campo');
                input.insertAdjacentElement('afterend', msg);
            }
        }
    }
}
/*  RENDERIZAR: formularios   */
const contenedorFormularios = document.getElementById("formularios__insumos_contenedor");
document.querySelectorAll("#menu_formularios li").forEach(item => {
    //OCULTAR: todos los forms
    item.addEventListener("click", () => {
        Array.from(contenedorFormularios.children).forEach(formulario => {
            formulario.classList.remove("activo");
        });
        const id = item.getAttribute("data-formulario");
        if (id) {
            const formAmostrar = document.getElementById(id);
            formAmostrar === null || formAmostrar === void 0 ? void 0 : formAmostrar.classList.add("activo");
        }
    });
});
