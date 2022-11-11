fn main() {
    loop {
        println!("Olá deseja adicionar um novo ToDo?");
        println!("[sim/nao]");
        let ask: String = input();
        if ask == "sim" {
            println!("Digite um novo ToDo abaixo:");
            let todo: String = input();
            println!("     ");
            println!("{} // Adicionado com sucesso! //", todo);
        } else if ask == "nao" {
            println!("Encerrando programa em");
            println!("3");
            println!("2");
            println!("1");
            break
        }
    }
}
    

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}