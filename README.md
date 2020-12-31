# History Search (hsearch)

> 📜 + 🔎 = ❤️

`hsearch` is a context-aware Bash/Zsh eternal history manager.
It facilitates the seamless collaboration between your past and your future by keeping track of bash/zsh history in a plain text file (`~/.zsh_eternal_history` or `~/.bash_eternal_history`) and providing a fuzzy search capability. It's written in Rust, so it's blazingly fast 🦀
![](assets/hsearch.gif)

## 💻 Usage

```sh
# search eternal command history using fuzzy search / will copy to clipboard
hsearch

# search commands executed on the current directory
hsearch --here

# search commands executed on the current directory today
hsearch --here --today

# search commands executed on the current directory today
hsearch --here --yesterday

# edit eternal history file
vim ~/.bash_eternal_history # or ~/.zsh_eternal_history for zsh
```

## 📜 Installation

### Getting Started

1. Install the **history_search** binary:

   #### Install Latest Version

   ##### From prebuilt binary, with Shell:

   ```sh
   curl -fsSL http://albert.tips/hsearch/install.sh | bash
   ```

   ##### From source

   ```sh
   git clone git@github.com:grepinsight/history_search.git && \
      cd history_search && \
      cargo install --path .
   ```

2. Add this line to bashrc or zshrc

   ```sh
   # in bash
   eval "$(hsearch init bash)"

   # in zsh
   eval "$(hsearch init zsh)"
   ```

## 🚀 Advanced Usage

### Using hsearch in a remote machine

Clipboard can be configured to work with remote machines.

See <https://seancoates.com/blogs/remote-pbcopy> for details

```sh
hsearch -s 127.0.0.1:2241 # chosen command history will be sent to 2241 port
```
