# RustDecompileApk


- **The RustDecompileApk is a CLI tool using for decompiler Android apk, which uses Rust intergrating**

- When you need to reverse some android apks ,maybe you have manually used some third-party tools. 
- ***Now we speed up the process by using Rust CLI***


## How to use it


-  it works in Mac and Linux: **./apkdecompiler -f  xxxx.apk**
 ![](https://raw.githubusercontent.com/zhulg/allpic/master/decompilerapk.gif)



- run done will open outpath:

```
✅ create ouput:you decompiler outpath
✅ dex2jar...done
✅ decompile class...done
✅ decompile Resource...done
```

## source build

- git clone 

```
git clone https://github.com/zhulg/RustDecompileApk.git

```

- vscode open

```
cd RustDecompileApk
code .
```

- cargo build

```
debug
├── apkdecompiler
├── apkdecompiler.d
├── build
├── deps
├── examples
├── incremental
├── lib
└── output
```

- ./apkdecompiler --help 

```
ApkDecompiler for Android, create by Spark Coding BU

Usage: apkdecompiler [OPTIONS]

Options:
  -f, --file <file>  The path to your apk. [default: -]
  -h, --help         Print help information
  -V, --version      Print version information

Longer explanation to appear after the options when displaying the help information from --help or -h
```

## License
[Apache 2.0](http://www.apache.org/licenses/LICENSE-2.0.html)

