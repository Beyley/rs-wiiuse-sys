# rs-wiiuse-sys
Raw wiiuse ffi bindings for rs-wiiuse
# usage
```toml
[dependencies]
rs-wiiuse-sys = { git = "https://github.com/Tazdevil971/rs-wiiuse-sys.git" }
```
If the environment variable ```WIIUSE_PATH``` is not defined the script will clone the repository automatically using ```WIIUSE_REPO``` or the default value ```https://github.com/wiiuse/wiiuse.git```