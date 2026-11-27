# Guia do OneLauncher buff

[← Voltar ao início](../README.md)

## Instalar e criar um perfil

Use o [instalador Offline Fix3](https://github.com/lovskme/OneLauncher-buff/releases/download/2.12.1-offline-fix3/OneClient-Offline-Fix3-Windows-x64-setup.exe) no Windows x64. Esta build independente não tem assinatura digital.

No primeiro acesso, preencha o nome e escolha **Add offline**. Se o launcher já está configurado, abra **Settings → Accounts → Add offline**. O nome aceita de 3 a 16 letras ASCII, números e sublinhado. Nomes duplicados são rejeitados e o primeiro perfil criado se torna o padrão.

Perfis offline funcionam no singleplayer e em servidores que permitem jogadores offline. Serviços Microsoft e servidores autenticados continuam exigindo uma conta válida.

## Preparar a instância

Crie uma instância **Minecraft 1.8.9 com o bundle OneClient / Ornithe**. A build inclui um componente local baseado no PolyPlus e precisa das dependências normais desse bundle.

Faça o primeiro download do jogo, Java, bibliotecas, assets e mods com internet. Depois, os recursos já instalados podem ser usados sem conexão, conforme o modo de jogo.

O suporte descrito aqui é para **1.8.9 Ornithe**. Uma instância vanilla ou Forge não usa o componente local.

## Escolher skin e capa

Selecione o perfil offline e entre em **Wardrobe**.

| Ação | Resultado |
| --- | --- |
| Importar skin PNG clássica | Salva uma skin com modelo classic para o perfil |
| Importar skin PNG slim | Salva a skin e seleciona o modelo slim |
| Buscar por nome de usuário | Baixa a skin pública do nome informado e a guarda localmente |
| Importar capa PNG | Salva a capa local do perfil |
| Remover skin ou capa | Remove a textura selecionada |

Skins precisam ser PNG de **64 × 64** ou **64 × 32** pixels. Capas precisam ser PNG de **64 × 32**. O limite é **1 MiB** por arquivo. Arquivos inválidos são rejeitados.

A busca por nome depende de internet. Uma skin já importada ou baixada permanece disponível localmente.

No jogo, o Wardrobe compartilha a aparência do mesmo perfil. A biblioteca **Your skins** mantém os controles de importar, vestir, renomear, favoritar, escolher modelo e excluir. O drawer de capas contém os controles locais de PNG.

## Onde ficam os arquivos?

Dentro da pasta de dados do launcher, em:

```text
offline-appearance/<UUID-do-perfil>/
```

| Arquivo | Uso |
| --- | --- |
| `skin.png` | Skin selecionada |
| `cape.png` | Capa selecionada |
| `model.txt` | Modelo classic ou slim |
| `library.json` e `library/` | Metadados e PNGs da biblioteca local no jogo |

Antes de reinstalar ou mudar de computador, guarde uma cópia da pasta de dados se quiser preservar os perfis e as aparências.

## Resolver problemas

| Situação | O que conferir |
| --- | --- |
| Skin não aparece no jogo | Perfil offline selecionado, build Fix3 instalada e instância 1.8.9 com bundle OneClient / Ornithe |
| Erro de componente ausente | Repare a instalação do bundle OneClient nessa instância; o launcher interrompe o lançamento se não encontrar o componente necessário |
| PNG recusado | Formato PNG real, dimensões corretas e tamanho dentro do limite |
| Busca por nome falha | Conexão, nome válido e existência de skin personalizada para esse jogador |
| Outros jogadores não veem a skin | Comportamento esperado: a aparência é local, sem sincronização para outros clientes |
| Servidor pede autenticação | Entre com uma conta Microsoft válida; o perfil offline não fornece autenticação |
| Dúvida sobre a versão instalada | Reinstale usando o arquivo da release Offline Fix3; o atualizador oficial está desativado nesta build |

Os logs do launcher registram a pasta usada para instalar o componente. No jogo, procure por `polyplus/local-appearance` nos logs para conferir sua inicialização.

## Conferir o download

Baixe também `SHA256SUMS.txt` na [release](https://github.com/lovskme/OneLauncher-buff/releases/tag/2.12.1-offline-fix3). No PowerShell, dentro da pasta do instalador:

```powershell
Get-FileHash .\OneClient-Offline-Fix3-Windows-x64-setup.exe -Algorithm SHA256
```

Compare o hash retornado com a linha do mesmo arquivo em `SHA256SUMS.txt`.

## Atualizações e código

Esta build não instala automaticamente versões oficiais que substituiriam o Wardrobe local. Novas versões do fork devem ser obtidas nas releases deste repositório.

A release Fix3 corresponde ao commit [`c7d2cf1f`](https://github.com/lovskme/OneLauncher-buff/commit/c7d2cf1f664dad5c8a7aa2ee3f8b7cd077e0d5df), na branch `offline-profiles`. O trabalho de CrewX está em uma branch separada e não integra este instalador.
