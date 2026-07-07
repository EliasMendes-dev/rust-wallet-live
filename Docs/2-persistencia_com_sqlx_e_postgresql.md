# 2 - Persistência com SQLx e PostgreSQL

## Introdução
Nesta etapa, o projeto deixou de depender apenas de memória para armazenar os ativos e passou a usar um banco de dados PostgreSQL com SQLx. A ideia principal foi aprender como conectar a aplicação a um banco, executar consultas assíncronas e organizar a camada de acesso a dados.

## O que aprendi
Nesta fase, os principais aprendizados foram:

- conexão com PostgreSQL usando SQLx;
- uso de `PgPool` para gerenciar conexões reutilizáveis;
- criação de uma camada de repositório para isolar consultas do banco;
- uso de migrações para criar e manter a estrutura do banco;
- leitura de variáveis de ambiente com `dotenvy`;
- integração de erros do banco no fluxo da API;
- testes de rotas com banco real usando SQLx.

## Dependências adicionadas
Para essa etapa, foram adicionadas ou passadas a usar as seguintes dependências:

- `sqlx`: acesso ao PostgreSQL e execução de queries;
- `dotenvy`: carregar variáveis de ambiente a partir de um arquivo `.env`;
- `tokio`: suporte assíncrono para operações de banco;
- `serde` e `serde_json`: manipulação de JSON nas rotas;
- `thiserror`: integração de erros de banco no `AppError`.

## Estrutura da nova implementação
A partir desta etapa, o projeto passou a ter uma organização mais forte:

- `src/app.rs`: agora cria e mantém uma conexão com o banco via `PgPool`;
- `src/repository.rs`: camada responsável por listar, criar e atualizar ativos no banco;
- `src/routes/api.rs`: rotas passam a usar o repositório em vez de manipular um `HashMap` em memória;
- `src/error.rs`: agora inclui o erro do banco como uma variante de `AppError`.

## Como a persistência funciona
### 1. Carregamento da configuração
Ao iniciar a aplicação, o código carrega a variável `DATABASE_URL` usando `dotenvy`.

Essa variável aponta para a instância do PostgreSQL, por exemplo:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/wallet_db
```

### 2. Criação da conexão
O estado global da aplicação cria um `PgPool`, que é um pool de conexões reutilizáveis para o banco.

### 3. Repositório
O repositório encapsula as operações de persistência:

- `list_assets()`: consulta todos os ativos;
- `create_asset()`: insere um novo ativo e retorna o objeto criado;
- `update_asset()`: atualiza um ativo identificado pelo `id`.

### 4. Uso nas rotas
As handlers das rotas chamam o repositório para executar as operações no banco. Isso faz a API trabalhar com dados reais, em vez de dados temporários em memória.

## Migrações
As migrações foram usadas para criar a estrutura inicial do banco. Com SQLx, a aplicação pode rodar as migrações com o comando:

```bash
cargo sqlx migrate run
```

A tabela principal do projeto é a tabela `assets`, com colunas como:

- `id`
- `name`
- `unit_value`

## Fluxo da aplicação com banco
1. O servidor inicia.
2. O app lê a configuração do banco.
3. O pool de conexões é criado.
4. A requisição chega na rota correspondente.
5. O handler usa o `Repository` para executar a operação no PostgreSQL.
6. O resultado é retornado como JSON para o cliente.

## Tratamento de erros
Com a persistência, novos cenários de falha passaram a existir, como problemas de conexão ou erros de execução SQL. Esses problemas foram tratados centralmente no enum `AppError`, que agora conta com a variante:

- `Database(#[from] sqlx::Error)`

Isso faz com que falhas de banco retornem uma resposta adequada para o cliente.

## Testes
A camada de rotas também passou a ter testes com SQLx. Os testes rodam com um banco temporário e usam fixtures para garantir cenários previsíveis de criação, leitura e atualização de ativos.

## O que mudou em relação ao passo anterior
No primeiro passo, os ativos ficavam em memória. Agora, com persistência:

- os dados sobrevivem ao reinício da aplicação;
- a API passa a trabalhar com um banco real;
- o projeto ganha mais semelhança com uma aplicação backend real.

## Aprendizados principais
Até esta etapa, o projeto consolidou:

- integração com banco relacional;
- uso de SQLx em Rust;
- persistência de dados via PostgreSQL;
- organização em camadas com repositório;
- uso de migrações e testes com banco.

## Próximos passos sugeridos
- adicionar autenticação mais robusta;
- criar endpoints de remoção de ativos;
- separar melhor o modelo de domínio da camada de banco;
- adicionar docker-compose para subir o banco junto com a aplicação.
