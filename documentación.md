# Introducción
Este proyecto a grandes rasgos consiste en crear un filesystem en el lenguaje rust o C,
este debe residir en el espacio de usuario y tiene como objetivo usar imágenes blanco
y negro para almacenar archivos. Su información se almacena en los pixeles de color
blanco y negro.
Es importante que este FileSystem tenga las siguientes funciones de la biblioteca
FUSE: getattr, create, open, read, write, rename, mkdir, readdir, opendir, rmdir,
statfs, fsync, access, unlink, flush y iseek.
Como máximo debe usar 1000x1000 px. El sistema debe usar i-nodos como estructura
de indexación.

Debe contener los siguientes archivos:
● Mkfs.bwfs (Crea el FS)
● Fsck.bwfs (Realiza un chequeo de consistencia del BWFS)
● Mount.bwfs (Lo monta en el Sistema Operativo)
Es importante destacar que el FS debe ser persistente en el disco duro del equipo,
también debe ser posible crear archivos de cualquier tipo y de cualquier tamaño.
(Mientras no se pase de 1000 x 1000 px)
Por último se debe implementar una estrategia para administrar la fragmentación del
sistema de archivos.

# Ambiente de desarrollo
* Ubuntu 22.04 LTS
* CLion 2022.2
* GitHub
* Rust versión 1.63.0

# Estructuras de datos usadas y funciones

###  fsck.bwfs (FileSystem Check)
En este caso se utilizaron las siguientes funciones:
* 
* 
* 
* 

### mount.bwfs  (FileSystem Mount)
En el mount se utilizaron las funciones:
* 
* 
* 


### mkfs.bwfs  (FileSystem Make)
Para la creación del FileSystem se usó:
###### <u>fileStructure:</u>  
* FileAttrDef (Struct): Este struct define los parametros o atributos que va a tener cada archivo o directorio.
* TimeSpecDef (Struct): Define los atributos de tiempo que va a tener un archivo o directorio.
* FileTypeDef (Enum): Define los tipos que va a tener un archivo o directorio.
###### <u>fsstructure:</u>  
* **Disk**
  * <u>Disk (Struct):</u> Estructura que define los atributos del disco en el que se va a guardar los datos del FS.
  * <u>new  (Implementación de Disk):</u>  Crea un nuevo Disk, crea un nuevo superbloque de memory blocks, asigna los tiempos iniciales y le da atributos.
  * <u>get_next_available_inode (Implementación de Disk)</u>: Obtiene el siguiente inode que este disponible en el Disk.
  * <u>write_inode  (Implementación de Disk):</u> Escribe el inode ingresado al superbloque de inodes.
  * <u>remove_inode  (Implementación de Disk):</u> Elimina el inode ingresado del superbloque de inodes.
  * <u>clear_reference  (Implementación de Disk):</u> Elimina la referencia a un bloque de memoria de un inode.
  * <u>add_reference  (Implementación de Disk):</u> Agrega la referencia a un bloque de memoria de un inode.
  * <u>get_inode  (Implementación de Disk):</u> Obtiene un inode por medio del Id, si no encuentra nada no devuelve nada.
  * <u>get_mut_inode  (Implementación de Disk):</u> Obtiene un inode mutable por medio del Id, si no encuentra nada no devuelve nada.
  * <u>find_inode_in_reference  (Implementación de Disk):</u>  Obtiene un Inode por medio del nombre, si no encuentra nada no devuelve nada.
  * <u>add_data_to_memory_block</u>  (Implementación de Disk): Agrega datos a un bloque de memoria asociado a un Inode buscado por medio del Id.
  * <u>delete_data_to_memory_block</u>  (Implementación de Disk): Borra datos a un bloque de memoria asociado a un Inode buscado por medio del Id.
  * <u>write_content  (Implementación de Disk):</u> Escribe los datos en el memory block asociado a un Inode por medio del Id.
  * <u>get_bytes_content  (Implementación de Disk):</u> Obtiene los datos en el memory block asociado a un inode por medio del Id.
