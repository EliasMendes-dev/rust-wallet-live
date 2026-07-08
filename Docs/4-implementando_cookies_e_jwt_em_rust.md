# 3 - Implementando Cookies e JWT em Rust

## Introdução

Este documento descreve a etapa em que o projeto passou a usar sessões baseadas em cookie para autenticação de usuário em Rust. O foco foi aprender como criar uma sessão segura com `tower-sessions` e como armazenar os dados do usuário na sessão.

## O que eu aprendi

Nesta etapa, os principais aprendizados foram:

- uso de cookies de sessão para manter o usuário autenticado;
- configuração de `tower-sessions` no Axum para gerenciar sessões;
- armazenamento de dados de usuário serializáveis na sessão;
- proteção de rotas via extração de usuário autenticado;
- transição do fluxo de login em HTML para comportamento de sessão.

## Dependências relevantes

As bibliotecas utilizadas nessa etapa incluem:

- `tower-sessions`: gerencia sessões via cookie e sessão interna;
- `axum`: framework para definir rotas e handlers;
- `askama`: renderiza a página de login em HTML;
- `serde`: serialização e desserialização de dados na sessão;
- `password-auth`: hash e verificação segura de senha;
- `sqlx`: persistência de usuários e ativos no PostgreSQL.

## Estrutura da implementação

A partir desta etapa, a aplicação passa a usar:

- `src/app.rs`: configura o `SessionManagerLayer` com expiração de sessão;
- `src/routes/frontend.rs`: define a rota de login e salva o usuário na sessão;
- `src/auth/user.rs`: extrai o `User` da sessão e também implementa autenticação/registro;
- `src/auth/mod.rs`: expõe os módulos de autenticação.

## Como a sessão funciona

### 1. Configuração do servidor

No `src/app.rs`, foi criado um `MemoryStore` e um `SessionManagerLayer`:

- `MemoryStore::default()` mantém as sessões em memória;
- `Expiry::OnInactivity(Duration::minutes(10))` define expiração após 10 minutos inativos.

O `SessionManagerLayer` é aplicado ao router principal.

### 2. Fluxo de login

O formulário de login em `src/routes/frontend.rs` envia um `POST /login` com `username` e `password`.

O handler de login:

- tenta autenticar o usuário existente;
- se o usuário não existir, registra um novo usuário com hash de senha;
- grava o `UserSession` no objeto `Session`.

Esse `UserSession` contém:

- `id` do usuário;
- `username`.

### 3. Persistência do estado do usuário

A sessão é armazenada no cookie enviado ao navegador, e o servidor mantém os dados de sessão em memória.

Quando o usuário faz requisições subsequentes, o middleware de sessão reconstrói o estado a partir do cookie e permite extrair o usuário autenticado.

### 4. Proteção de rotas

A implementação de `FromRequestParts<AppState>` para `User` em `src/auth/user.rs` permite que handlers recebam `User` ou `Option<User>` diretamente:

- se o cookie de sessão estiver presente e válido, o usuário é carregado;
- caso contrário, a requisição é rejeitada com `AppError::MissingAuthorization` ou redirecionada para `/login`.

## JWT no contexto do projeto

Embora o vídeo aborde cookies e JWT, o código atual está usando a abordagem de sessão via cookie com `tower-sessions`.

Esse passo foca na autenticação por sessão. O uso de JWT pode ser uma evolução natural futura para um fluxo estateless, mas ainda não há código JWT explicitamente no repositório.

## O que mudou em relação ao passo anterior

Antes dessa etapa, o projeto já tinha modelagem de usuário e autenticação básica com hash de senhas, mas não havia:

- sessão gerenciada por cookie;
- extração de usuário autenticado via `FromRequestParts`;
- armazenamento de sessão no navegador.

Agora, o login mantém o estado do usuário entre requisições e permite proteger páginas com sessão.

## Próximos passos sugeridos

- migrar de `MemoryStore` para uma store persistente (Redis, banco ou arquivo);
- implementar JWT para autenticação stateless;
- adicionar logout e expiração explícita de sessão;
- proteger rotas de API com autorização por papel.
