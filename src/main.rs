fn teste_funcao() {
    println!("Oi")
}

fn nome(n: String){
    println!("Hello, {n}");
}

fn five() -> i8 { // Função com return embutido
    5
}

fn mais_um(i: i32) -> i32 {
    i + 1
}

fn main() {
    println!("Hello, world!");
    teste_funcao();
    nome("mateus".to_string());
    let x = five();
    let i = mais_um(5);

    println!("{x}, {i}");
}
