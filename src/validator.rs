use crate::users::User;
use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug)]
pub enum Validador {
    NaoValidador,
    Inscrito,
    Validador,
}

impl Validador {
    pub fn inscrever_user(input: &str) -> Option<Self> {
        match input.trim() {
            "1" => Some(Validador::Inscrito),
            "2" => Some(Self::NaoValidador),
            _ => None,
        }
    }
    pub fn executar_sorteio(usuarios: &mut Vec<User>) {
        let mut roleta: Vec<usize> = Vec::new();
        for (i, user) in usuarios.iter_mut().enumerate() {
            if matches!(user.validador, Validador::Inscrito) {
                let entradas = user.stake as usize;
                for _ in 0..entradas {
                    roleta.push(i);
                }
            }
        }

        if let Some(&escolhido_idx) = roleta.choose(&mut thread_rng()) {
            let escolhido = &mut usuarios[escolhido_idx];
            escolhido.validador = Validador::Validador;
        } else {
            println!("Nenhum validador elegivel para o sorteio.");
        }
    }

    pub fn recompensa_penalidade_validadores(usuarios: &mut Vec<User>) {
        let mut rng = thread_rng();
        let chance: u8 = rng.gen_range(0..100);
        if let Some(validador) = usuarios
            .iter_mut()
            .find(|u| matches!(u.validador, Validador::Validador))
        {
            if chance > 85 {
                let perda = (validador.stake as f64 * 0.005) as u64;
                validador.stake = validador.stake.saturating_sub(perda);
                println!("Usuario {} perdeu {} Stakes.", validador.nome, perda);
            } else {
                //let valor_anterior = validador.stake;
                let ganho = (validador.stake as f64 * 0.0075) as u64;
                validador.stake += ganho;
                println!("Usuario {} recebeu {} Stakes.", validador.nome, ganho);
            }
        }
    }
    pub fn remover_validador_to_inscrito(usuarios: &mut Vec<User>) {
        if let Some(validador) = usuarios
            .iter_mut()
            .find(|u| matches!(u.validador, Validador::Validador))
        {
            validador.validador = Validador::Inscrito;
            println!(
                "Usuario {} possui agora {} Stakes.",
                validador.nome, validador.stake
            );
        }
    }
}
