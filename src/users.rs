use crate::validator::Validador;
use rand::Rng;
//use serde::{Deserialize, Serialize};
//use uuid::Uuid;
pub struct User {
    //pub id: String,
    pub nome: String,
    pub saldo: u64,
    pub validador: Validador,
    pub stake: u64,
    pub perfil: PerfilStake,
}

pub enum PerfilStake {
    Iniciante,
    Intermediario,
    Avancado,
    Profissional,
}

impl PerfilStake {
    pub fn percentual_maximo(&self) -> (f64, f64) {
        match self {
            PerfilStake::Iniciante => (0.0, 0.25),
            PerfilStake::Intermediario => (0.25, 0.5),
            PerfilStake::Avancado => (0.5, 0.75),
            PerfilStake::Profissional => (0.75, 1.0),
        }
    }
    pub fn perfil_usuario(perfil: &str) -> Option<Self> {
        match perfil.trim() {
            "1" => Some(PerfilStake::Iniciante),
            "2" => Some(PerfilStake::Intermediario),
            "3" => Some(PerfilStake::Avancado),
            "4" => Some(PerfilStake::Profissional),
            _ => None,
        }
    }
}

impl User {
    pub fn novo(nome: &str, perfil: PerfilStake, validador: Validador) -> Self {
        Self {
            //id: Uuid::new_v4().to_string(),
            nome: nome.to_string(),
            saldo: 1000,
            stake: 0,
            validador,
            perfil,
        }
    }
    pub fn realizar_stake(&mut self, valor: u64) -> Result<(), String> {
        if valor > self.saldo {
            return Err(format!("Saldo indisponivel"));
        }
        self.saldo -= valor;
        self.stake += valor;

        Ok(())
    }

    pub fn aplicar_stake_profile(&mut self) {
        let (min_pct, max_pct) = self.perfil.percentual_maximo();
        let mut rng = rand::thread_rng();
        let percentual = rng.gen_range(min_pct..=max_pct);
        let valor_stake = (self.saldo as f64 * percentual).round() as u64;

        match self.realizar_stake(valor_stake) {
            Ok(_) => println!("Stake realizado com sucesso! Usuário: {}, Valor: {}, Novo saldo: {}, Total em stake: {}",
                self.nome, valor_stake, self.saldo, self.stake),
            Err(e) => println!("Erro ao realizar stake para {}: {}", self.nome, e),
        }
    }
}
