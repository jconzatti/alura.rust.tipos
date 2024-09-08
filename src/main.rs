fn main() {
    vetor();
    matriz();
    //vetor_erro_no_acesso_de_elemento();
    vetor_com_acesso_de_elemento_com_usize();
}

fn vetor(){
    let notas: [f32; 4] = [10.0, 8.0, 9.5, 6.0];

    println!("Nota 1: {}, nota 2: {}, nota 3: {}, nota 4: {}", notas[0], notas[1], notas[2], notas[3]);

    for nota in notas{
        println!("A nota é {}", nota);
    }

    let notas: [f32; 4] = [6.5; 4]; //todas as quatro notas são 6.5

    for i in 0..notas.len() {
        println!("Nota {}: {}", i+1, notas[i]);
    }
}

fn matriz(){
    let matriz: [[f32; 3]; 2] = [
        [0.0, 0.1, 0.2],
        [6.5, 8.1, 1.7]
    ];

    for linha in matriz{
        for elemento in linha{
            println!("Elemento {}", elemento)
        }
    }
}

/*
//O compilador Rust compila todas as funções, mesmo que não é usada.
fn vetor_erro_no_acesso_de_elemento(){
    let meu_vetor: [f32; 4] = [0.0; 4]; 
    let indice: i64 = 0;
    println!("Aqui da erro de usize -> {}", meu_vetor[indice]) //slice indices are of type `usize` or ranges of `usize`
}
*/

fn vetor_com_acesso_de_elemento_com_usize(){
    let meu_vetor: [f32; 4] = [201.8745; 4]; 
    let indice: usize = 0;
    println!("Acesso com tipo usize -> {}", meu_vetor[indice])
}
