#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec, symbol_short, panic_with_error};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MilestoneStatus {
    Pending = 0,
    ApprovedTechnical = 1,
    Completed = 2,
    InDispute = 3,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Milestone {
    pub amount: i128,
    pub status: MilestoneStatus,
    pub evidence_hash: Symbol,
}

#[contracttype]
pub enum DataKey {
    Institution = 0,
    Provider = 1,
    Supervisor = 2,
    Financiero = 3,
    Arbiter = 4,
    Milestones = 5,
    IsInitialized = 6,
    Token = 7,
}

#[contract]
pub struct SmartEscrow;

#[contractimpl]
impl SmartEscrow {
    /// Inicializa el contrato con los roles y el token (USDC)
    pub fn initialize(
        env: Env,
        institution: Address,
        provider: Address,
        supervisor: Address,
        financiero: Address,
        arbiter: Address,
        token: Address,
    ) {
        if env.storage().instance().has(&DataKey::IsInitialized) {
            panic!("Contrato ya inicializado");
        }

        env.storage().instance().set(&DataKey::Institution, &institution);
        env.storage().instance().set(&DataKey::Provider, &provider);
        env.storage().instance().set(&DataKey::Supervisor, &supervisor);
        env.storage().instance().set(&DataKey::Financiero, &financiero);
        env.storage().instance().set(&DataKey::Arbiter, &arbiter);
        env.storage().instance().set(&DataKey::Token, &token);

        // Definición de los 3 hitos ($20k, $40k, $40k)
        let mut milestones: Vec<Milestone> = Vec::new(&env);
        
        // Hito 1: $20,000
        milestones.push_back(Milestone {
            amount: 20_000_000_000, // Ajustado a 7 decimales si es USDC estándar
            status: MilestoneStatus::Pending,
            evidence_hash: symbol_short!("none"),
        });

        // Hito 2: $40,000
        milestones.push_back(Milestone {
            amount: 40_000_000_000,
            status: MilestoneStatus::Pending,
            evidence_hash: symbol_short!("none"),
        });

        // Hito 3: $40,000
        milestones.push_back(Milestone {
            amount: 40_000_000_000,
            status: MilestoneStatus::Pending,
            evidence_hash: symbol_short!("none"),
        });

        env.storage().instance().set(&DataKey::Milestones, &milestones);
        env.storage().instance().set(&DataKey::IsInitialized, &true);
    }

    /// El Supervisor aprueba técnicamente un hito tras revisar la evidencia
    pub fn approve_milestone(env: Env, milestone_index: u32, evidence_hash: Symbol) {
        let supervisor: Address = env.storage().instance().get(&DataKey::Supervisor).unwrap();
        supervisor.require_auth();

        let mut milestones: Vec<Milestone> = env.storage().instance().get(&DataKey::Milestones).unwrap();
        let mut milestone = milestones.get(milestone_index).unwrap();

        if milestone.status != MilestoneStatus::Pending {
            panic!("El hito no está en estado pendiente");
        }

        milestone.status = MilestoneStatus::ApprovedTechnical;
        milestone.evidence_hash = evidence_hash;
        
        milestones.set(milestone_index, milestone);
        env.storage().instance().set(&DataKey::Milestones, &milestones);
    }

    /// El Financiero libera los fondos una vez aprobado técnicamente
    pub fn release_funds(env: Env, milestone_index: u32) {
        let financiero: Address = env.storage().instance().get(&DataKey::Financiero).unwrap();
        financiero.require_auth();

        let mut milestones: Vec<Milestone> = env.storage().instance().get(&DataKey::Milestones).unwrap();
        let mut milestone = milestones.get(milestone_index).unwrap();

        if milestone.status != MilestoneStatus::ApprovedTechnical {
            panic!("El hito requiere aprobación técnica previa");
        }

        let provider: Address = env.storage().instance().get(&DataKey::Provider).unwrap();
        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        
        // Lógica de transferencia usando el cliente de Token de Soroban
        // Nota: Se asume que el contrato tiene balance suficiente depositado previamente
        let client = soroban_sdk::token::Client::new(&env, &token_addr);
        client.transfer(&env.current_contract_address(), &provider, &milestone.amount);

        milestone.status = MilestoneStatus::Completed;
        milestones.set(milestone_index, milestone);
        env.storage().instance().set(&DataKey::Milestones, &milestones);
    }

    /// Obtener el estado de un hito
    pub fn get_milestone(env: Env, index: u32) -> Milestone {
        let milestones: Vec<Milestone> = env.storage().instance().get(&DataKey::Milestones).unwrap();
        milestones.get(index).unwrap()
    }
}
