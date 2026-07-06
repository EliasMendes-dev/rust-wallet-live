# Documentação inicial do projeto Rust Wallet Live

## Visão geral
Este projeto é uma API em Rust construída com o framework Axum, com foco em aprender conceitos de backend, autenticação, tratamento de erros e uso de enums.

O projeto foi iniciado durante o curso da DIO e representa o primeiro passo de implementação de uma aplicação simples para controlar ativos financeiros em memória.

## Estado atual do projeto
Até o momento, a aplicação já possui:
- servidor HTTP rodando na porta 3000;
- rotas da API organizadas em módulo próprio;
- armazenamento em memória de ativos usando `HashMap`;
- criação, listagem e atualização de ativos;
- autenticação simples via header `Authorization`;
- tratamento centralizado de erros com `enum`;
- logs básicos com tracing.

## Conceito do vídeo "Primeiros Passos com Enum"
O vídeo introduziu o uso de enums em Rust. No projeto, esse conceito já foi aplicado no módulo de erros através de `AppError`, que define diferentes variantes de falha:
- `MissingAuthorization`
- `InvalidCredentials`
- `AssetDoesNotExist`

Essa implementação permite que o código trate diferentes cenários de forma organizada e expressiva.

## Estrutura do projeto
- [Cargo.toml](../Cargo.toml): definição do pacote e dependências.
- [src/main.rs](../src/main.rs): ponto de entrada da aplicação.
- [src/app.rs](../src/app.rs): configuração do estado global e inicialização do servidor.
- [src/routes/api.rs](../src/routes/api.rs): definição das rotas e handlers da API.
- [src/models.rs](../src/models.rs): modelo `Asset`.
- [src/auth/admin.rs](../src/auth/admin.rs): extractor de autenticação para rotas protegidas.
- [src/error.rs](../src/error.rs): tratamento estruturado de erros com enum.

## Funcionalidades implementadas
### 1. Servidor Axum
A aplicação inicia um servidor HTTP com Axum e expõe a API sob o prefixo `/api`.

### 2. Estado global
O estado da aplicação armazena os ativos em memória por meio de um `Arc<Mutex<HashMap<i64, Asset>>>`.

### 3. Rotas disponíveis
- `GET /api/assets`: lista todos os ativos cadastrados.
- `POST /api/assets`: cria um novo ativo.
- `PATCH /api/assets`: atualiza um ativo existente.

### 4. Autenticação simples
As rotas de criação e atualização exigem um header `Authorization` com o valor `im-the-admin`.

### 5. Tratamento de erros
Os erros retornam respostas JSON com status HTTP apropriados:
- `400 Bad Request` para falta de autorização.
- `401 Unauthorized` para credenciais inválidas.
- `404 Not Found` quando o ativo não existe.

## Como executar
No terminal, na raiz do projeto, execute:

```bash
cargo run
```

A aplicação ficará disponível em `http://localhost:3000`.

## Status do aprendizado
Até aqui, o projeto já consolidou:
- estrutura básica de uma API em Rust;
- uso de módulos e organização de código;
- uso de enums para representar estados/erros;
- autenticação como extractor customizado;
- manipulação de estado compartilhado.

## Próximos passos sugeridos
- adicionar testes;
- persistir dados em banco ou arquivo;
- criar mais endpoints;
- expandir o uso de enums para outras áreas do projeto.
