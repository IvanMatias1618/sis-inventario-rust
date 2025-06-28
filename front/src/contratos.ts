import type { Insumo, Respuesta, Lista, InsumoEditado, InsumoValor } from './modelos.js';

export interface InsumosConsulta {
  crear: (insumo: Insumo) => Promise<Response>;
  buscarPorNombre: (nombre: string) => Promise<string[]>;
  listar: () => Promise<string[]>;
  valorInsumo: (nombre:string)=> Promise<InsumoValor>;
  editarInsumo: (datos: InsumoEditado) => Promise<Response>;
  eliminarInsumo: (nombre: string) => Promise<Response>
}




