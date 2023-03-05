fn teste_funcao() {
    println!("Oi")
}

fn nome(n: String){
    println!("Hello, {n}");
}

fn main() {
    println!("Hello, world!");
    teste_funcao();
    nome("mateus".to_string())
}
