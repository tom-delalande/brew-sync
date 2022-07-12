# Brew Sync

This is a small app that I am mostly making just to learn rust.
It uses a TOML file to declarativly store brew dependencies.

Here is an example file
```TOML
[formulae]
neovim = {}
postgres = {}
wget = {}

[casks]
reaper = {}
firefox = {}
```

Use the app like this

```bash
brew-sync path-to-file-name.toml
```
it will default to `brew.toml` if no file is provided.

The app will scan the diff between your currently installed apps and the ones supplied in the file. 
Installing missing apps and uninstalling old ones.

It currently does not support flags.

