# OneClient 2.12.1 — Offline Fix3

Build independente do OneClient / OneLauncher da Polyfrost para **Windows x64**.

## Incluído nesta versão

- Criação de perfis offline no primeiro acesso ou nas configurações, sem uma conta Microsoft cadastrada.
- Importação de skins classic/slim e capas em PNG, com armazenamento por perfil.
- Busca de skins por nome de usuário e biblioteca local no Wardrobe do jogo.
- Sincronização da aparência entre o launcher e o jogo.
- Correções de cache e redesenho dos avatares e de atualização ao trocar o perfil.
- Resolução da pasta de mods efetivamente usada pela instância.
- Correção do vínculo da skin/capa ao jogador ativo quando o servidor informa um UUID diferente.
- Atualização automática pelo instalador oficial desativada para preservar o componente local.

## Como usar

Instale o arquivo `OneClient-Offline-Fix3-Windows-x64-setup.exe`, crie um perfil em **Add offline** e prepare uma instância **Minecraft 1.8.9 com o bundle OneClient / Ornithe**. Depois, abra **Wardrobe** para importar sua aparência.

Skins: PNG de 64 × 64 ou 64 × 32. Capas: PNG de 64 × 32. A aparência é local; outros clientes não a recebem automaticamente.

## Verificação

O [build 45901138960](https://github.com/lovskme/OneLauncher-buff/actions/runs/45901138960) terminou com sucesso, incluindo testes de autenticação/perfis, validação de PNG, cache/renderização de avatares e instalação do componente. O CI também verificou a instalação silenciosa do pacote Windows e suas bibliotecas de runtime.

**Funcionamento em partida confirmado pelo mantenedor em 27/11/2026.** A confirmação se refere ao uso relatado; não representa uma matriz de testes de todos os servidores ou versões.

## Procedência

| Item | Referência |
| --- | --- |
| Código do launcher | `c7d2cf1f664dad5c8a7aa2ee3f8b7cd077e0d5df` |
| Branch de origem | `offline-profiles` |
| Workflow | `offline_windows.yml` |
| Run do instalador | `45901138960` |
| Artifact original | `15156903530` — `oneclient-offline-windows-x64-installer` |

O executável desta release é o instalador produzido por esse build, apenas renomeado para identificar o Fix3. O arquivo `SHA256SUMS.txt` permite conferir o download.

## Compatibilidade

- Instalador customizado disponível para **Windows x64**.
- Componente de aparência local para **Minecraft 1.8.9 Ornithe**, com o bundle OneClient.
- Serviços e servidores autenticados continuam exigindo login Microsoft.
- Itens pagos do catálogo mantêm suas regras de propriedade.
- Instalador **sem assinatura digital**. Esta distribuição é independente da Polyfrost.
- O trabalho de compatibilidade CrewX não faz parte deste pacote.

Base e créditos: [Polyfrost / OneLauncher](https://github.com/Polyfrost/OneLauncher) e PolyPlus. Código, patch do componente e respectivas licenças estão disponíveis no repositório.
