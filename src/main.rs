use std::io::{self, Write};

mod users;
mod validator;

use users::PerfilStake;
use users::User;
use validator::Validador;

fn criar_user(nome: &str, perfil_input: &str, validador_input: &str, usuarios: &mut Vec<User>) {
    match PerfilStake::perfil_usuario(&perfil_input) {
        Some(perfil_enum) => match Validador::inscrever_user(validador_input) {
            Some(validador_enum) => {
                let mut user = User::novo(nome.trim(), perfil_enum, validador_enum);
                println!("Usuário {} criado com sucesso!", user.nome);
                User::aplicar_stake_profile(&mut user);
                usuarios.push(user);
            }
            None => println!("Valor inválido para validador."),
        },
        None => println!("Perfil inválido."),
    }
}

fn inscrever_user(usuario: &mut Vec<User>) {
    let mut nome = String::new();
    let mut perfil_input = String::new();
    let mut validador_input = String::new();

    print!("Digite o nome do usuario: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut nome)
        .expect("Erro ao ler nome de usuario.");

    print!("\n");

    println!("Escolha um perfil que se adque a sua personalidade: ");
    println!("1 - Iniciante");
    println!("2 - Intermediario");
    println!("3 - Avancado");
    println!("4 - Profissional");

    print!("Digite o número do perfil: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut perfil_input)
        .expect("Erro ao ler perfil de usuario. Input invaldo.");

    print!("\n");

    if perfil_input >= String::from("3") {
        println!("Deseja se inscrever como Validador? ");
        println!("1 - Sim");
        println!("2 - Nao");

        print!("Digite sua resposta: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut validador_input)
            .expect("Input invaldo.");

        print!("\n");
    } else {
        validador_input = String::from("2");
    }

    criar_user(&nome, &perfil_input, &validador_input, usuario);

    print!("\n");
}

fn main() {
    let mut usuarios: Vec<User> = Vec::new();
    for _i in 0..10 {
        inscrever_user(&mut usuarios);
    }
    for _i in 0..60 {
        Validador::executar_sorteio(&mut usuarios);
        for _j in 0..6 {
            Validador::recompensa_penalidade_validadores(&mut usuarios);
        }
        Validador::remover_validador_to_inscrito(&mut usuarios);
        println!("Fim da rodada.\n");
    }
}
