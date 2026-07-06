# Rust Wallet Live

Este projeto é uma API em Rust construída com Axum para aprender os fundamentos de backend, rotas HTTP, autenticação simples, tratamento de erros e uso de enums.

## O que o projeto faz
A aplicação controla ativos financeiros em memória através de uma API REST simples com os seguintes endpoints:

- `GET /api/assets`: lista os ativos cadastrados.
- `POST /api/assets`: cria um novo ativo.
- `PATCH /api/assets`: atualiza um ativo existente.

## Arquitetura básica
O projeto está organizado em módulos para separar responsabilidades:

- `src/main.rs`: ponto de entrada.
- `src/app.rs`: configuração do servidor e estado global.
- `src/routes/api.rs`: definição das rotas e handlers.
- `src/models.rs`: estrutura do modelo `Asset`.
- `src/auth/`: autenticação simples.
- `src/error.rs`: tratamento centralizado de erros.

## Como executar
No terminal, na raiz do projeto, rode:

```bash
cargo run
```

A API ficará disponível em `http://localhost:3000`.

## Documentação
- [Docs/1-primeiros_passos_com_exum.md](Docs/1-primeiros_passos_com_exum.md): documentação detalhada do primeiro passo do projeto, incluindo aprendizados, dependências, fluxo e estrutura.
