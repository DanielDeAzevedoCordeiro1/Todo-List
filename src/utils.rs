use std::io::{self, Write, stdout};

pub fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Erro ao ler dados de entrada!");
    buffer.trim().to_string()
}

pub fn show_menu() {
    println!("\n==========================================");
    println!("           ToDo App Manager            ");
    println!("==========================================");
    println!("\n[1] Listar todas as todos");
    println!("[2] Criar nova todo");
    println!("[3] Deletar todo");
    println!("[4] Salvar todos");
    println!("[5] Sair\n");
    print!("Escolha uma opcao: ");
    stdout().flush().unwrap();
}
