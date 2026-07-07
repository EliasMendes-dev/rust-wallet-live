# Rust Wallet Live

Este projeto é uma API em Rust construída com Axum para aprender os fundamentos de backend, rotas HTTP, autenticação simples, tratamento de erros, uso de enums e persistência com PostgreSQL.

## O que o projeto faz
A aplicação controla ativos financeiros por meio de uma API REST simples com os seguintes endpoints:

- `GET /api/assets`: lista os ativos cadastrados.
- `POST /api/assets`: cria um novo ativo.
- `PATCH /api/assets`: atualiza um ativo existente.

## Arquitetura básica
O projeto está organizado em módulos para separar responsabilidades:

- `src/main.rs`: ponto de entrada.
- `src/app.rs`: configuração do servidor e conexão com o banco.
- `src/routes/api.rs`: definição das rotas e handlers.
- `src/models.rs`: estrutura do modelo `Asset`.
- `src/repository.rs`: camada de acesso a dados com SQLx.
- `src/auth/`: autenticação simples.
- `src/error.rs`: tratamento centralizado de erros.

## Persistência
A partir desta etapa, os ativos são salvos em um banco PostgreSQL com SQLx, usando um pool de conexões e migrações.

## Como executar
No terminal, na raiz do projeto, rode:

```bash
cargo sqlx migrate run
cargo run
```

Antes disso, certifique-se de ter o PostgreSQL rodando e de definir a variável `DATABASE_URL` no ambiente ou em um arquivo `.env`.

A API ficará disponível em `http://localhost:3000`.

## Documentação
- [Docs/1-primeiros_passos_com_exum.md](Docs/1-primeiros_passos_com_exum.md): documentação do primeiro passo, com aprendizados, dependências, fluxo e estrutura.
- [Docs/2-persistencia_com_sqlx_e_postgresql.md](Docs/2-persistencia_com_sqlx_e_postgresql.md): documentação detalhada da implementação com SQLx, PostgreSQL e migrações.

# docker exec -it rust-wallet-live-db-1 psql -U postgres -d postgres