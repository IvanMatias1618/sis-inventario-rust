var form = document.getElementById('crear__insumo');
var submitBtn = document.getElementById('btn__agregar_insumo');
form.addEventListener('submit', function (event) { return ; }, {
    event: event,
    : .preventDefault(),
    const: formInfo = new FormData(form),
    const: nombre = formInfo.get('nombre'),
    const: cantidad = Number(formInfo.get('cantidad')),
    const: cantidad_minima = Number(formInfo.get('cantidad_minima')),
    const: precio = Number(formInfo.get('precio'))
});
