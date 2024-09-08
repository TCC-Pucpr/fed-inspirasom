### Como rodar o programa

Pode usar qualquer sistema operacional, porém para que
você consiga compilar e executar o programa, você vai precisar ter instalado no seu computador:

- Node e npm
- Rust
- Tauri, usando cargo para fazer a instalação

Se voce estiver num linux:

#### Fedora

``` shell
sudo dnf install alsa-lib-devel
```

#### Ubuntu

``` shell
sudo apt install libudev-dev
```

Para rodar, entre no diretório raiz do projeto, a seguir você irá
executar o comando `npm install` para baixar todas as dependências.
Em seguida, executar `npm run tauri dev`.

### Contributing

Para incrementar a versão do projeto, por favor use o seguinte comando:

``` shell
npm run ver p
```

#### Modificando banco de dados

Pré requisitos:

- ``sea-orm-cli``: Instala com ``` cargo install sea-orm-cli```

O banco de dados é gerado dentro do `src-tauri/persistence/migrations`.

Para fazer uma nova atualizacao ao banco de dados, entre no diretorio `src-tauri/persistence` e execute o comando:

``` shell
sea-orm-cli migrate generate [nome da migracao]
```

Nome deve ser em snake case ou entre aspas " e sem o [ ]

Um novo arquivo sera gerado dentro do crate `migrations` com o nome sendo a data local, e o nome escolhido.

Dentro desse arquivo modifique as funcoes up e down, fazendo as mudancas para nova versao e rollback.
[Mais detalhes sobre migracao com SeaORM](https://www.sea-ql.org/SeaORM/docs/migration/writing-migration/)

Para executar as novas mudancas, primeiro edite o arquivo `src-tauri/persistence/.env` e adicione o
caminho absoluto para o arquivo (se nao tiver apenas crie um de texto vazio com final .db)

```shell
sea-orm-cli migrate up
```

O database será atualizado, porém ainda é necessario atualizar os structs de cada tabela dentro do
`src-tauri/persistence/entity`. Para isso, dentro do diretorio `src-tauri/persistence` rode:

```shell
sea-orm-cli generate entity -o entity/src/entities
```

[Mais detalhes sobre rodando as migracoes com SeaORM](https://www.sea-ql.org/SeaORM/docs/migration/running-migration/)

OBS: Voce pode usar configuracoes de run se voce estiver usando algum produto da Jetbrains e nao
quer rodar esses comandos na mao.

### Código do Arduino

[link](https://github.com/TCC-Pucpr/arduino-inspirasom/tree/9-criacao-da-branch-do-prototipo).
