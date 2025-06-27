import type { Insumo } from './modelos.js';
import type { InsumosConsulta } from './contratos.js';

const url_base = 'http://127.0.0.1:8080/insumos';

export const servicioDeInsumos: InsumosConsulta = {
  async crear(datos: Insumo): Promise<Response> {
    return fetch(`${url_base}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(datos)
    });
  },
  async buscarPorNombre(nombre: string): Promise<string[]> {
    const respuesta = await fetch(`${url_base}/${encodeURIComponent(nombre)}`);
    if (!respuesta.ok) throw new Error('Error al buscar insumo');
    return respuesta.json();
  },

  async listar(): Promise<string[]> {
    const respuesta = await fetch(`${url_base}/listar`);
    if (!respuesta.ok) throw new Error('Error al listar insumos');
    return respuesta.json();
  }
};
