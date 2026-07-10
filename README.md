# Rust Wallet Live

## O que o projeto faz

Este projeto é uma aplicação de carteira financeira simples construída em Rust. Ele permite que o usuário se registre, faça login e visualize ativos disponíveis e ativos próprios. A autenticação é baseada em sessão no servidor, e os dados transacionais usam `decimal` para preservar precisão financeira.

## Demonstração

Abaixo estão dois exemplos do fluxo da aplicação:

- Fluxo de login, cadastro e autenticação bem-sucedida:

  ![Fluxo de login e cadastro](assets/login_e_cadastro_veloz.gif)

- Adição de ativos com compra de valor menor e maior:

  ![Adicionando ativos](assets/assets_veloz.gif)

## Como executar a aplicação

1. Crie um arquivo `.env` com a variável `DATABASE_URL` apontando para o banco PostgreSQL:

```env
DATABASE_URL=postgres://usuario:senha@localhost/nome_do_banco
```

2. Execute as migrations:

```bash
cargo sqlx migrate run
```

3. Inicie a aplicação:

```bash
cargo run
```

A aplicação será iniciada em `http://0.0.0.0:3000`.

## Quais tecnologias foram usadas

- Rust
- Axum (web framework)
- Askama (templates HTML)
- SQLx (acesso a banco de dados Postgres)
- tower-sessions (sessões em memória)
- rust_decimal (valores financeiros precisos)
- password_auth (hash de senhas)
- dotenvy (carregamento de variáveis de ambiente)
- tracing (logs)

## Qual melhoria você implementou

- Adição de tela de cadastro com `username`, `email` e `password`.
- Login com validação de `email` e `password`.
- Tratamento de erro inline nas páginas de login e registro, exibindo mensagens como `Invalid Credentials` e `The provided email is not valid`.
- Validação de email no backend e remoção de mensagens de erro internas do usuário.
- Uso de `tower-sessions` em vez de JWT por limitações de dependências no ambiente local.
- Migração de valores financeiros de `double` para `decimal` para evitar perda de precisão.

## Como testar sua versão

Execute a suíte de testes do Rust:

```bash
cargo test
```

Caso seja necessário reexecutar as migrations em um banco limpo:

```bash
cargo sqlx migrate revert
cargo sqlx migrate run
```

## O que você aprendeu durante o desafio

- Como implementar autenticação baseada em sessão com `tower-sessions` em Rust.
- Por que `decimal` é mais adequado para valores financeiros do que `double` em bancos e em aplicação.
- Como renderizar formulários com erros inline usando Askama.
- Como limitar mensagens de erro expostas ao usuário para melhorar a segurança da aplicação.