* **filesystem_management**
  * <u>BWFS (Struct):</u> Define la estructura del FS, básicamente que disco va a usar.
  * <u>new  (Implementación de BWFS):</u> Crea un nuevo FS basado en un disco en especifico.
  * <u>get_disk  (Implementación de BWFS):</u> Obtiene el disco que se esta usando en el FS actualmente.
  * <u>set_disk  (Implementación de BWFS):</u> Setea el nuevo disco sobre el que se va a basar el FS.
  * <u>save_fs  (Implementación de BWFS):</u> Guarda el FileSystem en una imagen blanco y negro.
  * <u>drop  (Implementación de Drop para BWFS):</u> Apaga el FS guardandolo de manera persistente en imagenes.
  * <u>getattr  (Implementación de Filesystem para BWFS):</u> Obtiene los atributos de un archivo existente en el FS.
  * <u>create  (Implementación de Filesystem para BWFS):</u> Crea un archivo nuevo en el FS.
  * <u>open  (Implementación de Filesystem para BWFS):</u> Abre un archivo existente en el FS.
  * <u>read  (Implementación de Filesystem para BWFS):</u> Lee un archivo existente en el FS.
  * <u>write  (Implementación de Filesystem para BWFS):</u> Escribe sobre un archivo ya existente en el FS.
  * <u>rename  (Implementación de Filesystem para BWFS):</u> Renombra el archivo existente en el FS.
  * <u>mkdir (Implementación de Filesystem para BWFS):</u> Crea un directorio nuevo en el FS en el que se pueden guardar archivos.
  * <u>readdir (Implementación de Filesystem para BWFS):</u> Lee el directorio que se le pase como parámetro, este debe existir en el FS.
  * <u>opendir (Implementación de Filesystem para BWFS):</u> Abre un directorio existente en el FS.
  * <u>rmdir (Implementación de Filesystem para BWFS):</u> Elimina o remueve un directorio existente en el FS.
  * <u>statfs (Implementación de Filesystem para BWFS):</u> Muestra las estadísticas básicas del FS, como cantidad de inodos o bloques de memoria.
  * <u>fsync (Implementación de Filesystem para BWFS):</u> Sincroniza los contenidos de los archivos, si es diferente a 0 no borra los metadatos pero si los datos del usuario.
  * <u>access (Implementación de Filesystem para BWFS):</u> Revisa si puede acceder a un archivo ya existente en el FS.
  * <u>unlink (Implementación de Filesystem para BWFS): </u>Desvincula un archivo, ya sea vinculo normal o vínculo simbólico.
  * <u>flush (Implementación de Filesystem para BWFS):</u> Trata de eliminar o *flushear* los datos del caché.
  * <u>lseek (Implementación de Filesystem para BWFS):</u> Encuentra el primer hueco de datos en un offset especifico.
* **I-node**
  * <u>I-node (Struct):</u> Define los elementos que debe contener un I-node como lo son el nombre, sus atributos y referencias a bloques de memoria.
  * <u>add_reference  (Implementación de I-node):</u> Agrega una referencia a un bloque de memoria a un I-node.
  * <u>delete_reference  (Implementación de I-node):</u> Elimina una referencia a un bloque de memoria a un I-node.
  * <u>change_name  (Implementación de I-node):</u> Cambia el nombre de un I-node.
* **memory_block**
  * <u>MemoryBlock (Struct):</u> Define la estructura de un bloque de memoria, que tiene una referencia a su I-node padre y los datos.
  * <u>add_data  (Implementación de MemoryBlock):</u> Agrega datos a un bloque de memoria, aquí se realiza la verificación de que este no pase de 1000*1000 de tamaño.
  * <u>delete_data  (Implementación de MemoryBlock):</u>  Elimina todos los datos de un bloque de memoria.
* **save_disk**
  * <u>encode (Función):</u> Codifica un disco y lo serializa a binario.
  * <u>decode (Función):</u> De-codifica un disco y lo de-serializa de binario.
  * <u>write_pixels (Función):</u> Se encarga de escribir los datos serializados en binario de un disco a una imagen de blanco y negro.
  * <u>validate_path (Función):</u> Valida que la ruta de la imagen a escribir exista.
  * <u>load_fs (Función):</u> Carga un sistema de archivos usando la ruta de este.
  * <u>load_disk (Función):</u> Carga un disco usando la ruta de este.
