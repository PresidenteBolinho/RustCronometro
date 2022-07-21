use std::io;

// Função com o objetivo de simular o aperto de um botão
fn press() {
    //Criação da variavel mutavel que vai receber as ações dos botões.
    let mut action = String::new();

    //Leitura da ação.
    io::stdin()
        .read_line(&mut action).
        expect("Não deu certo.");
    
    //Coversão para inteiro.
    let action: u8 = action.trim().parse().expect("Não é uma opção valida.");

    //Controle de fluxo com base na ação.
    if action == 1 {
            println!("Botão apertado");
    }
}

fn main() {
    //Perguntas: Do que eu preciso
    //Descobrir uma forma de contar o tempo, e retornar essa contagem em tempo real para o contador
    //Especificiar os comportamentos dos botões e suas interações com o contador
    println!("Bem vindo ao meu cronometro!");

    //Display que vai mostrar a contagem do tempo
    println!("Contador");

    //Localização dos botões ou funções deles
    println!("Começar");
    println!("Pausar");
    println!("Parar");

    press();
}
