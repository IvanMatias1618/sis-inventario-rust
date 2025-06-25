const form = document.getElementById('crear__insumo') as HTMLFormElement;
const submitBtn = document.getElementById('btn__agregar_insumo') as HTMLButtonElement;

form.addEventListener('submit', (event: SubmitEvent) = {
  event.preventDefault();
  const formInfo = new FormData(form);
  const nombre = formInfo.get('nombre') as string;
  const cantidad = Number(formInfo.get('cantidad'));
  const cantidad_minima = Number(formInfo.get('cantidad_minima'));
  const precio = Number(formInfo.get('precio'));
});
