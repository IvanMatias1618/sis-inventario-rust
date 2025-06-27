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
