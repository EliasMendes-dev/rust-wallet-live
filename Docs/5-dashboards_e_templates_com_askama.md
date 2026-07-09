# 5 - Dashboards e Templates com Askama

## Introdução

Nesta etapa, o projeto avançou para uma interface web mais rica usando templates Askama e um dashboard de ativos. O objetivo foi aprender a renderizar páginas HTML com `askama`, criar um painel de controle para ativos e usar filtros personalizados para formatar valores.

## O que eu aprendi

Os principais aprendizados desta fase foram:

- renderizar templates HTML com Askama em Axum;
- criar uma dashboard para exibir ativos do usuário com status e histórico;
- definir filtros Askama personalizados para formatação numérica e de datas;
- montar um formulário de compra de ativos e salvar a compra no servidor;
- usar Tailwind CSS diretamente nos templates para uma interface mais moderna.

## Dependências relevantes

Esta etapa usa as seguintes bibliotecas:

- `askama`: renderização de HTML a partir de structs Rust;
- `axum`: framework para rotas e handlers;
- `serde`: serialização de dados para os templates e para sessões;
- `tower-sessions`: manter a sessão do usuário autenticado;
- `time`: manipulação de datas e formatação de data/hora;
- `sqlx`: persistência de ativos e histórico no banco.

## Estrutura da implementação

A implementação adicionou ou alterou principalmente:

- `src/routes/frontend.rs`: novas rotas para `/assets`, `/logout` e geração do dashboard;
- `templates/assets.html`: template principal do dashboard de ativos;
- `templates/login.html`: página de login estilizada com Tailwind;
- `src/models.rs`: modelagem de `OwnedAsset`, `PurchaseHistory` e filtros para o template;
- `src/auth/user.rs`: extração do usuário autenticado para proteger o dashboard.

## Como o dashboard funciona

### 1. Rota de dashboard

A rota `GET /assets` carrega os ativos do usuário e os ativos disponíveis. Ela usa `AssetsPage` para renderizar o template `assets.html` com:

- `owned_assets`: ativos já comprados pelo usuário;
- `available_assets`: catálogo de ativos disponíveis para compra;
- `user`: dados do usuário autenticado.

### 2. Template Askama

O template `templates/assets.html` exibe:

- um cabeçalho com o nome do usuário;
- a lista de ativos próprios com detalhes de compra;
- um modal de compra para registrar novas posições;
- filtros que formatam números, moedas e datas.

### 3. Filtros personalizados

A pasta `src/routes/frontend.rs` define filtros Askama que formatam:

- valores numéricos com `pretty_number`;
- preços em formato monetário com `currency`;
- ganhos/perdas com `signed_amount`;
- datas legíveis com `human_datetime`.

### 4. Formulário de compra

O formulário `POST /assets` permite ao usuário selecionar um ativo, informar quantidade e preço unitário. O servidor grava a compra e redireciona de volta para `/assets`.

## O que mudou em relação ao passo anterior

Antes desta etapa, a aplicação já tinha login e sessão em HTML. Agora ela também tem:

- uma dashboard de ativos completa;
- interface de compra de ativos;
- uso de templates Askama para renderizar HTML a partir de structs Rust;
- filtros ricos para formatação de conteúdo.

## Observações

O dashboard usa `OwnedAsset.purchase_history()` para iterar sobre o histórico de compras e exibir a evolução de cada posição. A interface também oferece um `logout` simples.

