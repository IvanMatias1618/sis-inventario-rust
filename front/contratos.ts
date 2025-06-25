import type { Insumo, Respuesta, Lista } from './modelos';

export interface InsumosConsulta {
  crear: (insumo: Insumo) => Promise<Response>;
  buscarPorNombre:(nombre: string) => Promise<string[]>;
  listar: () => Promise<string[]>; 
}


