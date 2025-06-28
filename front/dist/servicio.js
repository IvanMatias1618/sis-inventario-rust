var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
const url_base = 'http://127.0.0.1:8080/insumos';
export const servicioDeInsumos = {
    crear(datos) {
        return __awaiter(this, void 0, void 0, function* () {
            return fetch(`${url_base}/crear`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(datos)
            });
        });
    },
    buscarPorNombre(nombre) {
        return __awaiter(this, void 0, void 0, function* () {
            const respuesta = yield fetch(`${url_base}/buscar?consulta=${encodeURIComponent(nombre)}`);
            if (!respuesta.ok)
                throw new Error('Error al buscar insumo');
            return respuesta.json();
        });
    },
    listar() {
        return __awaiter(this, void 0, void 0, function* () {
            const respuesta = yield fetch(`${url_base}/todos`);
            if (!respuesta.ok)
                throw new Error('Error al listar insumos');
            return respuesta.json();
        });
    },
    valorInsumo(nombre) {
        return __awaiter(this, void 0, void 0, function* () {
            const respuesta = yield fetch(`${url_base}/valor?consulta=${encodeURIComponent(nombre)}`);
            if (!respuesta.ok)
                throw new Error(`Error al buscar el insumo ${nombre}`);
            const info = yield respuesta.json();
            return info;
        });
    },
    editarInsumo(datos) {
        return __awaiter(this, void 0, void 0, function* () {
            return fetch(`${url_base}/editar`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(datos)
            });
        });
    },
    eliminarInsumo(nombre) {
        return __awaiter(this, void 0, void 0, function* () {
            const respuesta = yield fetch(`${url_base}/${encodeURIComponent(nombre)}`);
            if (!respuesta.ok)
                throw new Error(`Error al eliminar el insumo: ${nombre}`);
            return respuesta.json();
        });
    }
};
