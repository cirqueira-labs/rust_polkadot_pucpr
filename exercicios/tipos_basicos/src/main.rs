fn main() {
    let mut idade:i32 =25;
    let peso: f64 =70.5;
    let ativo: bool = true;
    println!("Idade {:?} Peso {:?} Ativo {:?}", idade, peso, ativo);
    idade = 50;
    println!("Idade {:?} Peso {:?} Ativo {:?}", idade, peso, ativo);
}

// 1. Crie um novo projeto chamado tipos_basicos usando cargo.
// 2. Declare três variáveis com os seguintes tipos:
// • Uma i32 chamada idade com o valor 25.
// • Um f64 chamado peso com o valor 70.5.
// • Uma bool chamada ativo com o valor true.
// 3. Imprima os valores dessas variáveis usando println!.
// 4. Adicione a anotação de tipo explicitamente em cada variável.
// 5. Modifique a variável idade para ser mutável e atribua a ela um novo valor.
