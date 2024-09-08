fn main() {
    array(); //tamanho fixo
    matriz();
    //array_erro_no_acesso_de_elemento();
    array_com_acesso_de_elemento_com_usize();
    println!("E fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Domingo));
    println!("A cor é {}", nome_da_cor(Cor::CYMK{ciano:40,amarelo:70,magenta:255,preto:0}));
    conteudo_opcional();
    vector(); //array com tamanho dinâmico
    conta_corrente();
}

fn array(){
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
fn array_erro_no_acesso_de_elemento(){
    let meu_array: [f32; 4] = [0.0; 4]; 
    let indice: i64 = 0;
    println!("Aqui da erro de usize -> {}", meu_array[indice]) //slice indices are of type `usize` or ranges of `usize`
}
*/

fn array_com_acesso_de_elemento_com_usize(){
    let meu_array: [f32; 4] = [201.8745; 4]; 
    let indice: usize = 0;
    println!("Acesso com tipo usize -> {}", meu_array[indice])
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

    if let Some(conteudo) = &conteudo_do_arquivo {
        println!("O conteúdo do arquivo é: {}", conteudo)    
    }

    if let None = conteudo_do_arquivo {
        println!("Arquivo não existe!")
    }
}

fn ler_arquivo(arquivo: String) -> Option<String>{
    if arquivo != "" {
        Some(String::from("Conteúdo do arquivo!"))
    } else {
        None
    }
}

fn vector(){
    let mut notas: Vec<f32> = Vec::with_capacity(4); //Vec::new();
    notas.push(10.0);
    notas.push(8.8);
    notas.push(6.5);
    //let mut notas: Vec<f32> = vec![10.0, 8.8, 6.5];
    println!("{:?}", notas);
    println!("Capacidade: {}", notas.capacity());

    notas.push(7.2);
    println!("{:?}", notas);
    println!("Capacidade: {}", notas.capacity());

    println!("Nota 1 = {}", notas[0]);
    println!("Nota 5 = {}", match notas.get(4) {
        Some(nota) => *nota,
        None => 0.0  
    });

    println!("Último valor de nota removido: {:?}", notas.pop());
    println!("{:?}", notas);

    /*
    while let Some(nota) = notas.pop() {
        println!("Valor de nota removida: {}", nota);
    }
    println!("{:?}", notas);
    */

    for nota in &notas {
        println!("Valor de nota: {}", nota);
    }
    println!("{:?}", notas);
}

struct ContaCorrente {
    titular: Titular,
    saldo: f64 //problema de ponto flutuante: https://www.youtube.com/watch?v=qeZloBkUf6M
}

impl ContaCorrente {
    fn sacar(&mut self, valor: f64){
        self.saldo -= valor;
    }
}

struct Titular {
    documento: String,
    nome: String,
    sobrenome: String
}

impl Titular {
    fn nome_completo(&self) -> String {
        let mut nome_completo = String::new();
        nome_completo.push_str(&self.nome);
        nome_completo.push_str(" ");
        nome_completo.push_str(&self.sobrenome);
        nome_completo.to_string()
    }
}

fn conta_corrente(){
    let titular: String = String::from("Jhoni Conzatti");
    let saldo: f64 = 100.0;
    println!("Dados da conta corrente: titular = {}, saldo = R$ {}", titular, saldo);

    let mut conta_corrente: ContaCorrente = ContaCorrente{
        titular: Titular {
            documento: String::from("06699988811"), 
            nome: String::from("Jhoni"), 
            sobrenome: String::from("Conzatti")
        }, 
        saldo: 100.0
    };

    conta_corrente.sacar(50.0);

    println!(
        "Dados da conta corrente: titular = {} (cpf {}), saldo = R$ {}", 
        conta_corrente.titular.nome_completo(),
        conta_corrente.titular.documento, 
        conta_corrente.saldo
    );
}