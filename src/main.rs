fn main() {
    vetor();
    matriz();
    //vetor_erro_no_acesso_de_elemento();
    vetor_com_acesso_de_elemento_com_usize();
    println!("E fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Domingo));
    println!("A cor é {}", nome_da_cor(Cor::CYMK{ciano:40,amarelo:70,magenta:255,preto:0}));
    conteudo_opcional();
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

#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool{
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false
    }
} 

#[allow(dead_code)]
enum Cor {
    Vermelho,
    Verde,
    Azul,
    RGB(u8, u8, u8),
    CYMK{
        ciano: u8,
        amarelo: u8,
        magenta: u8,
        preto: u8
    }
}

fn nome_da_cor(cor: Cor) -> String{
    match cor {
        Cor::Vermelho | Cor::RGB(255,0,0) => String::from("vermelha"),
        Cor::Verde | Cor::RGB(0,255,0) => String::from("verde"),
        Cor::Azul | Cor::RGB(0,0,255) => String::from("azul"),
        Cor::RGB(0,0,0) | Cor::CYMK{ciano: _, amarelo: _, magenta: _, preto: 255}=> String::from("preta"),
        Cor::RGB(255,255,255) => String::from("branca"),
        Cor::CYMK{ciano: 255, amarelo: _, magenta: _, preto: _}=> String::from("ciano"),
        Cor::CYMK{ciano: _, amarelo: 255, magenta: _, preto: _}=> String::from("amarela"),
        Cor::CYMK{ciano: _, amarelo: _, magenta: 255, preto: _}=> String::from("magenta"),
        Cor::RGB(_,_,_) => String::from("RGB desconhecida"),
        Cor::CYMK{ciano: _, amarelo: _, magenta: _, preto: _} => String::from("CYMK desconhecida")
    }
}

fn conteudo_opcional(){
    let conteudo_do_arquivo = ler_arquivo(String::from("file.txt"));
    match &conteudo_do_arquivo {
        Some(conteudo) => println!("{}", conteudo),
        None => println!("Arquivo não existe!")
    }
    println!("{:?}", conteudo_do_arquivo);

    if let Some(conteudo) = conteudo_do_arquivo {
        println!("O conteúdo do arquivo é: {}", conteudo)    
    }
}

fn ler_arquivo(arquivo: String) -> Option<String>{
    if arquivo != "" {
        Some(String::from("Conteúdo do arquivo!"))
    } else {
        None
    }
}
