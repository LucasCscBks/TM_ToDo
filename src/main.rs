fn main() {
    loop {
        println!("Olá deseja adicionar um novo ToDo?");
        println!("[sim/nao]");
        let mut ask: String = input();
        if ask == "sim" {
            println!("Digite um novo ToDo abaixo:");
            let todo: String = input();
            println!("{} // Adicionado com sucesso!", todo);
        } else if ask == "nao" {
            println!("Encerrando programa 3...2...1");
            break
        }
    }
}
    

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}