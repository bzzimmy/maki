<img src="./banner.png">

My personal fork of [maki](https://github.com/tontinton/maki), an AI coding agent optimized for minimal use of context tokens, while providing a great user experience.

This fork tracks upstream and regularly merges in improvements. It only carries a few of my own tweaks:

* Reasoning previews shown by default, toggle with `Alt+T`.
* Cycle thinking levels with `Shift+Tab`, preference is remembered.
* OpenAI: browser login alongside device auth (`maki auth login openai`).
* OpenAI: Codex subscription fast mode.

Everything else is upstream — see the [upstream README](https://github.com/tontinton/maki) and the [official docs](https://maki.sh/docs) for features, providers, and configuration.

## Installation

Build from source (default branch is `personal`):

```sh
cargo install --locked --git https://github.com/bzzimmy/maki.git --branch personal maki
```

Or grab a pre-built binary from the [upstream releases](https://github.com/tontinton/maki/releases/latest).
