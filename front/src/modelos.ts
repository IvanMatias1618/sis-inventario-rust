export interface Insumo {
  nombre: string;
  cantidad: number;
  cantidad_minima: number;
  precio: number;
}

export interface InsumoEditado {
  id: number,
  nombre?: string,
  cantidad?: number,
  cantidad_minima?: number,
  precio?: number,
}

export interface InsumoValor {
  id: string,
  nombre: string,
  cantidad: number,
  cantidad_minima: number,
  precio: number
}

export interface Respuesta {
  mensaje: string;
}

export interface Lista {
  respuesta: string[];
}
