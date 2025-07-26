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



import { servicioDeInsumos } from './servicio.js';
import type { InsumosConsulta } from './contratos.js';
import type { Insumo, InsumoEditado, InsumoValor } from './modelos.js';

/* CREAR: Insumo   */

const form = document.getElementById('crear__insumo') as HTMLFormElement;
const submitBtn = document.getElementById('btn__agregar_insumo') as HTMLButtonElement;

form.addEventListener('submit', async (event: SubmitEvent) => {
  event.preventDefault();
  const formInfo = new FormData(form);
  const nombre = formInfo.get('nombre') as string;
  const cantidad = Number(formInfo.get('cantidad'));
  const cantidad_minima = Number(formInfo.get('cantidad_minima'));
  const precio = Number(formInfo.get('precio'));

  if (!nombre) { alert("El nombre esta vacio."); return; }
  if (cantidad <= 0) alert("La cantidad no puede ser negativa");
  if (cantidad_minima <= 0) alert("La cantidad minima no puede ser negativa");
  if (precio <= 0) alert("El precio no puede ser negativo");

  const nuevoInsumo: Insumo = {
    nombre,
    cantidad,
    cantidad_minima,
    precio
  };

  servicioDeInsumos.crear(nuevoInsumo).then(async res => {
    const data = await res.json();
    const resultado = formatearRespuesta(data, res.status);
    renderRespuesta("crear__insumo", resultado);
  }).catch(() => {
    const resultado = formatearRespuesta({ error: 'servidor no responde' }, 500);
    renderRespuesta('crear__insumo', resultado);
  });


});

/*  BUSCAR: insumo */

const formBuscar = document.getElementById("buscar__insumo") as HTMLFormElement;
const btn_buscar = document.getElementById("btn_buscar__insumo") as HTMLButtonElement;

formBuscar.addEventListener('submit', async (event: SubmitEvent) => {
  event.preventDefault();
  const formInfo = new FormData(formBuscar);
  const nombre = formInfo.get('nombre') as string;
  if (!nombre) alert("El nombre esta vacio");

  servicioDeInsumos.buscarPorNombre(nombre).then(async res => {
    const data = await res.join(', ');
    const resultado = formatearRespuesta({ message: data }, 200);
    renderRespuesta("buscar__insumo", resultado);
  }).catch(() => {
    const resultado = formatearRespuesta({ error: 'servidor no responde' }, 500);
    renderRespuesta('buscar__insumo', resultado);
  });


});

/*  TODOS: los insumos.  */

const formTodos = document.getElementById("insumos_todos") as HTMLFormElement;

formTodos.addEventListener('submit', async (event: SubmitEvent) => {
  event.preventDefault();
  servicioDeInsumos.listar().then(async res => {
    const data = res.join(`, `);
    const resultado = formatearRespuesta({ message: data }, 200);
    renderRespuesta("insumos_todos", resultado);
  }).catch(() => {
    const resultado = formatearRespuesta({ error: 'servidor no responde' }, 500);
    renderRespuesta('insumos_todos', resultado);
  });


});

/*  VALOR: de un insumo.  */

const formValor = document.getElementById("valor_insumo") as HTMLFormElement;
const btn_valor = document.getElementById("btn_valor_insumo") as HTMLButtonElement;
formValor.addEventListener('submit', async (event: SubmitEvent) => {
  event.preventDefault();
  const formInfo = new FormData(formValor);
  const nombre = formInfo.get("nombre__valor_insumo") as string;
  if (!nombre) alert("el nombre esta vacio");

  servicioDeInsumos.valorInsumo(nombre).then(async res => {
    const info = `id: ${res.id} nombre: ${res.nombre}, cantidad: ${res.cantidad},cantidad minima: ${res.cantidadMinima}, precio por kilo: ${res.precio}`;
    const resultado = formatearRespuesta({ message: info }, 200);
    renderRespuesta("valor_insumo", resultado);
  }).catch(() => {
    const resultado = formatearRespuesta({ error: 'servidor no responde' }, 500);
    renderRespuesta('valor_insumo', resultado);
  });


});

/*  EDITAR: insumo  */

