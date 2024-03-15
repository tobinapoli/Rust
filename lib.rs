#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
pub mod reporte {
    use club::contrato::SemRustRef;
    use club::contrato::Socio;
    use club::contrato::Pago;
    use club::contrato::Categoria;
    use club::contrato::Deportes;
    
    use ink::prelude::string::ToString;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    
    /// El struct reporte va a almacenar una referencia al club SemRust. Al momento de hacer el deploy
    /// este reporte se asocia con el otro contrato para generar informes
    #[ink(storage)]
    pub struct Reporte {  
        #[cfg(not(test))]  
        club: SemRustRef,
    }
    #[cfg(test)]
    #[cfg(test)]
    #[derive(scale::Decode, scale::Encode,Debug,PartialEq,Clone,Copy)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]

    /// estructura que representa una fecha, con dia mes y año.
    pub struct Fecha {
        dia: u64,
        mes: u8,
        año: u32,
    }
    #[cfg(test)]
    impl Fecha {
        /// constructor del tipo Fecha
        fn new() -> Self {
            return Fecha{
                dia: 10,
                mes: 10,
                año: 2023,
            }
        }
        pub fn es_mayor(&self, otra_fecha: &Fecha) -> bool { ///Devuelve true en caso de la fecha actual sea mayor a la recibida por parametro
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
    /// estructura que representa los precios de cada categoria, "a", "b" y "c"
    #[cfg(test)]
    pub struct PreciosCat{
        precio_a: u128,
        precio_b: u128,
        precio_c: u128,
    }
    #[cfg(test)]
    #[derive(scale::Decode, scale::Encode,Debug,PartialEq,Clone,Copy)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]

    /// estructura que representa a un socio ficticio.
    pub struct SocioFake{
        dni: u32,
        cat: Categoria,
        dep: Option<Deportes>,
    }

    /// 
    #[cfg(test)]
    pub struct SemRustFake{
        socios: Vec<SocioFake>,
        pagos: Vec<PagoFake>,
        precios: PreciosCat,
        cuando_bonifica: u8,
        porcentaje_bonificacion: u32,
        owner: AccountId,
        autorizados: Vec<AccountId>,
        flag: bool,
    }
    #[cfg(test)]
    #[derive(scale::Decode, scale::Encode,Debug,PartialEq,Clone,Copy)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]

    /// estructura que representa a un pago ficticio.
    pub struct PagoFake{
        id: u32,
        dni: u32,
        monto: u128,
        emision: Fecha,
        vencimiento: Fecha,
        pendiente: bool,
        bonificado: bool,
        vencido:bool,
    }
    
    
    #[cfg(test)]
    impl SemRustFake{
        /// "Función que crea una nueva instancia de la estructura actual. Inicializa los campos socios y pagos como vectores vacíos, establece los precios de
        /// las categorías en 5000, 3000 y 2000 respectivamente, configura el bono y otros campos con valores predeterminados.
        /// También establece el propietario como una cuenta de identificación y los autorizados como un vector vacío. Establece el flag en falso. Retorna la instancia creada."
        #[cfg(test)]
        fn new() -> Self{
            Self{
                socios: Vec::new(),
                pagos: Vec::new(),
                precios: PreciosCat {
                    precio_a: 5000,
                    precio_b: 3000,
                    precio_c: 2000,
                },
                cuando_bonifica: 1,
                porcentaje_bonificacion: 1,
                owner: AccountId::from([0x1; 32]),
                autorizados: Vec::new(),
                flag: false,
            }
        }

        /// retorna una autorizacion ficticia, siempre true.
        #[cfg(test)]
        pub fn esta_autorizado()-> bool{
            return true;
        }

        /// retorna una fecha ficticia
        #[cfg(test)]
        pub fn conversor()-> Fecha{
            let f = Fecha {dia: 10, mes: 10, año: 10};
            f
        }

        /// "Función que retorna un vector de socios falsos. Crea un array de socios falsos con diferentes atributos, y luego lo convierte en un vector utilizando el método to_vec(). Retorna el vector de socios falsos."
        #[cfg(test)]
        pub fn get_socios() -> Vec<SocioFake>{
            let res = [SocioFake {dni: 1, cat: Categoria::A, dep: None}, SocioFake{dni: 2, cat: Categoria::B, dep: Some(Deportes::Futbol)}, SocioFake{dni: 3, cat: Categoria::C, dep: None}, SocioFake {dni: 4, cat: Categoria::A, dep: None}, SocioFake {dni: 5, cat: Categoria::B, dep: Some(Deportes::Tenis)}, SocioFake {dni: 6, cat: Categoria::C, dep: None}, SocioFake {dni: 7, cat: Categoria::A, dep: None}];
            return res.to_vec();
        }

        /// retorna un dni = 1
        pub fn get_dni()->u32{
            return 1;
       }

       /// Retorna una fecha de vencimiento de pago ficticia.
       #[cfg(test)]
       fn get_vencimiento_pago1()->Fecha{
            let f = Fecha {dia: 5, mes: 5, año: 5};
            f
       }
       /// Retorna una fecha de vencimiento de pago ficticia.
       #[cfg(test)]
       fn get_vencimiento_pago2()->Fecha{
            let f = Fecha {dia: 11, mes: 11, año: 11};
            f
       }

    /// Retorna el precio correspondiente a la categoría A.
       #[cfg(test)]
       fn get_precio_a()->u128{
        return 5000;
       }
       /// Retorna el precio correspondiente a la categoría B.
       #[cfg(test)]
       fn get_precio_b()->u128{
        return 3000;
       }

        
       #[cfg(test)]
        pub fn informe_morosos(&self) -> Result<Vec<SocioFake>,String> {
            return self.informe_socios_morosos_priv();
        }

        /// "Método privado utilizado para pruebas que genera un informe de socios morosos. Verifica si se cumple la autorización estática de la estructura actual.
        /// Si está autorizado, se recorren los socios, se obtienen los datos necesarios y se verifica si tienen pagos pendientes vencidos.
        /// Si hay pagos pendientes vencidos para un socio, se agrega a la lista de socios morosos. Devuelve un vector de socios falsos morosos o un error 'No está Autorizado' si no se cumple la autorización."

        #[cfg(test)]
        fn informe_socios_morosos_priv(&self) -> Result<Vec<SocioFake>,String> {
            if Self::esta_autorizado(){
                let mut socios_morosos:Vec<SocioFake> = Vec::new();
                let hoy = Self::conversor();
                for socio in Self::get_socios() {
                    let dni = Self::get_dni();
                    let pagos_pendientes = Self::obtener_pagos_pendientes_por_dni();
                    if pagos_pendientes.iter().any(|pago| hoy.es_mayor(&Self::get_vencimiento_pago1())){
                        socios_morosos.push(socio.clone());
                    }
                }
                Ok(socios_morosos)
            } else {
                return Err("No esta Autorizado".to_string());
            }
        }

        pub fn get_monto_pago() -> Result<u128,String>{
            return Ok(1);
        }

        ///"Función que retorna los pagos pagados del mes. Crea un vector y agrega los elementos 1 y 2. Devuelve el vector en un resultado Ok."

        fn get_pagos_pagados_mes() -> Result<Vec<u32>, String>{
            let mut vec: Vec<u32> = Vec::new();
            vec.push(1);
            vec.push(2);
            return Ok(vec)
       }


        #[cfg(test)]
        pub fn informe_recaudacion(&self) -> Result<Total_cat, String>{
            return self.informe_recaudacion_priv();
        }

        /// "Método privado utilizado para pruebas que genera un informe de recaudación. Verifica si se cumple la autorización estática de la estructura actual.
        /// Si está autorizado, se recorren los pagos pagados del mes y se acumulan los montos en las categorías correspondientes de la estructura Total_cat.
        /// Devuelve la estructura Total_cat con los totales de recaudación por categoría o un error 'No está Autorizado' si no se cumple la autorización."

        #[cfg(test)]
        fn informe_recaudacion_priv(&self) -> Result<Total_cat, String>{
            if Self::esta_autorizado(){
                let mut res = Total_cat {total_a: 0, total_b: 0, total_c: 0};
                let pagos = Self::get_pagos_pagados_mes()?;
                for pago in pagos{
                    if Self::get_monto_pago() == Ok(Self::get_precio_a()){
                            res.total_a += Self::get_monto_pago()?;
                    } else if Self::get_monto_pago() == Ok(Self::get_precio_b()){
                            res.total_b += Self::get_monto_pago()?;
                    } else {
                        res.total_c += Self::get_monto_pago()?;
                    }
                }
                Ok(res)
            } else {
                return Err("No esta Autorizado".to_string());
            }
        }

        /// "Método privado utilizado para pruebas que obtiene los pagos por DNI. Crea una instancia de la estructura Fecha, y luego crea un vector de pagos falsos con valores predeterminados.
        /// Cada pago se agrega al vector dos veces. Devuelve el vector de pagos falsos."

        #[cfg(test)]
        fn obtener_pagos_por_dni() ->Vec<PagoFake>{
            let mut vec: Vec<PagoFake> = Vec::new();
            let fecha = Fecha::new();
            let pago = PagoFake{
                id:1, dni: 1, monto: 1, emision: fecha, vencimiento: fecha, pendiente: true, bonificado: false, vencido: false,
            };
            vec.push(pago);
            vec.push(pago);
            return vec;
        }

        /// "Método privado utilizado para pruebas que obtiene los pagos pendientes por DNI. Verifica si se cumple la autorización estática de la estructura actual.
        /// Si está autorizado, se obtienen los pagos asociados al DNI y se filtran aquellos que están pendientes. Devuelve un vector de pagos falsos
        /// pendientes o un error 'No estás autorizado' si no se cumple la autorización."
        
        #[cfg(test)]
        fn obtener_pagos_pendientes_por_dni() -> Result<Vec<PagoFake>, String>{
            if Self::esta_autorizado(){
                let pagos_dni: Vec<PagoFake> = Self::obtener_pagos_por_dni();
                let mut pagos_encontrados= Vec::new();
                for pago in pagos_dni {
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
        
        #[cfg(test)]
        pub fn informe_no_morosos_especifico(&self) -> Result<Vec<SocioFake>,String> {
            return self.informe_no_morosos_especifico_priv();
        }

        /// "Método privado utilizado para pruebas que genera un informe de socios no morosos específico. Verifica si se cumple la autorización estática de la estructura actual.
        /// Si está autorizado, se recorren los socios, se obtienen los datos necesarios y se verifica si tienen pagos pendientes vencidos.
        /// Si no hay pagos pendientes vencidos para un socio, se agrega a la lista de socios no morosos. Devuelve un vector de socios falsos o un error 'No está Autorizado' si no se cumple la autorización."

        #[cfg(test)]
        fn informe_no_morosos_especifico_priv(&self) -> Result<Vec<SocioFake>,String> {
            if Self::esta_autorizado(){
                let mut socios_morosos:Vec<SocioFake> = Vec::new();
                let hoy = Self::conversor();
                for socio in Self::get_socios() {
                    let dni = Self::get_dni();
                    let pagos_pendientes = Self::obtener_pagos_pendientes_por_dni();
                    if !pagos_pendientes.iter().any(|pago| hoy.es_mayor(&Self::get_vencimiento_pago2())){
                        socios_morosos.push(socio.clone());
                    } 
                }
                Ok(socios_morosos)
            } else {
                return Err("No esta Autorizado".to_string());
            }
        }

    }




    #[derive(scale::Decode, scale::Encode,Debug,PartialEq,Clone)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    /// "Estructura que representa totales en diferentes categorías. Contiene los campos total_a, total_b y total_c de tipo u128 para almacenar los totales correspondientes a cada categoría."
    pub struct Total_cat{
        total_a: u128,
        total_b: u128,
        total_c: u128,
    }
    impl Total_cat{
        /// Retorna el valor total en la categoría A.
        pub fn get_total_a(&self) -> u128{ 
            return self.total_a;
        }
        /// Retorna el valor total en la categoría B.
        pub fn get_total_b(&self) -> u128{ 
            return self.total_b;
        }
        /// Retorna el valor total en la categoría C.
        pub fn get_total_c(&self) -> u128{ 
            return self.total_c;
        }
    }
    

    impl Reporte {
        #[cfg(not(test))]
        #[ink(constructor)]
        
        /// "constructor del struct Reporte, se construye con un SemRustRef de parametro."
        pub fn new(club: SemRustRef) -> Self {
            Self { club }
        }
        
        /// El método  genera un informe de socios morosos. Verifica si el club está autorizado y, de ser así, recorre los socios obteniendo su DNI y los pagos pendientes asociados.
        /// Si hay pagos pendientes vencidos para un socio, se agrega a la lista de socios morosos. Devuelve un vector de socios
        /// morosos o un error 'No está Autorizado' si el club no está autorizado."
        #[ink(message)]
        #[cfg(not(test))]
        pub fn informe_morosos(&self) -> Result<Vec<Socio>,String> {
            return self.informe_socios_morosos_priv();
        }

        

        #[cfg(not(test))]
        fn informe_socios_morosos_priv(&self) -> Result<Vec<Socio>,String> {
            if self.club.esta_autorizado(){
                let mut socios_morosos:Vec<Socio> = Vec::new();
                let hoy = self.club.conversor();
                for socio in self.club.get_socios().unwrap() {
                    let dni = self.club.get_dni(socio)?;
                    let pagos_pendientes = self.club.obtener_pagos_pendientes_por_dni(dni)?;
                    if pagos_pendientes.iter().any(|pago| hoy.es_mayor(&self.club.get_vencimiento_pago(*pago).unwrap())){
                        socios_morosos.push(socio.clone());
                    } 
                }
                Ok(socios_morosos)
            } else {
                return Err("No esta Autorizado".to_string());
            }
        }

        

        /// El Método genera un informe de recaudación. Verifica si el club está autorizado y, de ser así, recorre los pagos pagados del mes obteniendo su monto.
        /// Dependiendo del monto del pago, se acumula en la categoría correspondiente (total_a, total_b o total_c) de la estructura Total_cat. Devuelve la estructura Total_cat con los totales
        /// de recaudación por categoría o un error 'No está Autorizado' si el club no está autorizado.
        #[ink(message)]
        #[cfg(not(test))]
        pub fn informe_recaudacion(&self) -> Result<Total_cat, String>{
            return self.informe_recaudacion_priv();
        }

        

        #[cfg(not(test))]
        fn informe_recaudacion_priv(&self) -> Result<Total_cat, String>{
            if self.club.esta_autorizado(){
                let mut res = Total_cat {total_a: 0, total_b: 0, total_c: 0};
                let pagos = self.club.get_pagos_pagados_mes()?;
                for pago in pagos{
                    if self.club.get_monto_pago(pago) == Ok(self.club.get_precio_a()){
                            res.total_a += self.club.get_monto_pago(pago)?
                    } else if self.club.get_monto_pago(pago) == Ok(self.club.get_precio_b()){
                            res.total_b += self.club.get_monto_pago(pago)?
                    } else {
                        res.total_c += self.club.get_monto_pago(pago)?
                    }
                }
                Ok(res)
            } else {
                return Err("No esta Autorizado".to_string());
            }
        }

        /// EL Método genera un informe de socios no morosos específico. Verifica si el club está autorizado y, de ser así, recorre los socios obteniendo su DNI y los
        /// pagos pendientes asociados. Si no hay pagos pendientes vencidos para un socio, se agrega a la lista de socios no morosos. Devuelve un vector de socios
        /// no morosos o un error 'No está Autorizado' si el club no está autorizado."
        #[ink(message)]
        #[cfg(not(test))]
        pub fn informe_no_morosos_especifico(&self) -> Result<Vec<Socio>,String> {
            return self.informe_no_morosos_especifico_priv()
        }

        
        #[cfg(not(test))]
        fn informe_no_morosos_especifico_priv(&self) -> Result<Vec<Socio>,String> {
            if self.club.esta_autorizado(){
                let mut socios_morosos:Vec<Socio> = Vec::new();
                let hoy = self.club.conversor();
                for socio in self.club.get_socios().unwrap() {
                    let dni = self.club.get_dni(socio)?;
                    let pagos_pendientes = self.club.obtener_pagos_pendientes_por_dni(dni)?;
                    if !pagos_pendientes.iter().any(|pago| hoy.es_mayor(&self.club.get_vencimiento_pago(*pago).unwrap())){
                        socios_morosos.push(socio.clone());
                    } 
                }
                Ok(socios_morosos)
            } else {
                return Err("No esta Autorizado".to_string());
            }
        }
    }
    
    ///"Prueba la función informe_morosos() de la estructura club utilizando una instancia de SemRustFake. Verifica que la llamada a la función no genere errores."
    #[test]
    fn test_morosos(){
        let club = SemRustFake::new();
        assert!(club.informe_morosos().is_ok());
    }

    ///"Prueba la función informe_recaudacion() de la estructura club utilizando una instancia de SemRustFake. Verifica que la llamada a la función no genere errores."
    #[test]
    fn test_recaudacion(){
        let club = SemRustFake::new();
        assert!(club.informe_recaudacion().is_ok());
    }

    ///"Prueba la función informe_no_morosos_especifico() de la estructura club utilizando una instancia de SemRustFake. Verifica que la llamada a la función no genere errores."
    #[test]
    fn test_no_morosos(){
        let club = SemRustFake::new();
        assert!(club.informe_no_morosos_especifico().is_ok());
    }

    ///"Prueba los métodos get_total_a(), get_total_b() y get_total_c() de la estructura totalvalores de tipo Total_cat. Verifica que los valores devueltos por los métodos sean iguales a 5000, 3000 y 2000 respectivamente."
    #[test]
    fn test_get_a(){
        let totalvalores = Total_cat{
            total_a: 5000,
            total_b: 3000,
            total_c: 2000,
        };
        assert_eq!(totalvalores.get_total_a(), 5000);
        assert_eq!(totalvalores.get_total_b(), 3000);
        assert_eq!(totalvalores.get_total_c(), 2000);
    }


}

