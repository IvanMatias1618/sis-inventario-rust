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
import type { Insumo } from './modelos.js';

const form = document.getElementById('crear__insumo') as HTMLFormElement;
const submitBtn = document.getElementById('btn__agregar_insumo') as HTMLButtonElement;

form.addEventListener('submit', async (event: SubmitEvent) => {
  event.preventDefault();
  const formInfo = new FormData(form);
  const nombre = formInfo.get('nombre') as string;
  const cantidad = Number(formInfo.get('cantidad'));
  const cantidad_minima = Number(formInfo.get('cantidad_minima'));
  const precio = Number(formInfo.get('precio'));

  if (!nombre) alert("El nombre esta vacio.");
  if (cantidad <= 0) alert("La cantidad no puede ser negativa");
  if (cantidad_minima <= 0) alert("La cantidad minima no puede ser negativa");
  if (precio <= 0) alert("El precio no puede ser negativo");

  const nuevoInsumo: Insumo = {
    nombre,
    cantidad,
    cantidad_minima,
    precio
  };
  try {
    const respuesta = await servicioDeInsumos.crear(nuevoInsumo);
    if (!respuesta.ok) throw new Error('Fallo la creacion');
    console.log('Insumo creado Exitosamente');
  } catch (error) {
    console.error('Error al enviar insumo:', error);
  }
});

const contenedorFormularios = document.querySelector("formularios__insumos_contenedor") as HTMLElement;
document.querySelectorAll("#menu_formularios li").forEach( item => {
  //OCULTAR: todos los forms
  item.addEventListener("click", () => {
    Array.from(contenedorFormularios.children).forEach( formulario => {
      formulario.classList.remove("activo");
    });
    const id = item.getAttribute("data-formulario");
    if (id) {
      const formAmostrar = document.getElementById(id);
      formAmostrar?.classList.add("active");
    }
  });
});
