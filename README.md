# History Search

Context-aware Bash/Zsh eternal history manager

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
