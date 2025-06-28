import type { Insumo, InsumoEditado, InsumoValor } from './modelos.js';
import type { InsumosConsulta } from './contratos.js';

const url_base = 'http://127.0.0.1:8080/insumos';

export const servicioDeInsumos: InsumosConsulta = {
  async crear(datos: Insumo): Promise<Response> {
    return fetch(`${url_base}/crear`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(datos)
    });
  },
  async buscarPorNombre(nombre: string): Promise<string[]> {
    const respuesta = await fetch(`${url_base}/buscar?consulta=${encodeURIComponent(nombre)}`);
    if (!respuesta.ok) throw new Error('Error al buscar insumo');
    return respuesta.json();
  },

  async listar(): Promise<string[]> {
    const respuesta = await fetch(`${url_base}/todos`);
    if (!respuesta.ok) throw new Error('Error al listar insumos');
    return respuesta.json();
  },

  async valorInsumo(nombre: string): Promise<InsumoValor> {
    const respuesta = await fetch(`${url_base}/valor?consulta=${encodeURIComponent(nombre)}`);
    if (!respuesta.ok) throw new Error(`Error al buscar el insumo ${nombre}`);
    const info: InsumoValor = await respuesta.json();
    return info; 
  },
  
  async editarInsumo(datos:InsumoEditado): Promise<Response> {
    return fetch(`${url_base}/editar`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json'},
      body: JSON.stringify(datos)
    });
  },
  async eliminarInsumo(nombre: string): Promise<Response>{
    const respuesta = await fetch(`${url_base}/${encodeURIComponent(nombre)}`);
    if (!respuesta.ok) throw new Error(`Error al eliminar el insumo: ${nombre}`);
    return respuesta.json();
  }
};
