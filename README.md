1. Gestionar el acceso mediante roles custom
2. Almacenar contraseñas
3. Crear "namespaces" para las contraseña
4. Agregar una fecha de deprecación para las contraseñas
5. Almacenar quien está utilizando cierta contraseña mediante una auditoria
6. Exponer métricas para que se pueda conectar con Grafana y Kibana
7. Utilizar Oauth 2 a través de Keycloak
8. Habilitar TLS para los endpoints
9. Crear una especie de Swagger o contrato para que cualquiera pueda ver los endpoints
   10 . Utilizar un secret id y client id para los bots
11. Renovar los secretos automáticamente
12. Referenciar los secretos por id, de forma que el secreto viaje encriptado y se desencripte a través de una clave generada para la petición, pero que expire pasado cierto tiempo.

Más adelante, en una fase futura:
13. Aplicar reglas lógicas para bloquear peticiones a ciertos usuarios, como la MAC o cosas parecidas
14. Aplicar un bloqueo general al servicio en base a anomalias en las peticiones o fallos de seguridad.
15. Monitorizar quie se está conectando y exponerlo en las mátricas
16. Agregar labels a modo de metadatros en las peticiones para poder clasificar las peticiones de secretos.
