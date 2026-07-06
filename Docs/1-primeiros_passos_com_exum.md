# 1 - Primeiros Passos com Enum

## Introdução
Este documento registra o que foi aprendido e implementado no primeiro passo do projeto Rust Wallet Live, com foco no uso de enums em Rust e na criação de uma API simples com Axum.

## O que eu aprendi
Durante esta etapa, foram trabalhados os seguintes conceitos:

- organização de um projeto Rust em módulos;
- criação de uma API básica com Axum;
- uso de `enum` para representar erros de forma clara e segura;
- implementação de autenticação simples por header HTTP;
- uso de `HashMap` para armazenar dados em memória;
- estruturação de handlers e rotas com estado compartilhado.

## Dependências utilizadas
As bibliotecas principais do projeto são:

- `axum`: framework web para criar a API;
- `tokio`: runtime assíncrono para executar o servidor;
- `serde`: serialização e desserialização de JSON;
- `serde_json`: manipulação de JSON;
- `thiserror`: geração de erros com `enum` e mensagens legíveis;
- `color-eyre`: tratamento de erros mais amigável;
- `tracing` e `tracing-subscriber`: logs da aplicação.

## Estrutura do projeto
A aplicação foi organizada em módulos para deixar o código mais limpo:

- `src/main.rs`: ponto de entrada;
- `src/app.rs`: configura o estado global e inicia o servidor;
- `src/routes/api.rs`: define as rotas e handlers;
- `src/models.rs`: define o modelo `Asset`;
- `src/auth/admin.rs`: implementa autenticação simples;
- `src/error.rs`: centraliza os tipos de erro com `enum`.

## Fluxo da aplicação
### 1. Inicialização
Ao iniciar o servidor, o programa:
- cria o estado global da aplicação;
- configura o logging;
- sobe o servidor na porta 3000.

### 2. Rotas disponíveis
A API possui três rotas principais:

- `GET /api/assets`: retorna todos os ativos salvos em memória.
- `POST /api/assets`: cria um novo ativo, desde que a autenticação esteja correta.
- `PATCH /api/assets`: atualiza um ativo existente, também protegido por autenticação.

### 3. Autenticação
As rotas de criação e atualização exigem o header `Authorization` com o valor:

```text
im-the-admin
```

Se o valor estiver incorreto ou ausente, a API retorna erro.

## O que faz o projeto
O projeto simula um pequeno sistema de carteira, onde cada ativo possui:

- `id`;
- `name`;
- `unit_value`.

Esses ativos ficam armazenados temporariamente em memória durante a execução da aplicação.

## Uso de enum no projeto
O ponto central do vídeo foi aprender como usar `enum` para organizar erros. No projeto, isso foi aplicado em `AppError` com as variantes:

- `MissingAuthorization`
- `InvalidCredentials`
- `AssetDoesNotExist`

Essa abordagem deixa o código mais legível e permite responder com status HTTP e mensagens mais claras.

## Aprendizados principais
Até esta etapa, os principais aprendizados foram:

- como estruturar uma API simples em Rust;
- como separar responsabilidades em módulos;
- como usar `enum` para representar diferentes cenários de erro;
- como trabalhar com estado compartilhado em aplicações assíncronas;
- como criar um fluxo básico de autenticação em uma API.

## Próximos passos sugeridos
- adicionar testes;
- persistir os ativos em banco ou arquivo;
- criar novos endpoints;
- melhorar a organização do código e a documentação.
