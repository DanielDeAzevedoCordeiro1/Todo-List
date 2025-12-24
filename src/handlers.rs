use std::io::Write;

use crate::todo::Todo;
use crate::todo_db::TodoDb;
use crate::utils::get_input;
use std::io::stdout;

pub fn handle_list(db: &TodoDb) {
    if db.is_empty() {
        println!("\nNenhuma todo encontrada!");
        return;
    }

    println!("\n================ Lista de Todos =================");
    for todo in db.list() {
        println!("\n{}", todo.display());
        println!("=================================================");
    }
}

pub fn handle_create(db: &mut TodoDb) {
    println!("\nCriar Nova Todo");
    println!("=================================================");
    print!("Digite o titulo da todo: ");
    stdout().flush().unwrap();

    let title = get_input();
    if title.is_empty() {
        println!("Titulo nao pode ser vazio!");
        return;
    }

    let todo = Todo::new(title);
    db.add(todo);
    println!("Todo criada com sucesso!");
}

pub fn handle_delete(db: &mut TodoDb) {
    if db.is_empty() {
        println!("\nNenhuma todo encontrada para deletar!");
        return;
    }

    println!("\n================ Deletar Todo =================");
    for (index, todo) in db.list().iter().enumerate() {
        println!("[{}] {}", index + 1, todo.title);
    }

    println!("\n[0] Cancelar");
    print!("\nDigite o numero da todo: ");
    stdout().flush().unwrap();

    match get_input().parse::<usize>() {
        Ok(0) => println!("Operacao cancelada."),
        Ok(num) if num <= db.list().len() => {
            let title = &db.list()[num - 1].title;
            println!("Deletando: {}", title);
            db.delete_by_index(num - 1);
            println!("Todo deletada com sucesso!");
        }
        _ => println!("Numero invalido!"),
    }
}
