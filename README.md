# Rust use C++ Dynamic Library Demo

本项目给出了在 Windows 环境下 Rust 调用 C++ 库（Open3D）的示例。

目前尚未实现脚本化打包。 需要借助 Clion IDE 来搞。

This Demo show How Rust use C++ library（CMake） in Windows OS.


## 打包
1. 下载 Open3D Binary 包
2. Clion Settings->CMake->CMake Options 中添加 -DOpen3D_ROOT=path/of/open3d，Build Type 选择 Release
3. Clion Settings -> ToolChains, Architecture 选择amd64（Open3d 只支持 x64）
4. Clion 菜单 build -> install, 编译软件，并安装 lib、dll 到当前相应目录中
5. `cargo run` 

## Package
1. Download Open3D binary package
2. Clion Settings->CMake->CMake Options add `-DOpen3D_ROOT=path/of/open3d`，`Build Type` choose `Release`
3. Clion Settings -> ToolChains, Architecture choose `amd64`
4. Clion menu build -> install
5. `cargo run`

# 问题
1. Debug 模式 和 Relase 模式的 DLL 是不同的，需要各自编译并复制到 Rust 项目中的 target/debug|release 目录下。
2. 当遇到 C++ 和 Rust 共享数据结构（C++ 与 Rust 相互调用）的时候， 这个 Demo 解决不了(如何在 CMake 中引入 Cargo 生成 header 的同时，在 Cargo 中引入 C++ 的 header)。


# 其他方案设想
1. 退回到 Cargo 编译所有，并使用 cxx 来生成 Header，可解决上述问题，但尚不知道如何解决 C++ 里面复杂的 Header 依赖问题。


# 最终上生产方案
1. CMake 封装 C++ 成 C HEADER。
2. Rust use bindgen to use C++。

Bazel 项目封装 可参考：https://github.com/timzaak/bazel_wrapper
