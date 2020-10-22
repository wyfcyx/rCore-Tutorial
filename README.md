# rCore-Tutorial V3（开发中）

[![Actions Status](https://github.com/rcore-os/rCore-Tutorial/workflows/CI/badge.svg?branch=master)](https://github.com/rcore-os/rCore-Tutorial/actions)

## 快速上手

### 项目背景

[这里](https://rcore-os.github.io/rCore_tutorial_doc/)是教程第二版，于 2020 年春季学期作为清华大学操作系统课程的新增可选实验，只支持 `qemu-system-riscv64` 的单核模式，且 syscall 支持较少。目前已经不再维护了。[项目主页](http://os.cs.tsinghua.edu.cn/oscourse/OsTrain2019/g7)

[这里](https://github.com/rcore-os/rCore-Tutorial/)是教程第三版，在第二版基础上重构，在一定程度上进行了简化，让整个框架更加软工友好，特别是完善了内存的回收机制。有着比较详细的代码文档，但是也只支持 `qemu-system-riscv64` 的单核模式，syscall 支持较少。[项目主页](http://os.cs.tsinghua.edu.cn/oscourse/OS2020spring/projects/g06)

[这里](https://github.com/wyfcyx/rCore-Tutorial/tree/multicore)是正在开发中的教程第三版完善分支，同时支持 `k210, qemu-system-riscv64` 两个平台，底层的 SBI 实现基于[RustSBI](https://github.com/luojia65/rustsbi) 最大程度上屏蔽了两个平台的不同。支持多核，增加了更多 syscall，且语义更加接近于传统 Unix，目前已经兼容 ucore 的大部分应用并尽可能兼容 linux-riscv64。目前的实现还有一些问题，在 `k210` 平台上跑的时候可能出现死循环的情况。[项目主页](http://os.cs.tsinghua.edu.cn/oscourse/OsTrain2020/g1)

### 环境配置

首先[这里](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/pre-lab/env.html)给出了包括 Rust/Qemu/虚拟机环境配置的相关说明，在配置完成之后，应该能够基于 Qemu 跑教程第三版的 master。

为了跑完善之后的 multicore 分支，则需要一些额外的工具链：

* 当提示缺少 `riscv64-unknown-elf-*` 的时候，下载[macOS 平台的预编译工具链](https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.3.0-2020.04.0-x86_64-apple-darwin.tar.gz?_ga=2.230260892.1021855761.1603335606-1708912445.1603335606)或[Ubuntu 平台的](https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.3.0-2020.04.0-x86_64-linux-ubuntu14.tar.gz?_ga=2.230260892.1021855761.1603335606-1708912445.1603335606)，安装之后添加到环境变量中。
* 当提示缺少 `riscv64-linux-musl-*`，下载[musl-libc 工具链](https://musl.cc/riscv64-linux-musl-cross.tgz)，安装之后添加到环境变量中。

PC 需要通过串口与 K210 开发版通信，完成内核镜像烧写和运行时的交互。因此请确保当前 Python 环境安装了 pyserial 包且 miniterm 串口终端应当可用。

### 在 Qemu 平台上运行 multicore 分支

```sh
git clone https://github.com/wyfcyx/rCore-Tutorial.git
cd rCore-Tutorial
make run LOG=error
```

注意 `LOG=<LOG_LEVEL>` 表示 log 的控制级别。log 的重要程度从高到低依次为 error, warn, info, debug, trace，当设置为某个控制级别后只会打印重要程度不低于设置的控制级别的 log。正常运行的时候根据情况不同设置为 error, warn, info 之一即可，而不应设置为更低等级，否则会影响到 kernel 的行为。

内核启动起来之后需要稍等一会，等内核完成初始化之后，屏幕最下方会出现一行 

```
>> 
```

此时我们可以直接通过键盘输入程序的名字来运行程序。

在初始化过程中已经列出了所有的程序，以 `r_` 开头的是用 Rust 编写的用户程序（位于 [user/rust 目录](https://github.com/wyfcyx/rCore-Tutorial/tree/multicore/user/rust/src/bin)下），其他的则来自于 ucore（位于 [user/ucore 目录](https://github.com/wyfcyx/rCore-Tutorial/tree/multicore/user/ucore/src)下）。可以直接运行 `r_usertests` 来跑[所有目前可用的测试](https://github.com/wyfcyx/rCore-Tutorial/blob/multicore/user/rust/src/bin/r_usertests.rs#L7)。

我们可以先按下 Ctrl+A，再按下 X 来退出 Qemu 模拟器。

### 在 K210 平台上运行 multicore 分支

首先我们需要在 sdcard 中写入文件系统。将 microsd 卡放入读卡器中插入 PC，随后：

```sh
cd user
make sdcard SDCARD=/dev/sdb
```

其中 `SDCARD` 设置 microsd 块设备在当前 OS 中的位置，通常来说是 /dev/sdb。**注意不设置 SDCARD 的话，默认为 /dev/sdb，在写入前务必小心损坏其他磁盘！**目前这个操作在 wsl/wsl2 上似乎无法完成。

然后，在开发板断电的情况下将 microsd 插入进去。

接着我们用数据线连接 PC 和开发板并返回主目录：

```sh
make run BOARD=k210 LOG=error
```

运行起来之后和 Qemu 平台是一样的。我们可以通过 Ctrl+] 退出 Miniterm 串口终端。

### [ucore 应用移植的相关记录](https://github.com/wyfcyx/osnotes/blob/master/book/%E4%BB%8E%E9%9B%B6%E5%BC%80%E5%A7%8B%E7%9A%84%E7%BC%9D%E5%90%88%E4%B9%8B%E6%97%85.md#ucore-%E5%BA%94%E7%94%A8%E7%A7%BB%E6%A4%8D)

## Quick Run
### For K210 platform
```sh
# Prepare sdcard if you want to run on k210 platform
# Use SDCARD to configure sdcard location, it is /dev/sdb by default
cd user && make sdcard
# plug the sdcard in the k210 board and then run tutorial on k210 platform
make run BOARD=k210 LOG=error
```
### For Qemu platform
```sh
# Run tutorial on qemu platform
make run
```
### User programs
There are three simple user programs now, they are:
* r_hello_world;
* r_fantastic_text;
* r_notebook[user Ctrl + C to exit].

To execute a program, type its name in the shell and press enter.
After it finishes, some statistics will be showed on the screen as well, which indicates that the program uses multicores.

[本教学仓库](https://github.com/rcore-os/rCore-Tutorial)是继 [rCore_tutorial V2](https://rcore-os.github.io/rCore_tutorial_doc/) 后重构的 V3 版本。

本文档的目标主要针对「做实验的同学」，我们会对每章结束后提供完成的代码，你的练习题只需要基于我们给出的版本上增量实现即可，不需要重新按照教程写一遍。

而对想完整实现一个 rCore 的同学来说，我们的文档可能不太友好。因为在编写教程过程中，我们需要对清晰和全面做很多的权衡和考虑、需要省略掉大量 Rust 语法层面和 OS 无关的代码以带来更好的可读性和精简性，所以想参考本文档并完整实现的同学可能不会有从头复制到尾的流畅（这样的做法也不是学习的初衷），可能需要自己有一些完整的认识和思考。

另外，如果你觉得字体大小和样式不舒服，可以通过 GitBook 上方的按钮调节。

## 仓库目录

- `docs/`：教学实验指导分实验内容和开发规范
- `notes/`：开题报告和若干讨论
- `os/`：操作系统代码
- `user/`：用户态代码
- `SUMMARY.md`：GitBook 目录页
- `book.json`：GitBook 配置文件
- `rust-toolchain`：限定 Rust 工具链版本
- `deploy.sh`：自动部署脚本
<!-- Rust 工具链版本需要根据时间更新 -->

## 实验指导

基于 GitBook，目前已经部署到了 [GitHub Pages](https://rcore-os.github.io/rCore-Tutorial-deploy/) 上面。

### 文档本地使用方法

<!-- 下面的代码不再使用标签，因为也同时会渲染到 GitHub 的项目首页 -->
```bash
npm install -g gitbook-cli
gitbook install
gitbook serve
```

## 代码

### 操作系统代码
本项目基于 cargo 和 make 等工具，在根目录通过 `make run` 命令即可运行代码，更具体的细节请参见 `Makefile`、`os/Makefile` 以及 `user/Makefile`。

### 参考和感谢

本文档和代码部分参考了：
- [rCore](https://github.com/rcore-os/rCore)
- [zCore](https://github.com/rcore-os/zCore)
- [rCore_tutorial V2](https://rcore-os.github.io/rCore_tutorial_doc/)
- [使用Rust编写操作系统](https://github.com/rustcc/writing-an-os-in-rust)

在此对仓库的开发和维护者表示感谢，同时也感谢很多在本项目开发中一起讨论和勘误的老师和同学们。

<!-- TODO LICENSE -->
