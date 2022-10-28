# Hoarder

A simple dotfiles manager that make things clearly.

## Installation

```shell
cargo install hoarder
```

## Usage

> example: [plimeor/dotfiles](https://github.com/plimeor/dotfiles)

### Setup

1. Run `hoarder init` to create a `hoarder.json` in current dir
2. Set env `HOARDER` to this dir
3. Update `hoarder.json` to specify the file to be collected, example:

```json
{
  "nvim": {
    ".config/nvim": "~/.config/nvim"
  },
  "zsh": {
    ".zshrc": "~/.zshrc",
    ".zsh_profile": "~/.zsh_profile"
  }
}
```

### Collect

Use `hoarder collect` to collect the specified file and leave a symlink in place, example:

```shell
- dotfiles # you should use env `HOARDER` to specify this dir
  - nvim
    - .config
      - nvim  
  - zsh
    - .zshrc
    - .zsh_profile 
  - hoarder.json # configuration 
```

### Restore

Use `hoarder restore` to copy back files to original location.
