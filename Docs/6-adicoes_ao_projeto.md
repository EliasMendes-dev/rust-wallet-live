# 6 - Adições ao projeto

## Uso de `tower-sessions` em vez de JWT

### Motivo

O uso de JWT exigiria dependências adicionais e compilação de bibliotecas nativas, o que é pesado em ambientes com pouco espaço e sem suporte fácil a `CMake` e outras toolchains.

### Como foi feito

A autenticação foi implementada com `tower-sessions` usando `SessionManagerLayer` e armazenamento em memória (`MemoryStore`). O estado de sessão guarda um `UserSession` contendo `id` e `username` do usuário.

### Resultado

A aplicação passou a manter o usuário autenticado entre requisições sem precisar de tokens JWT. Isso simplifica o ambiente de desenvolvimento e evita dependências externas pesadas.

## Migração de `double` para `decimal`

### Motivo

Valores financeiros não devem ser representados com ponto flutuante (`double`) devido a problemas de precisão e arredondamento. Para garantir cálculos corretos, foi usada a biblioteca `rust_decimal` e o tipo `decimal` do Postgres.

### Como foi feito

- Alterei a coluna do banco para tipo `numeric`/`decimal` nas migrations.
- Atualizei o modelo em Rust para usar `rust_decimal::Decimal` em vez de `f64`.
- Adaptei a renderização e os cálculos de valores para manter formatação adequada.

### Resultado

Agora os valores de ativos e transações usam uma representação exata, evitando erros de precisão e comportamentos incorretos em operações financeiras.

## Tela de cadastro

### Motivo

Antes do ajuste, a aplicação não suportava criação de conta pelo usuário final. Era necessário adicionar um fluxo de registro com validação de entrada e feedback claro.

### Como foi feito

- Adicionei `templates/register.html` para exibir o formulário.
- Criei a rota `/register` em `src/routes/frontend.rs` com GET e POST.
- No backend, `src/auth/user.rs` valida `username`, `email` e `password`.
- Mensagens de erro são exibidas na própria página quando o usuário fornece email inválido, username vazio ou email já existente.

### Resultado

O usuário agora pode criar uma conta diretamente na aplicação. O formulário preserva os dados digitados em caso de erro e mostra mensagens amigáveis em inglês sobre a causa do problema.