const formEdit = document.getElementById("editar__insumo") as HTMLFormElement;
const btn_edit = document.getElementById("btn_editar__insumo") as HTMLButtonElement;
formEdit.addEventListener('submit', async (event: SubmitEvent) => {
  event.preventDefault();
  const formInfo = new FormData(formEdit);
  const insumo = formInfo.get("insumo") as string;

  const insumoEditado: Partial<InsumoEditado> = {};

  const nombre = formInfo.get("nombre") as string;
  if (nombre) insumoEditado.nombre = nombre;

  const cantidadStr = formInfo.get("cantidad") as string;
  const cantidad = Number(cantidadStr);
  if (cantidadStr && !isNaN(cantidad)) insumoEditado.cantidad = cantidad;

  const cantidadMinStr = formInfo.get("cantidad_minima") as string;
  const cantidad_minima = Number(cantidadMinStr);
  if (cantidadMinStr && !isNaN(cantidad_minima)) insumoEditado.cantidad_minima = cantidad_minima;

  const precioStr = formInfo.get("precio") as string;
  const precio = Number(precioStr);
  if (precioStr && !isNaN(precio)) insumoEditado.precio = precio;

  servicioDeInsumos.editarInsumo(insumo, insumoEditado).then(async res => {
    const data = await res.json();
    const resultado = formatearRespuesta(data, res.status);
    renderRespuesta("editar__insumo", resultado);
  }).catch(() => {
    const resultado = formatearRespuesta({ error: 'servidor no responde' }, 500);
    renderRespuesta('editar__insumo', resultado);
  });



});

/*   ELIMINAR: insumo  */

const formEliminar = document.getElementById("eliminar__insumo") as HTMLFormElement;
const btn_eliminar = document.getElementById("btn_eliminar__insumo") as HTMLButtonElement;

formEliminar.addEventListener('submit', async (event: SubmitEvent) => {
  event.preventDefault();
  const formInfo = new FormData(formEliminar);
  const insumo = formInfo.get("nombre") as string;
  if (!insumo) alert("Que insumo quieres eliminar?");

  servicioDeInsumos.eliminarInsumo(insumo).then(async res => {
    const data = await res.json();
    const resultado = formatearRespuesta(data, res.status);
    renderRespuesta("eliminar__insumo", resultado);
  }).catch(() => {
    const resultado = formatearRespuesta({ error: 'servidor no responde' }, 500);
    renderRespuesta('eliminar__insumo', resultado);
  });
});

/* AUXILIARES: */

function formatearRespuesta(respuesta: any, status: number): ResultadoRespuesta {
  if (status >= 200 && status < 300) {
    return {
      mensaje: respuesta?.message || '¡Todo salió bien! 🌟',
      tipo: 'success',
    };
  } else {
    const errores = respuesta?.fieldErrors || {};
    const campos = Object.keys(errores).join(', ');
    return {
      mensaje: campos
        ? `Revisa los campos: ${campos} 🧸`
        : respuesta?.error || 'Algo falló 💔',
      tipo: 'error',
      errores,
    };
  }
}

/* FORMATEAR: respuesta   */

type ResultadoRespuesta = {
  mensaje: string;
  tipo: 'success' | 'error';
  errores?: Record<string, string>;
};

function renderRespuesta(nombreFormulario: string, resultado: ResultadoRespuesta) {
  const form = document.getElementById(nombreFormulario) as HTMLFormElement;
  const div = form.querySelector('[name="respuesta"]');

  if (div) {
    div.innerHTML = `💬 <strong>${resultado.mensaje}</strong>`;
    div.classList.remove('oculto', 'success', 'error');
    div.classList.add('visible', resultado.tipo);
  }

  // Si hay errores por campo, vamos a pintarlos lindos también
  if (resultado.errores) {
    for (const campo in resultado.errores) {
      const input = form.querySelector(`[name="${campo}"]`) as HTMLElement;
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

const contenedorFormularios = document.getElementById("formularios__insumos_contenedor") as HTMLElement;
document.querySelectorAll("#menu_formularios li").forEach(item => {
  //OCULTAR: todos los forms
  item.addEventListener("click", () => {
    Array.from(contenedorFormularios.children).forEach(formulario => {
      formulario.classList.remove("activo");
    });
    const id = item.getAttribute("data-formulario");
    if (id) {
      const formAmostrar = document.getElementById(id);
      formAmostrar?.classList.add("activo");
    }
  });
});
