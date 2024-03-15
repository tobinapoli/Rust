#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::contrato::SemRustRef;
pub use self::contrato::Socio;
pub use self::contrato::Categoria;
#[ink::contract]
pub mod contrato {
#[derive(scale::Decode, scale::Encode,Debug,PartialEq,Clone,Copy)]
#[cfg_attr(
feature = "std",
derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]  
pub struct Fecha {
    dia: u64,
    mes: u8,
    año: u32,
}
impl <'a> Fecha {
    ///Devuelve la cantidad de dias correspondiente al mes
    fn tipo_mes(&self) -> Result<u8,&'a str> {
        match self.mes {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => Ok(31),
            4 | 6 | 9 | 11 => Ok(30),
            2 => if self.es_bisiesto() { Ok(29) } else { Ok(28) },
            _ => Err("mes invalido"),
        }
    }
    fn es_bisiesto(&self) -> bool {
        (self.año % 4 == 0) && (self.año % 100 != 0 || self.año % 400 == 0)
    }
    ///Suma una cantidad de dias recibida por parametros a la fecha actual
    fn sumar_dias(&mut self, cant: u64) { 
        self.dia += cant;
        while let Ok(tipo_mes) = self.tipo_mes() {
            if self.dia > tipo_mes as u64 {
                self.dia -= tipo_mes as u64;
                self.mes += 1;
                if self.mes > 12 {
                    self.mes = 1;
                    self.año += 1;
                }
            } else {
                break;
            }
        }
    }
    ///Devuelve true en caso de la fecha actual sea mayor a la recibida por parametro
    pub fn es_mayor(&self, otra_fecha: &Fecha) -> bool { 
        if self.año > otra_fecha.año {
            true
        } else if self.año < otra_fecha.año {
            false
        } else if self.mes > otra_fecha.mes {
            true
        } else if self.mes < otra_fecha.mes {
            false
        } else {
            self.dia > otra_fecha.dia
        }
    }
}


    use ink::prelude::string::String;
    use ink::prelude::string::ToString;
    use ink::prelude::vec::Vec;

    ///El struct SemRust es el que almacena toda la informacion acerca del club, teniendo asi
    ///un vec de socios para almacenar a cada socio, un vec de pagos para almacenar cada pago realizado,
    ///los precios de cada categoria(A,B,C) guardados en la variable precios. Ademas, la variable cuando_bonifica indica
    ///desde cuantos meses contando para atras el socio se considera como bonificado, por ejemplo si el valor es 3, si el socio pago los ultimos 3 meses
    ///se considerara bonificado. Asi mismo, la variable porcentaje_bonificacion como bien dice su nombre indicara que porcentaje sera el descuento que recibira
    ///el socio por ser bonificado. El owner sera el que haga el deploy del contrato, por lo que se almacenara en esta variable su AccountId, por otra parte,
    ///Los autorizados se almacenan en un vec, este vec indica que personas pueden realizar operaciones sobre el contrato. Para eso hay que tener en cuenta
    ///el flag, que dependiendo del valor que tenga, indica si hay que tener en cuenta a si la cuenta que quiere hacer una operacion esta autorizada(flag en true)
    ///o no(flag en false)
    #[ink(storage)]
    pub struct SemRust{
        socios: Vec<Socio>,
        pagos: Vec<Pago>,
        precios: PreciosCat,
        cuando_bonifica: u8,
        porcentaje_bonificacion: u32,
        owner: AccountId,
        autorizados: Vec<AccountId>,
        flag: bool,
    }
    #[derive(scale::Decode, scale::Encode,Debug,PartialEq,Clone,Copy)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
       ///El enum categoria almacena categoria A, donde cada socio puede acceder a todos los deportes del club y al gimnasio, categoria B, donde
       ///el socio puede acceder al gimnasio y a un deporte especifico, y categoria C donde el socio solo puede acceder al gimnasio.
    pub enum Categoria{
        A,
        B,
        C,
    }


    #[derive(scale::Decode, scale::Encode,Debug,PartialEq,Clone,Copy)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]

    
     ///El enum deportes guarda los deportes que posee el club SemRust, dependiendo de la categoria, el socio
    ///accedera a todos, a uno solo o a ninguno. El gimnasio es para todas las categorias.
    pub enum Deportes{
        Futbol,
        Basquet,
        Rugby,
        Hockey,
        Natacion,
        Tenis,
        Paddle,
    }
    #[derive(scale::Decode, scale::Encode,Debug,PartialEq,Clone,Copy)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]

    ///El struct socio guarda la informacion para representar a un socio del club. El mismo posee un dni,
    ///una categoria seleccionada del struct categoria, y un option de un deporte que, en caso de ser categoria A y C,
    ///el mismo sera un None(indicando el todos o el ninguno), y en caso de ser B guardara un solo deporte especifico como indica
    ///la categoria misma.
    pub struct Socio{
        dni: u32,
        cat: Categoria,
        dep: Option<Deportes>,
    }
    

    #[derive(scale::Decode, scale::Encode,Debug,PartialEq,Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]

    
        
    
    
        ///El struct precios categoria guarda los precios de cada categoria almacenados en formato u128, cuando se hace el deploy se 
        ///eligen estos precios por defecto:
        ///precio_a: 5000,
        ///precio_b: 3000,
        ///precio_c: 2000,
        ///Despues estos precios podran ser modificados.
    pub struct PreciosCat{
        precio_a: u128,
        precio_b: u128,
        precio_c: u128,
    }


    #[derive(scale::Decode, scale::Encode,Debug,PartialEq,Clone,Copy)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
   
     ///El struct Pago guarda la informacion propia de cada pago. El mismo almacena el dni del socio,
     ///si la persona esta bonificada, y si el pago se vencio(dependera de la fecha de vencimiento).
     ///Esta informacio es almacenada en el vec de pagos del struct SemRust
    pub struct Pago{
        id: u32,
        dni: u32,
        monto: u128,
        emision: Fecha,
        vencimiento: Fecha,
        pendiente: bool,
        bonificado: bool,
        vencido:bool,
    }
   
    


        ///Esta es la implementacion del struct SemRust, que sera la interfaz con la que podra interactuar el usuario al momento de hacer el deploy.
        ///teniendo asi un constructor, y metodos con el encabezado #[ink(message)] que indican que con ese metodo se puede interactuar, y metodos privados
    impl SemRust {
        /// Este es el constructor del contrato que sera invocado cuando el usuario haga el deploy, en ese momento el usuario debe seleccionar
        /// el valor de la variable cuando_bonifica, y el porcentaje de descuento para esa bonificacion.
        /// Lo que hace el constructor es setear el owner con el caller de ese momento, es decir, la AccountId que haga el deploy sera el owner,
        /// Ademas, se crea el vec de socios y de pagos vacio, se asignan los valores seleccionados para la bonificacion y el owner mismo con el vec
        /// de autorizados vacio y se setea el flag en false por defecto. Este flag luego podra ser modificado.
        /// El metodo devuelve una variable del mismo struct SemRust.
        #[ink(constructor)]
        pub fn new(cuando_bonifica: u8, porcentaje_bonificacion: u32) -> Self{
            let owner = Self::env().caller();
            Self{
                socios: Vec::new(),
                pagos: Vec::new(),
                precios: PreciosCat {
                    precio_a: 5000,
                    precio_b: 3000,
                    precio_c: 2000,
                },
                cuando_bonifica,
                porcentaje_bonificacion,
                owner,
                autorizados: Vec::new(),
                flag: false,
            }
        } 


        /// La creacion del socio llama a un metodo privado que hace la funcion de creacion, este metodo devuelve un result con un string por si arroja error.
        /// Para este metodo se hace el llamado con la variable del struct SemRust creada previamente pasada
        /// como una referencia mutable(puesto que se va a modificar), el dni del socio a crear, la categoria seleccionada y el deporte dependiendo
        /// de la categoria elegida
        #[ink(message)]
        pub fn nuevo_socio(&mut self, dni: u32, cat: Categoria, dep: Option<Deportes>)->Result<(),String>{
            return self.creacion_socio(dni, cat, dep);
        } 

        ///La creacion del socio es un metodo privado que hace la creacion, este metodo devuelve un result por si arroja un error al momento de crearse el socio.
        ///lo primero que se hace es chequear si el usuario que esta haciendo el llamado esta autorizado o no, en caso de estarlo, se crea un socio con los datos
        ///proporcionados y se llama al metodo crear_pago con el dni y la categoria, este metodo le creara un pago al socio correspondiente con fecha de vencimiento
        ///para los proximos 10 dias. Luego de eso se agrega al socio al vec de socios del struct SemRust.
        ///En caso de no estar autorizado, este metodo devuelve el error "No estas autorizado".
        // En caso de que el socio este repetido, se devuelve el error "Ya existe un socio con ese dni"
        fn creacion_socio(&mut self, dni: u32, cat: Categoria, dep: Option<Deportes>)->Result<(),String>{
            if self.esta_autorizado(){
                if self.socios.iter().any(|socio| socio.dni == dni) {
                    return Err("El DNI ya esta registrado".to_string());
                }
                else{
                    let s = Socio {dni, cat, dep};
                    self.crear_pago(dni, cat);
                    self.socios.push(s);
                    Ok(())
                }
            }else{
                Err("No estás autorizado".to_string())
            }
        }

        
            ///El metodo es owner recibe un AccountId y devuelve un booleano indicando si esa id coincide con el owner del contrato.
        fn es_owner(&self, id: AccountId) -> bool{
            return self.owner.eq(&id);
        }


        /// El metodo esta autorizado es el que hace los chequeos para verificar que la persona que hace el llamado puede realizar operaciones sobre el contrato.
        /// Para ello primero se toma el id de la persona que esta haciendo el llamado y se devuelve un or entre 3 condiciones. Si el flag esta en falso, quiere decir
        /// que cualquiera puede operar, si el id es el owner(llamando al metodo es_owner), tambien puede operar, y si la id existe en el vec de autorizados, tambien puede operar.
        /// En cualquiera de esos 3 casos se devuelve true, caso contrario se devuelve false.
        #[ink(message)]
        pub fn esta_autorizado (&self)->bool{
            return self.esta_autorizado_priv();
        }

        fn esta_autorizado_priv(&self) -> bool{
            let id = self.env().caller();
            if  !self.flag || self.es_owner(id) || self.autorizados.iter().any(|a| a.eq(&id)) {
                return true
            }else  {
                return false
            }
        }
        /// El metodo set_owner recibe un AccountId y llama a un metodo privado que hace el seteo, este metodo devuelve un result con un error en caso
        /// de no ser posible realizar el seteo.
        #[ink(message)]
        pub fn set_owner(&mut self, id: AccountId) -> Result<(), String>{
            return self.set_owner_priv(id);
        }

            ///Este metodo realiza el cambio de owner. Lo primero que hace es verificar que el owner sea el que esta haciendo el llamado
            ///puesto que solo este mismo puede setear un nuevo owner, en caso de que cumpla con esta condicion se hace el cambio por la id
            ///recibida como parametro. Caso contrario se devuelve el error "No sos el owner"
        fn set_owner_priv(&mut self, id: AccountId) -> Result<(), String>{
            if self.es_owner(self.env().caller()){
                self.owner = id;
                Ok(())
            }else{
                return Err("No sos el owner".to_string());
            }
            
        }

        ///El metodo autorizar se usa para que el owner pueda agregar nuevas AccountId al vec de autorizados.  
        #[ink(message)]
        pub fn autorizar(&mut self, id: AccountId) -> Result<(),String>{
            self.autorizar_priv(id)
        }
        
           /// Este metodo hace la autorizacion en si. Lo que hace es chequear que el caller sea el owner, despues verifica que esa id no haya sido agregada
            ///previamente al vec para evitar repetidos. En caso de que este repetida arroja el error "Ya estas autorizado". En caso contrario se agrega 
            ///el id al vec de autorizados. En el caso de que el que haga el llamado no sea el owner se devuelve el error "No sos el owner"
        fn autorizar_priv(&mut self, id: AccountId) -> Result<(),String>{
            if self.es_owner(self.env().caller()) {
                if self.autorizados.iter().any(|a| a.eq(&id)){
                    return Err("Ya estas autorizado".to_string())
                }
                else{
                    self.autorizados.push(id);
                }
            }else {
                return Err("No sos el owner".to_string());
            }

            return Ok(());

        }
        #[ink(message)]
        ///El metodo desautorizar se usa para que el owner pueda eliminar AccountId existentes del vec de autorizados. 
        pub fn desautorizar(&mut self, id: AccountId) -> Result<(),String>{
            self.desautorizar_priv(id)
        }
        ///Este metodo por el contrario de la autorizacion hace la desautorizacion. Lo que hace es chequear que el caller sea el owner, despues busca la posicion exacta
        ///En caso de que este repetida arroja el error "Ya estas autorizado". En caso contrario se agrega 
        ///el id al vec de autorizados. En el caso de que el que haga el llamado no sea el owner se devuelve el error "No sos el owner"
        fn desautorizar_priv(&mut self, id: AccountId)-> Result<(),String>{
            if self.es_owner(self.env().caller()){
                if let Some(index) = self.autorizados.iter().position(|p| p.eq(&id)){
                    self.autorizados.remove(index);
                    Ok(())
                }
                else{
                    return Err("No se encontro ninguna id".to_string());
                }
            }else{
                return Err("No sos el owner".to_string());
            }                            
                              
        }
        
        /// Metodo que setea el flag al valor contrario, es decir, si el flag se encuentra en true cambiara a false y si se encuentra en false cambiara a true.
        #[ink(message)]
        pub fn set_flag(&mut self)-> Result<(), String>{
            return self.set_flag_priv();
        }

        fn set_flag_priv(&mut self) -> Result<(), String>{
            if self.es_owner(self.env().caller()){
                self.flag = !self.flag;
                Ok(())
            }
            else{
                return Err("No sos el owner".to_string())
            }

        }
        /// Metodo que devuelve el valor del flag en el momento
        #[ink(message)]
        pub fn get_flag(&self) -> bool{
            return self.get_flag_priv();
        } 

        fn get_flag_priv(&self) -> bool {
            return self.flag;
        }
        

        ///Metodo que crea un pago recibiendo el dni y la categoria correspondiende,
        /// Ademas se crea la fecha de vencimiento para los proximos 10 dias y dependiendo
        /// de si el socio es bonificado o no se le hace el descuento correspondiente
        fn crear_pago(&mut self, dni: u32, cat:Categoria){
            let fech = self.conversor();
            let monto =  match cat {
                Categoria::A  => self.precios.precio_a,
                Categoria::B  => self.precios.precio_b,
                Categoria::C  => self.precios.precio_c,
            };
            let mut v = fech;
            v.sumar_dias(10);
            let id = (self.pagos.len()+1) as u32;
            let mut p = Pago{
                id,
                monto, 
                dni: dni,
                emision: fech,
                vencimiento: v,
                pendiente: true,
                bonificado: self.es_bonificado(dni).unwrap(),
                vencido: false,
            };
            if p.bonificado{
                p.monto -= p.monto * (self.porcentaje_bonificacion as u128 / 100);
            }
            self.pagos.push(p); 
        }
        ///A partir del timestamp devuelve una fecha en formato dia/mes/año
        #[ink(message)]
        pub fn conversor(&self)->Fecha{ 
            let timestamp = self.env().block_timestamp();
            let seconds = timestamp / 1000;
            let minutes = seconds / 60;
            let hours = minutes / 60;
            let days: u64 = hours / 24;
            
            let mut f = Fecha{
                dia: 1,
                mes: 1,
                año: 1970,
            };
            f.sumar_dias(days);
            return f;
        }

        /// Bonificado
        fn es_bonificado(&self, dni: u32)->Result<bool, String>{ //Verifica si se cumple la cantidad de pagos sucesivos en fechas, para otorgar una bonificación
            let vec = self.obtener_pagos_por_dni(dni);
            if vec.len() < self.cuando_bonifica as usize {
                return Ok(false);
            }
            let ultimas = &vec[vec.len() - self.cuando_bonifica as usize..];
            for valor in ultimas {
                if valor.vencido {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        ///Metodo que obtiene los pagos pendientes de un dni especifico
        #[ink(message)]
        pub fn obtener_pagos_pendientes_por_dni(&self, dni_buscar: u32) -> Result<Vec<Pago>, String> { //Devuelve el vector de los pagos a partir de un DNI recibido por parametro
            return self.obtener_pagos_pendientes_por_dni_priv(dni_buscar);
            
        }
        fn obtener_pagos_pendientes_por_dni_priv(&self, dni_buscar: u32) -> Result<Vec<Pago>, String>{
            if self.esta_autorizado(){
                let pagos_dni: Vec<Pago> = self.obtener_pagos_por_dni(dni_buscar);
                let mut pagos_encontrados= Vec::new();
                for pago in &pagos_dni {
                    if pago.pendiente {
                        pagos_encontrados.push(pago.clone());
                    }
                }
                Ok(pagos_encontrados)
            }
            else{
                Err("No estás autorizado".to_string())
            }
        }
        
        /// La función obtener_pagos_por_dni busca y devuelve todos los pagos asociados a un número de identificación (DNI) específico.
        /// Recorre una lista de pagos y agrega aquellos cuyo DNI coincide con el valor proporcionado. Retorna un vector con los pagos encontrados.
        fn obtener_pagos_por_dni(&self, dni_buscar: u32) ->Vec<Pago>{
            let mut pagos_encontrados: Vec<Pago> = Vec::new();
                for pago in &self.pagos {
                    if pago.dni == dni_buscar {
                        pagos_encontrados.push(pago.clone());
                    }
                }
            pagos_encontrados
        }

        #[ink(message)]
        pub fn obtener_pagos_realizados(&self)->Result<Vec<Pago>,String>{ 
            return self.obtener_pagos_realizados_priv();
        }

        fn obtener_pagos_realizados_priv(&self) ->Result<Vec<Pago>,String>{
            if self.esta_autorizado(){
                let mut pagos_encontrados: Vec<Pago> = Vec::new();
                    for pago in &self.pagos {
                        if !pago.pendiente {
                            pagos_encontrados.push(pago.clone());
                        }
                    }
                Ok(pagos_encontrados)
            }else{
                Err("No estás autorizado".to_string())
            }
        }
        

        ///Este metodo permite registrar un pago de un socio especifico indicando su dni y el monto adecuado
        #[ink(message)]
        pub fn registrar_pago(&mut self, dni: u32, monto: u128)->Result<(),String>{
            self.registrar_pago_priv(dni, monto)
        }
                
        fn registrar_pago_priv(&mut self, dni: u32, monto: u128)->Result<(),String>{
            if self.esta_autorizado(){      
                if let Some(pos) = self.pagos.iter().position(|pago| pago.dni == dni && pago.pendiente){
                let mut pago = self.pagos[pos];
                let f = self.conversor();
                if pago.monto == monto {
                    if f.es_mayor(&pago.vencimiento){
                        pago.vencido=true;
                    }
                    pago.pendiente=false;
                    self.pagos[pos] = pago;
                    Ok(())
                } else {
                    Err("Monto equivocado".to_string())
                }
            } else {
                Err("No hay pagos pendientes".to_string())
            }
          } else {
            return Err("No estas autorizado".to_string())
          }
     }
        

        ///El método público generar_pago_mensual genera pagos mensuales para los socios del sistema si el usuario está autorizado. 
        /// Utiliza el método privado generar_pago_mensual_priv que implementa la lógica para generar los pagos.
        ///  El método itera sobre la lista de socios, obtiene el último pago de cada socio y compara la fecha de emisión con la fecha actual.
        ///  Si la fecha actual es mayor o igual en mes y año, se crea un nuevo pago mensual para el socio. Al finalizar, 
        /// se retorna un resultado Ok(()) si la generación se realizó correctamente, o se emite un mensaje de error Err("No estas autorizado") 
        /// si el usuario no tiene autorización. En resumen, el método permite generar pagos mensuales para los socios autorizados del sistema, basándose
        ///  en la comparación de fechas y la creación de pagos correspondientes.
        #[ink(message)]
        pub fn generar_pago_mensual(&mut self)->Result<(), String>{
            return self.generar_pago_mensual_priv();
        }
        fn generar_pago_mensual_priv(&mut self)->Result<(), String>{
            if self.esta_autorizado(){
                let hoy = self.conversor();
                for numero in 0..self.socios.len(){
                    let vec = self.obtener_pagos_por_dni(self.socios[numero].dni);
                    let pago = vec[vec.len() - 1];
                    let fecha = pago.emision;
                    if (hoy.mes == fecha.mes && hoy.año > fecha.año) || (hoy.mes > fecha.mes && hoy.año >= fecha.año){
                        self.crear_pago(self.socios[numero].dni, self.socios[numero].cat);
                    }               
                }
                Ok(())
            }
            else{
                return Err("No estas autorizado".to_string())
            }
        }


        ///Los métodos públicos actualizar_precio_a, actualizar_precio_b y actualizar_precio_c permiten actualizar los precios correspondientes
        ///  (precio_a, precio_b y precio_c) en la estructura precios. Cada método recibe un nuevo precio como parámetro
        ///  y devuelve un resultado Result<(), String> indicando si la actualización se realizó con éxito o si hubo un error.
    
        #[ink(message)]
        pub fn actualizar_precio_a(&mut self, precio_a: u128)-> Result<(), String>{
            return self.actualizar_precio_a_priv(precio_a);
        }
        fn actualizar_precio_a_priv(&mut self, precio_a: u128)-> Result<(), String>{
            if self.esta_autorizado(){
                self.precios.precio_a = precio_a;
                Ok(())
            }else{
                Err("No estas autorizado".to_string())
            }
        }
        #[ink(message)]
        pub fn actualizar_precio_b(&mut self, precio_b: u128) -> Result<(), String> {
            return self.actualizar_precio_b_priv(precio_b);
        }
        pub fn actualizar_precio_b_priv(&mut self, precio_b: u128) -> Result<(), String> {
            if self.esta_autorizado(){
                self.precios.precio_b = precio_b;
                Ok(())
            }
            else{
                return Err("No estas autorizado".to_string());

            }
        }
        #[ink(message)]
        pub fn actualizar_precio_c(&mut self, precio_c: u128)-> Result<(), String> {
            return self.actualizar_precio_c_priv(precio_c);
        }
        
        fn actualizar_precio_c_priv(&mut self, precio_c: u128)-> Result<(), String> {
            if self.esta_autorizado(){
                self.precios.precio_c = precio_c;
                Ok(())
            }
            else{
                return Err("No estas autorizado".to_string());

            }
        }
        
        ///El método público get_socio recibe un número de identificación (dni) y devuelve una opción (Option) que puede contener un objeto Socio. 
        /// Utiliza el método privado get_socio_priv para realizar el procesamiento.
        ///  El método privado itera sobre la lista de socios y busca aquel cuyo número de identificación (dni) coincida con el proporcionado. 
        /// Si se encuentra un socio correspondiente, se devuelve una opción que contiene una copia del objeto Socio. 
        /// En caso contrario, se retorna None, indicando que no se encontró ningún socio con el número de identificación proporcionado.
        #[ink(message)]
        pub fn get_socio(&mut self, dni: u32) -> Option<Socio>{
            return self.get_socio_priv(dni);
        }
        fn get_socio_priv(&self, dni: u32) -> Option<Socio> {
            self.socios.iter().find(|s| s.dni == dni).cloned()
        }

        ///El método público get_pago recibe un número de identificación (dni) y devuelve una opción (Option) que puede contener un objeto Pago. 
        /// Utiliza el método privado get_pago_priv para realizar el procesamiento. 
        /// El método privado busca en la lista de pagos aquel cuyo número de identificación (dni) coincida con el proporcionado. 
        /// Si se encuentra un pago correspondiente, se devuelve una opción que contiene el objeto Pago. 
        /// En caso contrario, se retorna None, indicando que no se encontró ningún pago con el número de identificación proporcionado.
        #[ink(message)]
        pub fn get_pago(&self, dni: u32) -> Option<Pago>{
            self.get_pago_priv(dni)
        }     
        fn get_pago_priv(&self, dni:u32) -> Option<Pago>{
            if let Some(res) = self.pagos.iter().find(|pago| pago.dni == dni){
                return Some(*res);
            } else {
                return None;
            }
       }    

       ///El método público get_dni devuelve el número de identificación (dni) de un objeto Socio si el usuario está autorizado. 
       /// Utiliza el método privado get_dni_priv para realizar el procesamiento. 
       /// El método privado verifica la autorización y, si el usuario está autorizado, devuelve el número de identificación (dni) del objeto Socio.
       ///  Si el usuario no está autorizado, se emite un mensaje de error indicando que no está autorizado.

       #[ink(message)]
       pub fn get_dni(&self, pj: Socio)->Result<u32, String>{
            return self.get_dni_priv(pj);
       }

       fn get_dni_priv(&self,pj: Socio) -> Result<u32, String>{
            if self.esta_autorizado(){
                return Ok(pj.dni);
            } else {
                return Err("No esta Autorizado".to_string());
            }
        } 

        ///El método público get_socios devuelve una lista de objetos Socio si el usuario está autorizado, utilizando el método privado get_socios_priv. 
        /// El método privado verifica la autorización y, si el usuario está autorizado, crea un nuevo vector y recorre la lista de socios, 
        /// agregando cada socio al vector. Al finalizar, se retorna el vector de socios si el usuario está autorizado. En caso contrario, 
        /// se emite un mensaje de error indicando que no está autorizado.


       #[ink(message)]
        pub fn get_socios(&self) -> Result<Vec<Socio>,String> {
            return self.get_socios_priv();
        }

        fn get_socios_priv(&self) -> Result<Vec<Socio>,String> {
            if self.esta_autorizado(){
                let mut vec = Vec::new();
                for socio in &self.socios {
                    vec.push(*socio);
                }
                return Ok(vec);
            } else {
                return Err("No esta Autorizado".to_string());
            }
        }
        
        ///El método público get_pagos_pagados_mes devuelve una lista de IDs de pagos que han sido pagados durante el mes y año actual. 
        /// Este método verifica la autorización del usuario y, en caso de estar autorizado, realiza el procesamiento llamando al método privado get_pagos_pagados_mes_priv. 
        /// Dentro de este último método, se recorre la lista de pagos para identificar aquellos que han sido pagados en el mes y año actual, 
        /// y se construye una lista de IDs correspondientes. Al finalizar, se retorna la lista de IDs si hay pagos registrados,
        ///  o un mensaje de error en caso de falta de autorización o si no se encontraron pagos correspondientes.
        
        #[ink(message)]
        pub fn get_pagos_pagados_mes(&self) -> Result<Vec<u32>, String>{
            return self.get_pagos_pagados_mes_priv();
        }

       fn get_pagos_pagados_mes_priv(&self) -> Result<Vec<u32>, String>{
            if self.esta_autorizado(){
                let hoy = self.conversor();
                let mut vec = Vec::new();
                for pago in &self.pagos{
                    if !pago.pendiente{
                        let fecha = pago.emision;
                        if hoy.mes == fecha.mes && hoy.año == fecha.año {
                            vec.push(pago.id);
                        }
                    }
                }
                return Ok(vec);   
            } else {
                return Err("No esta Autorizado".to_string());
            }
       }


        /// En la implementacion de los precios_cat, se implementan los getters de cada precio de categoria, para poder operar en distintos metodos del contrato
        #[ink(message)]
        pub fn get_precio_a(&self) -> u128{
            return self.get_precio_a_priv();
        }
        #[ink(message)]
        pub fn get_precio_b(&self) -> u128{
            return self.get_precio_b_priv();
        }
        #[ink(message)]
        pub fn get_precio_c(&self) -> u128{
            return self.get_precio_c_priv();
        }

        fn get_precio_a_priv(&self) -> u128{
            return self.precios.precio_a;
        }

        fn get_precio_b_priv(&self) -> u128{
            return self.precios.precio_b;
        }

        fn get_precio_c_priv(&self) -> u128{
            return self.precios.precio_c;
        }
    /// Este metodo recibe un id de un pago y devuelve el monto del mismo
        #[ink(message)]
        pub fn get_monto_pago(&self, id: u32) -> Result<u128,String>{
            return self.get_monto_pago_priv(id);
        }

        fn get_monto_pago_priv(&self, id: u32) -> Result<u128,String>{
            if self.esta_autorizado(){
                return Ok(self.pagos[(id -1) as usize].monto);
            } else {
                return Err("No esta Autorizado".to_string());
            }
        }
        /// Este metodo recibe un pago y devuelve la fecha de vencimiento del mismo
        #[ink(message)]
        pub fn get_vencimiento_pago(&self, pago: Pago) -> Result<Fecha,String>{
            return self.get_vencimiento_pago_priv(pago);
        }
        fn get_vencimiento_pago_priv(&self, pago: Pago) -> Result<Fecha,String>{
            if self.esta_autorizado(){
                return Ok(pago.vencimiento);
            } else {
                return Err("No esta Autorizado".to_string());
            }
        }
    }


    #[cfg(test)]
    mod tests {
    use super::*;
 
    #[test]
    fn test_tipo_mes() {
        let f1 = Fecha{
            dia: 1,
            mes: 1,
            año: 2023,
        };
        assert_eq!(f1.tipo_mes().unwrap(), 31);
        let f2 = Fecha{
            dia: 1,
            mes: 2,
            año: 2023,
        };
        assert_eq!(f2.tipo_mes().unwrap(), 28);
        let f3 = Fecha{
            dia: 1,
            mes: 2,
            año: 2024,
        };
        assert_eq!(f3.tipo_mes().unwrap(), 29);
        let f4 = Fecha{
            dia: 1,
            mes: 4,
            año: 2023,
        };
        assert_eq!(f4.tipo_mes().unwrap(), 30);
    }
        #[test]
        fn test_es_bisiesto() {
            let fecha = Fecha{dia: 1, mes: 1, año: 2023};
            assert_eq!(fecha.es_bisiesto(), false);
            let fecha = Fecha{dia: 1, mes: 1, año: 2024};
            assert_eq!(fecha.es_bisiesto(), true);
            let fecha = Fecha{dia: 1, mes: 1, año: 2100};
            assert_eq!(fecha.es_bisiesto(), false);
            let fecha = Fecha{dia: 1, mes: 1, año: 2000};
            assert_eq!(fecha.es_bisiesto(), true);
        }
        #[test]
        fn test_sumar_dias() {
            let mut fecha = Fecha{
                dia: 30,
                mes: 12,
                año: 2023,
            };
            fecha.sumar_dias(2);
            assert_eq!(fecha.dia, 1);
            assert_eq!(fecha.mes, 1);
            assert_eq!(fecha.año, 2024);
            let mut fecha = Fecha{
                dia: 31,
                mes: 1,
                año: 2023,
            };
            fecha.sumar_dias(1);
            assert_eq!(fecha.dia, 1);
            assert_eq!(fecha.mes, 2);
            assert_eq!(fecha.año, 2023);
        }
        
        
        #[test]
        fn test_es_mayor() {
            let fecha1 = Fecha{dia: 1, mes: 1, año: 2023};
            let fecha2 = Fecha{dia: 2, mes: 1, año: 2023};
            assert_eq!(fecha1.es_mayor(&fecha2), false);
            let fecha1 = Fecha{dia: 2, mes: 1, año: 2023};
            let fecha2 = Fecha{dia: 1, mes: 1, año: 2023};
            assert_eq!(fecha1.es_mayor(&fecha2), true);
            let fecha1 = Fecha{dia: 1, mes: 2, año: 2023};
            let fecha2 = Fecha{dia: 1, mes: 1, año: 2023};
            assert_eq!(fecha1.es_mayor(&fecha2), true);
            let fecha1 = Fecha{dia: 1, mes: 1, año: 2023};
            let fecha2 = Fecha{dia: 1, mes: 2, año: 2023};
            assert_eq!(fecha1.es_mayor(&fecha2), false);
            let fecha1 = Fecha{dia: 1, mes: 1, año: 2023};
            let fecha2 = Fecha{dia: 1, mes: 1, año: 2023};
            assert_eq!(fecha1.es_mayor(&fecha2), false);
        }


        #[test]
        fn my_test() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut club: SemRust = SemRust::new(1,20);
            club.creacion_socio(1, Categoria::A, Some(Deportes::Futbol));
            assert_eq!(club.socios.len(), 1); 
        }

        
        
        #[test]
        fn es_bonificado_test(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut club: SemRust = SemRust::new(1,20);
            club.creacion_socio(1,Categoria::A, None);
            club.registrar_pago(1, 5000);
            club.crear_pago(1, Categoria::A);
            club.registrar_pago(1, 5000);
            assert_eq!(club.es_bonificado(1).unwrap(),true);
        }
         
        #[test]
        fn no_es_bonificado_test(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut club: SemRust = SemRust::new(3,20);
            club.creacion_socio(1,Categoria::A, None);
            club.crear_pago(1, Categoria::A);
            club.registrar_pago(1,5000);
            club.registrar_pago(1,5000);
            assert_eq!(club.es_bonificado(1).unwrap(), false);

        }


        
        #[test]
        fn conversor_test(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut club: SemRust = SemRust::new(1,20);
            let f = Fecha{dia: 1, mes: 1, año: 1970}; 

            club.creacion_socio(1, Categoria::A, Some(Deportes::Futbol));
            assert_eq!(club.pagos.get(0).unwrap().emision, club.conversor());
            assert_eq!(club.pagos.get(0).unwrap().emision, f);
        }       



        #[test]
        fn get_pago_test(){     
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut club: SemRust = SemRust::new(20,10);         

            assert_eq!(club.get_pago(1), None);

            club.creacion_socio(1, Categoria::A, None);         


            let mut hoymas10 = club.conversor();
            hoymas10.sumar_dias(10);
            let p = Pago{id : 1, dni: 1, monto: club.precios.precio_a, emision: club.conversor(), vencimiento: hoymas10, pendiente: true, bonificado: false, vencido: false};
            assert_eq!(club.get_pago(1), Some(p));     
        }
        #[test]
        fn test_obtener_pagos_por_dni() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);


            let mut contrato = SemRust::new(3, 10); 
            
            contrato.nuevo_socio(1, Categoria::A, None);
            
            let pagos_existente = contrato.obtener_pagos_por_dni(1);
            assert_eq!(pagos_existente.len(),1);
            
            let pagos_inexistente = contrato.obtener_pagos_por_dni(2);
            assert_eq!(pagos_inexistente.len(),0);
        }

        
        #[test]
        fn crear_pago_categoria_a_test(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut club: SemRust = SemRust::new(1, 1);
            let dni: u32 = 123456; let categoria: Categoria = Categoria::A;
            club.crear_pago(dni, categoria);
            assert_eq!(club.pagos.len(), 1);
            assert_eq!(club.pagos[0].dni, dni);
            assert_eq!(club.pagos[0].monto, club.precios.precio_a);
        }

        
        #[test]
        fn crear_pago_categoria_b_test(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut club: SemRust = SemRust::new(1, 1);
            let dni: u32 = 123456; let categoria: Categoria = Categoria::B;
            club.crear_pago(dni, categoria);
            assert_eq!(club.pagos.len(), 1);
            assert_eq!(club.pagos[0].dni, dni);
            assert_eq!(club.pagos[0].monto, club.precios.precio_b);
        }
         
        #[test]
        fn crear_pago_categoria_c_test(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut club: SemRust = SemRust::new(1, 1);
            let dni: u32 = 123456; let categoria: Categoria = Categoria::C;
            club.crear_pago(dni, categoria);
            assert_eq!(club.pagos.len(), 1);
            assert_eq!(club.pagos[0].dni, dni);
            assert_eq!(club.pagos[0].monto, club.precios.precio_c);
        }


        
        #[test]
        fn registrar_pago_test_ok_a(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut club: SemRust = SemRust::new(1, 1);
            let dni: u32 = 123456; let categoria: Categoria = Categoria::A;
            club.crear_pago(dni, categoria);
            assert_eq!(club.registrar_pago(dni, club.precios.precio_a), Ok(()));
        }

        #[test]
        fn registrar_pago_test_ok_b(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut club: SemRust = SemRust::new(1, 1);
            let dni: u32 = 123456; let categoria: Categoria = Categoria::B;
            club.crear_pago(dni, categoria);
            assert_eq!(club.registrar_pago(dni, club.precios.precio_b), Ok(()));
        }

 
        #[test]
        fn registrar_pago_test_ok_c(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut club: SemRust = SemRust::new(1, 1);
            let dni: u32 = 123456; let categoria: Categoria = Categoria::C;
            club.crear_pago(dni, categoria);
            assert_eq!(club.registrar_pago(dni, club.precios.precio_c), Ok(()));
        }


        
        #[test]
        fn registrar_pago_test_ok_vencido(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut club: SemRust = SemRust::new(1, 1);
            let dni: u32 = 123456; let categoria: Categoria = Categoria::A;
            let f = Fecha{
                dia: 1,
                mes: 1,
                año: 2010,
            };
            club.crear_pago(dni, categoria);
            club.pagos[0].vencimiento = f;
            assert_eq!(club.registrar_pago(dni, club.precios.precio_a), Ok(()));
        }
        
         ///Testeo del registro de un pago erroneo para la categoria A.
         ///Se crea un pago de categoria A pero el monto ingresado
         ///para el registro es diferente al que posee el club.
        

        #[test]
        fn registrar_pago_test_error_monto_a(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut club: SemRust = SemRust::new(1, 1);
            let dni: u32 = 123456; let categoria: Categoria = Categoria::A;
            club.crear_pago(dni, categoria);
            assert!(club.registrar_pago(dni, club.precios.precio_b).is_err());
        }

         

        
        #[test]
        fn registrar_pago_test_error_monto_b(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut club: SemRust = SemRust::new(1, 1);
            let dni: u32 = 123456; let categoria: Categoria = Categoria::B;
            club.crear_pago(dni, categoria);
            assert!(club.registrar_pago(dni, club.precios.precio_a).is_err());
        }
        
         

        #[test]
        fn registrar_pago_test_error_monto_c(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut club: SemRust = SemRust::new(1, 1);
            let dni: u32 = 123456; let categoria: Categoria = Categoria::B;
            club.crear_pago(dni, categoria);
            assert!(club.registrar_pago(dni, club.precios.precio_a).is_err());
        }
         

        #[test]
        fn registrar_pago_test_error_no_hay_pendientes(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut club: SemRust = SemRust::new(1, 1);
            let dni: u32 = 123456;
            assert!(club.registrar_pago(dni, club.precios.precio_a).is_err());
        }
         

      
        #[test]
        fn obtener_socio_existe(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut club: SemRust = SemRust::new(1, 1);
            let dni: u32 = 123456;
            club.nuevo_socio(dni, Categoria::A, None);
            assert!(club.get_socio(dni).is_some());
        }
        
        
        
        #[test]
        fn obtener_socio_no_existe(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut club: SemRust = SemRust::new(1, 1);
            let dni: u32 = 123456;
            club.nuevo_socio(dni, Categoria::A, None);
            assert!(club.get_socio(45123).is_none());
        }

        #[test]
        fn generar_pago_mensual_test(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut club: SemRust = SemRust::new(1, 1);
            let dni: u32 = 123456; let categoria: Categoria = Categoria::B;

            assert_eq!(club.pagos.len(), 0);
            
            club.creacion_socio(dni, categoria, None);

            assert_eq!(club.pagos.len(), 1);

            club.pagos[0].emision.año = 1; 
            club.generar_pago_mensual();
            club.generar_pago_mensual();
            
            assert_eq!(club.pagos.len(), 2);  
        }

        #[test]
        fn test_obtener_pagos_pendiente_por_dni_no() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut contrato = SemRust::new(3, 10); 
            contrato.nuevo_socio(1, Categoria::A, None);
            contrato.registrar_pago(1, 5000);
            let pagos_existente_pendiente = contrato.obtener_pagos_pendientes_por_dni(1).unwrap();
            assert_eq!(pagos_existente_pendiente.len(), 0);
        }

        #[test]
        fn test_obtener_pagos_pendiente_por_dni() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut contrato = SemRust::new(3, 10); 
            contrato.nuevo_socio(1, Categoria::A, None);
            let pagos_existente_pendiente = contrato.obtener_pagos_pendientes_por_dni(1).unwrap();
            assert_eq!(pagos_existente_pendiente.len(), 1);
        }
        #[test]
        fn actualizar_precio_a() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut contrato = SemRust::new(3, 10); 
            assert!(contrato.actualizar_precio_a(4500).is_ok());

        }
        #[test]
        fn actualizar_precio_a_error() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut contrato = SemRust::new(3, 10); 
            contrato.set_flag();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert!(contrato.actualizar_precio_a(4500).is_err());
            
        }
        #[test]
        fn actualizar_precio_b() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut contrato = SemRust::new(3, 10); 
            assert!(contrato.actualizar_precio_b(3500).is_ok());

        }
        #[test]
        fn actualizar_precio_b_error() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut contrato = SemRust::new(3, 10); 
            contrato.set_flag();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert!(contrato.actualizar_precio_b(3500).is_err());
            
        }
        #[test]
        fn actualizar_precio_c() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut contrato = SemRust::new(3, 10); 
            assert!(contrato.actualizar_precio_c(2500).is_ok());

        }
        #[test]
        fn actualizar_precio_c_error() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut contrato = SemRust::new(3, 10); 
            contrato.set_flag();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert!(contrato.actualizar_precio_c(2500).is_err());
            
        }
        #[test]
        fn test_get_dni(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            let mut contrato = SemRust::new(3, 10); 
            let socio = Socio{
                dni: 123,
                cat: Categoria::A,
                dep: None,
            };
            assert!(contrato.get_dni(socio).is_ok());
        }
        #[test]
        fn test_get_dni_error(){
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            let mut contrato = SemRust::new(3, 10); 
            let socio = Socio{
                dni: 123,
                cat: Categoria::A,
                dep: None,
            };
            contrato.set_flag();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert!(contrato.get_dni(socio).is_err());
        }
        #[test]
        fn test_get_socios(){
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            let mut contrato = SemRust::new(3, 10); 
            
            assert!(contrato.get_socios().is_ok());
        }
        #[test]
        fn test_get_socios_error(){
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            let mut contrato = SemRust::new(3, 10); 
            contrato.set_flag();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert!(contrato.get_socios().is_err());
        }
    }

    // Testeo de funcionalidades
        #[test]
        fn test_autorizacion() {
            let account1 = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            let account3 = AccountId::from([0x3; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account1);

            let mut contrato = SemRust::new(3, 10); 
            assert!(contrato.autorizar(account2).is_ok());
            assert!(contrato.autorizar(account2).is_err());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);

            assert!(contrato.autorizar(account3).is_err());
        }

        
        #[test]
        fn test_desautorizacion() {
            let account1 = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            let account3 = AccountId::from([0x3; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account1);

            let mut contrato = SemRust::new(3, 10); 
            assert!(contrato.desautorizar(account2).is_err());
            contrato.autorizar(account2);
            assert!(contrato.desautorizar(account2).is_ok());
            assert!(contrato.desautorizar(account2).is_err());
        }

        #[test]
        fn test_set_owner(){
            let account1 = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account1);

            let mut contrato = SemRust::new(3, 10); 
            assert!(contrato.set_owner(account2).is_ok());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account1);
            assert!(contrato.set_owner(account1).is_err());
        }
        #[test]
        fn test_set_flag_ok(){
            let account1 = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account1);

            let mut contrato = SemRust::new(3, 10); 
            assert!(contrato.set_flag().is_ok());
            assert_eq!(contrato.flag, true);
        }
        #[test]
        fn test_set_flag_error(){
            let account1 = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account1);

            let mut contrato = SemRust::new(3, 10); 
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert!(contrato.set_flag().is_err());
        }
        
        #[test]
        fn get_flag_test(){
            let account1 = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account1);

            let mut contrato = SemRust::new(3, 10); 
            assert_eq!(contrato.get_flag(), false);
            contrato.set_flag().unwrap();
            assert_eq!(contrato.get_flag(), true);
        }

        #[test]
        fn test_get_precios(){
            let account1 = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account1);
            let mut contrato = SemRust::new(3,10);
            let result = contrato.get_precio_a();
            let result1 = contrato.get_precio_b();
            let result2 = contrato.get_precio_c();
            assert_eq!(result, 5000);
            assert_eq!(result1,3000);
            assert_eq!(result2,2000);

        }
        #[test]
        fn get_monto_pago_test(){
            let account1 = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            
            
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account1);
           
            let mut club: SemRust = SemRust::new(20,10);         
            club.creacion_socio(1, Categoria::A, None);     
            club.set_flag();
            assert!(club.get_monto_pago_priv(1).is_ok());
            club.set_owner(account2);
            assert_eq!(club.get_monto_pago_priv(1),Err("No esta Autorizado".to_string()));
        }

        #[test]
        fn esta_autorizado_test(){
            let acc = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(acc);
            let mut contrato = SemRust::new(1,1);
            contrato.set_flag();
            assert_eq!(contrato.esta_autorizado(), true);
        }
        
        #[test]
        fn get_vencimiento_pago(){
            let acc = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(acc);
            let mut contrato = SemRust::new(1,1);
            let fecha = Fecha{
                dia:16,
                mes:10,
                año:2003,
            };
            contrato.set_flag();
            let fecha = contrato.conversor();
            let pago = Pago{id:1, dni:1, monto:1, emision: fecha, vencimiento: fecha, pendiente: true, bonificado: false, vencido: false };
            assert!(contrato.get_vencimiento_pago(pago).is_ok());
            
        } 

        #[test]
        fn obtener_pagos_realizados(){
            let acc = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(acc);
            let mut contrato = SemRust::new(1,1);
            let socio1 = contrato.nuevo_socio(1,Categoria::A, None);
            let socio2 = contrato.nuevo_socio(2,Categoria::C, None);
            contrato.registrar_pago(1, 5000);
            contrato.registrar_pago(2,2000);
            assert!(contrato.obtener_pagos_realizados().is_ok());
        }
    }
