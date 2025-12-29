# convertor
CLI simples e multiplataforma para conversão de áudio/vídeo usando **ffmpeg** — feito como projeto de estudo em **Rust** (learning-by-doing), com foco em design limpo, modularização e pipeline de releases.

> Motivação pessoal: receber áudio/vídeo no WhatsApp/iMessage e precisar converter/compatibilizar para apps como Signal — sem ficar abrindo GUI, sites aleatórios ou ferramentas suspeitas.

## O que este projeto faz

- Converte arquivos de mídia (áudio/vídeo) chamando `ffmpeg` para realizar o trabalho
- Aplica **presets** prontos (WhatsApp, Signal, iMessage, etc.)
- Permite overrides pela CLI (audio-only, video-only, format, strip metadata)
- Gera scripts de **autocomplete** para shells (bash/zsh/fish/powershell)
- Tem comando `doctor` para validar dependências

## Requisitos

- `ffmpeg` instalado e disponível no `PATH`

### macOS (Homebrew)
```bash
brew install ffmpeg
```

### Windows
```
winget install --id=Gyan.FFmpeg --source=winget
```

### Linux Ubuntu
```
sudo apt update
sudo apt install -y ffmpeg
````

## Instalação
**Rodar localmente (dev)**
```
cargo run -- --help
```

**Build release local**
```
cargo build --release
./target/release/convertor --help
```

## Uso
Converter com preset (ex: áudio compatível para Signal)
```
convertor convert input.mp4 output.ogg --preset signal --no-metadata
```

Forçar sobrescrita do arquivo de saída
```
convertor convert input.wav output.opus --preset audio-opus --force
```

Converter apenas áudio
```
convertor convert input.mp4 output.opus --audio-only --format opus
```

Remover metadados
```
convertor convert input.mp4 output.mp4 --no-metadata
```

Diagnóstico do sistema (ffmpeg)
```
convertor doctor
```

## Presets
Os presets são atalhos para “planos de conversão” com container/codecs/bitrate/metadados.

Exemplos (pode evoluir):
- whatsapp → áudio em ogg/opus com bitrate baixo e metadata removida
- signal → áudio em ogg/opus com bitrate baixo e metadata removida
- imessage → áudio em m4a/aac (perfil mais comum)
- archive → preset mais “qualidade/arquivamento”
- audio-aac → áudio em AAC
- audio-opus → áudio em Opus

> **Observação:** presets e defaults não são “verdades absolutas”. São escolhas para interoperabilidade. Se você tiver uma opação melhor, mande PR/issue.

## Autocomplete

Gerar autocomplete (imprime no stdout):
```
convertor completion bash
convertor completion zsh
convertor completion fish
convertor completion powershell
```

**Exemplo (bash)**
```
convertor completion bash > /usr/local/etc/bash_completion.d/convertor
```

(Os paths variam por OS/shell — a ideia é só gerar e instalar onde seu shell carrega completions.)

## Como funciona o projeto

Este projeto foi construído seguindo uma linha de aprendizado bem direta:
1.	Fazer um CLI funcional com clap
2.	Modelar um “plano de conversão” (domain) e transformar isso em args do ffmpeg
3.	Adicionar presets e overrides
4.	Adicionar doctor, autocomplete e --force
5.	Refatorar para uma arquitetura modular e sustentável

Estrutura final
```
├── src
│   ├── cli
│   │   ├── app.rs
│   │   ├── completion.rs
│   │   ├── doctor.rs
│   │   └── mod.rs
│   ├── ffmpeg
│   │   ├── mod.rs
│   │   └── runner.rs
│   ├── main.rs
│   └── plan
│       ├── codec.rs
│       ├── conversion_plan.rs
│       ├── format.rs
│       ├── mod.rs
│       └── preset.rs
```

**Responsabilidades**
- plan/ → domínio (Format, Preset, Codecs, ConversionPlan)
- ffmpeg/ → execução do ffmpeg (runner)
- cli/ → definição da interface (clap) e comandos auxiliares
m- ain.rs → orquestração (parse → criar plan → validar → executar)

A intenção foi de deixar o `main.rs` quase “declarativo”: ele coordena, mas não implementa detalhes.

**Avisos importantes**
- Conversão de mídia pode ser lenta dependendo do codec/resolução.
- A qualidade final depende do preset/bitrate/codecs escolhidos.

## License
MIT