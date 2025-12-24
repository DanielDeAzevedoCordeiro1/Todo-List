mod handlers;
mod todo;
mod todo_db;
mod utils;

use handlers::{handle_create, handle_delete, handle_list};
use todo_db::TodoDb;
use utils::{get_input, show_menu};

const FILENAME: &str = "todos.json";

fn main() {
    let mut db = TodoDb::load(FILENAME)
        .unwrap_or_else(|_| {
            println!("Criando novo banco de dados...\n");
            TodoDb::new()
    });

    if !db.is_empty() {
        println!("{} todos carregadas!\n", db.list().len());
    }

    loop {
        show_menu();
        
        match get_input().parse::<u32>() {
            Ok(1) => handle_list(&db),
            Ok(2) => handle_create(&mut db),
            Ok(3) => handle_delete(&mut db),
            Ok(4) => {
                println!("\nSalvando alteracoes...");
                if let Err(e) = db.save(FILENAME) {
                    println!("Erro ao salvar: {}", e);
                }
            }
            Ok(5) => {
                println!("\nDeseja salvar as alteracoes antes de sair? (s/n): ");
                match get_input().to_lowercase().as_str() {
                    "s" | "sim" => {
                        println!("Salvando alteracoes...");
                        if let Err(e) = db.save(FILENAME) {
                            println!("Erro ao salvar: {}", e);
                        }
                    }
                    _ => println!("Alteracoes nao foram salvas."),
                }
                println!("Ate logo!\n");
                break;
            }
            _ => println!("\nOpcao invalida! Escolha de 1 a 5."),
        }
    }
}




