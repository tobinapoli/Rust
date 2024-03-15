# Club SemRust - Sistema de Registro de Pagos y Gestión de Actividades en Blockchain

## Descripción
Este proyecto implementa un sistema en blockchain para el registro de pagos de cuotas mensuales y la gestión de actividades deportivas para los socios del Club SemRust. Lo hace mediante dos SmartContracts, uno principal para la funcionalidad del club y otro que genera reportes a traves de la infomracion del primero

## Funcionalidades
1. **Registro de un nuevo socio:**
   - Al registrar un nuevo socio, se debe seleccionar la categoría correspondiente.
   - Se crea un pago pendiente con vencimiento en los próximos 10 días.

2. **Registro de un pago:**
   - Los socios pueden realizar el pago de su cuota mensual indicando su número de identificación (DNI) y el monto pagado.
   - El sistema verifica que el monto pagado corresponda a la categoría del socio y registra el pago en la blockchain.

3. **Consulta de pagos:**
   - Se puede consultar la lista de pagos realizados, mostrando la información del socio, la categoría y el monto pagado.

4. **Bonificación por pagos consecutivos:**
   - Si un socio acumula un número determinado de pagos consecutivos sin atrasos, se otorga un descuento en la cuota mensual del siguiente mes.

## Reportes (a través de otro contrato)
1. **Verificación de pagos pendientes:**
   - Muestra un listado de socios morosos (con pagos pendientes después de la fecha de vencimiento).

2. **Informe de recaudación:**
   - Genera un informe mensual con el total recaudado para cada categoría de socio.

3. **Reporte de socios no morosos para una actividad específica:**
   - Genera un reporte de los socios no morosos que tienen permitido asistir a una actividad deportiva específica.

## Contratos
- Se maneja un listado de direcciones autorizadas para realizar operaciones.
- El dueño del contrato puede autorizar o desautorizar direcciones.
- Se puede activar o desactivar la política de autorización.
- El dueño del contrato puede delegar el poder a otra dirección.

## Testing y Documentación
- Testing realizado sobre todas las funcionalidades (coverage 85%).
- Documentantacion de todos los structs y métodos.
