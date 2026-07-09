# 3 - Modelagem de Usuário e Autenticação

## Introdução
Nesta etapa, o projeto recebeu a modelagem de usuários e um fluxo básico de autenticação. A ideia foi permitir que a aplicação valide credenciais, registre usuários novos automaticamente e mantenha a segurança das senhas usando hashing.

## O que eu aprendi
Nesta fase, os principais aprendizados foram:

- modelagem de usuário em Rust com structs específicas;
- uso de hashing de senhas com a crate `password-auth`;
- lógica de autenticação e registro automático;
- separação entre usuário não autenticado (`UnauthenticatedUser`) e usuário autenticado (`User`);
- isolamento da lógica de autenticação na camada `auth`;
- utilização de templates Askama para renderizar uma página de login;
- integração entre rotas HTML e a camada de persistência.

## Dependências relacionadas
As dependências que entraram no fluxo de autenticação são:

- `password-auth`: geração e verificação de hashes de senha;
- `askama`: renderização de templates HTML para a página de login;
- `serde`: para decodificar formulários JSON/HTML se necessário;
- `sqlx`: para persistência de dados de usuário no PostgreSQL.

## Estrutura adicionada
A partir dessa etapa, o projeto passou a incluir:

- `src/auth/user.rs`: lógica de autenticação, registro e modelagem de usuário;
- `src/routes/frontend.rs`: rota de login que renderiza página HTML e processa o formulário;
- `src/models.rs`: modelagem de `UserRecord` para persistência no banco;
- `src/repository.rs`: consultas SQL de cadastro e busca de usuários.

## Fluxo de autenticação
### 1. Página de login
A rota `GET /login` retorna um template HTML com o formulário de login.

### 2. Submissão do formulário
A rota `POST /login` recebe `username` e `password`.

### 3. Verificação do usuário
O fluxo é:

- busca o usuário pelo nome no banco;
- se o usuário existir, verifica a senha com `password_auth::verify_password`;
- se não existir, registra o usuário automaticamente com a senha hash;
- retorna o nome do usuário na resposta HTML.

## Como funciona a modelagem
### `UnauthenticatedUser`
A struct `UnauthenticatedUser` contém `username` e `password` e tem dois métodos principais:

- `authenticate`: verifica se o usuário existe e se a senha está correta;
- `register`: cria um novo usuário caso o nome ainda não exista.

### `User`
A struct `User` representa o usuário autenticado exposto pelo fluxo de login.

## Persistência de usuários
A camada `Repository` passa a oferecer duas operações de usuário:

- `add_user(username, password_hash)`: insere um novo registro na tabela `users`;
- `get_user_by_name(username)`: consulta um usuário pelo nome.

## Tratamento de erros
A autenticação também estende o enum `AppError` com variantes específicas:

- `UserDoesNotExist`;
- `UsernameTaken`;
- `InvalidCredentials`.

Esses erros são convertidos em respostas HTTP apropriadas e ajudam a manter o fluxo mais claro.

## O que mudou na aplicação
Antes desse passo, o projeto apenas validava um header de admin fixo. Agora ele já possui:

- fluxo de login real;
- registro de usuário;
- hash seguro de senha;
- página de login em HTML;
- persistência de usuários em banco.

## Observações
O registro é implícito: ao fazer login com um usuário que ainda não existe, o sistema cria o usuário automaticamente.

