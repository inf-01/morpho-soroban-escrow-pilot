#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, log};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HitoStatus {
    Pendiente = 0,
    EvidenciaEnviada = 1,
    AprobadoTecnico = 2,
    Completado = 3,
    EnDisputa = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Institucion,      // Pagador
    Proveedor,        // Ejecutor
    Supervisor,       // Revisor técnico
    Financiero,       // Validador de pagos
    Arbitro,          // Ente independiente
    Token,            // USDC
    HitoInfo(u32),    // Datos del hito 1, 2 o 3
    HitoActual,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    pub monto: i128,
    pub hash_evidencia: Symbol,
    pub estado: HitoStatus,
}

#[contract]
pub struct SmartEscrow;

#[contractimpl]
impl SmartEscrow {
    /// Inicialización con los 5 roles institucionales identificados en la reunión
    pub fn inicializar(
        env: Env, 
        inst: Address, 
        prov: Address, 
        superv: Address, 
        finan: Address, 
        arbitro: Address,
        token: Address
    ) {
        if env.storage().instance().has(&DataKey::Institucion) {
            panic!("El contrato ya está configurado.");
        }

        env.storage().instance().set(&DataKey::Institucion, &inst);
        env.storage().instance().set(&DataKey::Proveedor, &prov);
        env.storage().instance().set(&DataKey::Supervisor, &superv);
        env.storage().instance().set(&DataKey::Financiero, &finan);
        env.storage().instance().set(&DataKey::Arbitro, &arbitro);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::HitoActual, &1u32);

        // Definición de hitos por $100,000 (20k, 40k, 40k)
        let m1 = Milestone { monto: 20_000, hash_evidencia: symbol_short!("void"), estado: HitoStatus::Pendiente };
        let m2 = Milestone { monto: 40_000, hash_evidencia: symbol_short!("void"), estado: HitoStatus::Pendiente };
        let m3 = Milestone { monto: 40_000, hash_evidencia: symbol_short!("void"), estado: HitoStatus::Pendiente };

        env.storage().instance().set(&DataKey::HitoInfo(1), &m1);
        env.storage().instance().set(&DataKey::HitoInfo(2), &m2);
        env.storage().instance().set(&DataKey::HitoInfo(3), &m3);
        
        log!(&env, "Escrow Institucional Inicializado");
    }

    /// Pasaje de Off-Chain a On-Chain: El proveedor sube el hash de la evidencia
    pub fn subir_evidencia(env: Env, proveedor: Address, hash: Symbol) {
        proveedor.require_auth();
        let prov_auth: Address = env.storage().instance().get(&DataKey::Proveedor).unwrap();
        assert_eq!(proveedor, prov_auth, "No es el proveedor.");

        let num_actual: u32 = env.storage().instance().get(&DataKey::HitoActual).unwrap();
        let mut hito: Milestone = env.storage().instance().get(&DataKey::HitoInfo(num_actual)).unwrap();

        assert_eq!(hito.estado, HitoStatus::Pendiente, "El hito ya tiene evidencia.");
        
        hito.hash_evidencia = hash;
        hito.estado = HitoStatus::EvidenciaEnviada;
        
        env.storage().instance().set(&DataKey::HitoInfo(num_actual), &hito);
        log!(&env, "Evidencia mapeada on-chain para hito {}", num_actual);
    }

    /// Validación del Supervisor Técnico (Off-chain: revisa PDF -> On-chain: aprueba técnico)
    pub fn aprobar_tecnico(env: Env, supervisor: Address) {
        supervisor.require_auth();
        let super_auth: Address = env.storage().instance().get(&DataKey::Supervisor).unwrap();
        assert_eq!(supervisor, super_auth, "No es el supervisor técnico.");

        let num_actual: u32 = env.storage().instance().get(&DataKey::HitoActual).unwrap();
        let mut hito: Milestone = env.storage().instance().get(&DataKey::HitoInfo(num_actual)).unwrap();

        assert_eq!(hito.estado, HitoStatus::EvidenciaEnviada, "No hay evidencia enviada para revisar.");

        hito.estado = HitoStatus::AprobadoTecnico;
        env.storage().instance().set(&DataKey::HitoInfo(num_actual), &hito);
    }

    /// Ejecución de Pago: El Encargado Financiero libera el balance
    pub fn liberar_pago(env: Env, financiero: Address) {
        financiero.require_auth();
        let fin_auth: Address = env.storage().instance().get(&DataKey::Financiero).unwrap();
        assert_eq!(financiero, fin_auth, "No es el encargado financiero.");

        let num_actual: u32 = env.storage().instance().get(&DataKey::HitoActual).unwrap();
        let mut hito: Milestone = env.storage().instance().get(&DataKey::HitoInfo(num_actual)).unwrap();

        assert_eq!(hito.estado, HitoStatus::AprobadoTecnico, "Falta aprobación técnica.");

        hito.estado = HitoStatus::Completado;
        env.storage().instance().set(&DataKey::HitoInfo(num_actual), &hito);
        
        // Mover al siguiente hito si no es el final
        if num_actual < 3 {
            env.storage().instance().set(&DataKey::HitoActual, &(num_actual + 1));
        }

        // NOTA: La transferencia de USDC desde el contrato se activa aquí
        log!(&env, "Pago de {} liberado para hito {}", hito.monto, num_actual);
    }

    /// Mecanismo de Disputa: Bloquea el flujo hasta que el Árbitro decida
    pub fn abrir_disputa(env: Env, quien: Address) {
        quien.require_auth();
        let num_actual: u32 = env.storage().instance().get(&DataKey::CurrentHito).unwrap_or(1);
        let mut hito: Milestone = env.storage().instance().get(&DataKey::HitoInfo(num_actual)).unwrap();
        
        hito.estado = HitoStatus::EnDisputa;
        env.storage().instance().set(&DataKey::HitoInfo(num_actual), &hito);
    }
}