# Instrucciones para ejecutar el programa
1. Se debe asegurar de tener los 3 binarios: 

	* fsck.bwfs
	
	* mount.bwfs
	
	* mkfs.bwfs
2. En caso de querer **revisar el estado de un FS** existente se debe: 
		2.1 Usar el binario fsck.bwfs de la siguiente manera por medio de la terminal: `./fsck.bwfs folderdelfs/`
3. En caso de querer **montar un FS** existente debe:
		3.1 Revisar que el FileSystem este en buenas condiciones, esto se puede realizar usando el binario fsck.bwfs de la siguiente manera por medio de la terminal: `./fsck.bwfs folderdelfs/`
		3.2 Después de confirmar el correcto estado del FS debe montarlo usando el binario mount.bwfs de la siguiente manera por medio de la terminal: `./mount.bwfs folderdelfs/ mountpoint/` con el parámetro mountpoint siendo una dirección en la que se quiera montar.
4. Por último en caso de querer **crear un nuevo FS** se debe:
		4.1 Crear un FS desde cero usando el binario mkfs.bwfs de la siguiente manera por medio de la terminal: `./mkfs.bwfs folderdelnuevofs/`
		4.2 Después se debe revisar que el FileSystem este en buenas condiciones, esto 	se puede realizar usando el binario fsck.bwfs de la siguiente manera por medio de la terminal: `./fsck.bwfs folderdelfs/`
		4.3 Después de confirmar el correcto estado del FS debe montarlo usando el binario mount.bwfs de la siguiente manera por medio de la terminal: `./mount.bwfs folderdelfs/ mountpoint/` con el parámetro mountpoint siendo una dirección en la que se quiera montar.
# Actividades realizadas por estudiantes
|Fecha|Inicio|Fin|Avance Realizado|Estudiante|
|---|---|---|---|---|
|a|a|a|a|a|
# GitLog
~~~
aaaa
~~~
# Autoevaluación
| Evaluación                            |Nota máxima obtenible|Nota Autoevaluada|
|---|---|---|
|Aprendizaje de mkfs|14%|-|
|Aprendizaje de fsck|5%|-|
|Aprendizaje de mount|10%|-|
|Funciones de la biblioteca|26%|-|
|Documentación|20%|20%|
|Persistencia en el Disco|25%|-|

| Autoevaluación                             | José Pablo | Pablo Muñoz |Luis Andrés|
|---|---|---|---|
|Aprendizaje de mkfs|5|5|5|
|Aprendizaje de fsck|5|5|5|
|Aprendizaje de mount|5|5|5|
|Aprendizaje de implementación de funciones|5|5|5|
|Aprendizaje de Diseño de FileSystem|5|5|5|


# Lecciones Aprendidas
En este proyecto se trató mucho el tema de como un SO administra los datos por medio de un FileSystem, se aprendió que es fundamental tener un buen FS para poder administrar los datos de manera eficiente, rápida y persistente, sin embargo no que sea fundamental no quiere decir que sea una tarea fácil, como grupo la pasamos especialmente difícil con este proyecto dado a que además de estar a final de semestre este lo obliga como ingeniero a pensar fuera de la caja.

Es un proyecto que enseña de manera espectacular como funciona el sistema de los i-nodos que (en este caso) fue el que escogimos para el proyecto, también es curioso ya que le enseña al programador y le brinda un mejor conocimiento sobre lo que está pasando por debajo de la interfaz gráfica que vemos todos los días.

En resumen, una conclusión muy buena a todos los temas vistos a  lo largo del curso ya que da una mejor perspectiva sobre el sistema operativo y como este esta estructurado a lo interno, además agradecemos el uso de Rust ya que demuestra que es un lenguaje perfecto para la programación a bajo nivel, es un lenguaje super completo y definitivamente es de las cosas que más rescatamos del curso.

# Bibliografía
- https://docs.rs/image/latest/image/
- https://blog.logrocket.com/decoding-encoding-images-rust-using-image-crate/?ref=morioh.com&utm_source=morioh.com
- https://morioh.com/p/a3e5136ef8db
