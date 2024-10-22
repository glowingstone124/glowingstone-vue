---
title: 在你的智能手机上编程

abstract: 了解如何使用termux安装基本的编译/调试工具

picture: none

category: 0
ai_generated: false
---
## 准备
你需要：

 1.安卓手机

 2.[Termux](https://termux.dev/)

 3.[AnLinux](https://github.com/EXALAB/AnLinux-App)


## 第一步

安装Termux和AnLinux,并且在安装完成之后打开Termux.

如果你需要访问内部存储，请在你的termux终端中执行 ``termux-setup-storage`` 来让Termux访问你的存储空间。

## 安装Linux发行版

打开AnLinux, 在``仪表板`` 或 ``Dashboard``页面选择一个你要安装的Linux发行版。

复制它的指令，并且在Termux中粘贴

在安装完成之后，你会在根目录发现``./start-(your system name).sh``

在Termux中执行 ``sh ./start-(your system name).sh`` 来启动Linux。

这里我将会用ubuntu作示范

## 设置你的Vim

Vim 是一款轻量级的文字编辑器，它为Linux而打造。

通常ubuntu会自带Vim,所以你可以直接启动vim: ``vim``

这会打开一个新文件，按下``i``进入编辑模式，编辑完成后按下``esc``,输入``:wq`` 来退出并保存，使用``:q``来直接退出。

如果你对于样式没有要求，在这一步你就完成了配置，可以开始使用Vim了！

如果你想要优化你的Vim,请继续往下看。

Vim 的全局配置一般在``/etc/vim/vimrc``或者``/etc/vimrc``，对所有用户生效。用户个人的配置在``~/.vimrc``。

直接使用``vim /your/path/to/file ``打开文件即可

推荐更改以下设置：

``syntax on`` 打开语法高亮

``set showmode`` 在底部显示，当前处于命令模式还是插入模式

``set showcmd`` 在底部显示当前键入的指令

``set encoding=utf-8`` 将编码转换为UTF-8

``set t_Co=256`` 使用256色

``filetype indent on`` 开启文件类型检查，并且载入与该类型对应的缩进规则

Vim 是一款可拓展的编辑器，所以你可以使用插件更改它的样式等，本文在此不做展开。

## 设置你的VScode

如果你不想配置繁杂的Vim, VScode会是一个更好的选择！

但是请注意，VScode是图形化的，你需要额外安装``VNC server``并且使用外置键盘鼠标获取桌面端体验，不然它会非常难用！

[VScode下载](https://code.visualstudio.com/#alt-downloads) 请注意一定要下载Arm32或者Arm64版本的VScode安装包！ 对于Ubuntu 或 Debian，请下载.deb后缀的安装包，在ubuntu终端中输入``dpkg -i /your/path/to/package/package.deb`` 来安装VScode客户端。一般情况下会成功安装

## 安装编译环境

在安装完编辑器之后，你需要安装Jdk或者GCC来编译Java和C的源文件。本节将介绍如何安装它们

### Java
直接在命令行中输入 ``apt install openjdk-17-jdk``来安装jdk17,输入``apt install openjdk-8-jdk``来安装JDK8

如果你需要其它版本的JDK，请寻找对应JDK的Arm64位deb包。

### C
在命令行中输入``apt install gcc gcc-c++`` 来安装gcc和g++

### Nodejs
在命令行中输入``apt install nodejs npm`` 来安装Nodejs和npm环境

### Git
在命令行中输入``apt install git`` 来安装git工具

## 完成设置，开始编程！

在按部就班做完这一切之后，你拥有了一部可以写程序和编译的手机！我不推荐你安装桌面环境，因为桌面环境通常在手机上表现不好，如果你执意安装，可以在AnLinux中安装窗口管理器。
