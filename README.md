# Rust use C++ Dynamic Library Demo

本项目给出了在 Windows 环境下 Rust 调用 C++ 库（Open3D）的示例。

目前尚未实现脚本化打包。 需要借助 Clion IDE 来搞。

This Demo show How Rust use C++ library in Windows OS.


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