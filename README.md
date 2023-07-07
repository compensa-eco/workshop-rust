# Workshop de Rust - Compensa

Bem-vindo ao Workshop de Rust da Compensa! Este projeto foi criado com o objetivo de iniciar o estudo e a aplicação da linguagem de programação Rust em nossa empresa. Neste repositório, você encontrará todos os recursos necessários para acompanhar o workshop e começar a desenvolver em Rust.

## Sobre Rust

Rust é uma linguagem de programação de sistemas de alto desempenho, focada em segurança, concorrência e eficiência. Com sua sintaxe moderna e recursos poderosos, Rust permite o desenvolvimento de software confiável e eficiente, sem comprometer a segurança. Combinando controle de memória de baixo nível e abstrações de alto nível, Rust oferece um ambiente amigável para desenvolver uma ampla gama de aplicativos, desde sistemas operacionais e drivers de dispositivos até servidores e aplicações Web.

## Instalação

Para instalar o Rust em seu sistema, siga as instruções do site oficial: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## Pré-requisitos

Antes de começar com o workshop, certifique-se de ter as seguintes dependências instaladas em seu sistema:

- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) - O Cargo é o gerenciador de pacotes e construtor de projetos para a linguagem Rust. Verifique se o Cargo está instalado junto com o Rust.

## Estrutura do Projeto

```
workshop-rust/
  ├── src/
  │   ├── main.rs
  │   └── lib.rs
  ├── Cargo.toml
  └── README.md
```

- **src/main.rs**: Este é o arquivo principal do projeto. Aqui você poderá escrever seu código em Rust e executá-lo.
- **src/lib.rs**: Este é um arquivo opcional que pode ser utilizado para criar uma biblioteca Rust.
- **Cargo.toml**: O arquivo Cargo.toml é responsável por gerenciar as dependências e as configurações do projeto em Rust.
- **README.md**: Este arquivo contém informações importantes sobre o projeto e seu objetivo.

## Executando o Projeto

Siga as etapas abaixo para executar o projeto em sua máquina local:

1. Clone este repositório para o seu ambiente local.

   ```
   git clone https://github.com/seu-usuario/compensa-workshop-rust.git
   ```

2. Acesse o diretório do projeto.

   ```
   cd compensa-workshop-rust
   ```

3. Compile e execute o projeto usando o Cargo.

   ```
   cargo run
   ```

   Isso compilará o código e executará o programa. Sinta-se à vontade para fazer as alterações necessárias em src/main.rs para explorar as funcionalidades da linguagem Rust.

## Recursos Adicionais

Aqui estão alguns recursos adicionais que podem ajudar você a se familiarizar mais com a linguagem Rust:

- [The Rust Programming Language](https://doc.rust-lang.org/book/) - O livro oficial de Rust, também conhecido como "The Rust Book", é um guia abrangente e bem escrito para aprender Rust desde o início.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Rust by Example fornece exemplos práticos que demonstram os conceitos e recursos da linguagem Rust.
- [Crates.io](https://crates.io/) - O repositório oficial de pacotes do ecossistema Rust. Aqui você pode encontrar várias bibliotecas úteis para seus projetos em Rust.

Divirta-se explorando o Rust! Se você tiver alguma dúvida durante o workshop, sinta-se à vontade para perguntar aos instrutores. Aproveite o aprendizado e comece a construir coisas incríveis com Rust!
