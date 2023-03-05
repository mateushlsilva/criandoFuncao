fn teste_funcao() {
    println!("Oi")
}

fn nome(n: String){
    println!("Hello, {n}");
}

fn five() -> i8 { // Função com return embutido
    5
}

fn main() {
    println!("Hello, world!");
    teste_funcao();
    nome("mateus".to_string());
    let x = five();

    println!("{x}");
}
