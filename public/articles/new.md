---
title: 在你的智能手机上编程
abstract: 了解如何使用termux安装基本的编译/调试工具
--- 
# 在你的智能手机上编程 / Code on your smart phone
## 中文版本 / Chinese Version
## 准备
你需要：
- 安卓手机
- [Termux](https://termux.dev/)
- [AnLinux](https://github.com/EXALAB/AnLinux-App)

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

## 英语版本 English Version
## Prepare
You need：
- Android Phone
- [Termux](https://termux.dev/)
- [AnLinux](https://github.com/EXALAB/AnLinux-App)

## First Step

Install termux and AnLinux, and open Termux later.

If you want termux has the permission to access local storage, please execute this in termux shell :``termux-setup-storage``

## Install Linux Distro

Open AnLinux, Select a Linux distro you want to install at ``Dashboard``.

Copy it's prompt and paste it in Termux shell.

When install complete, You will find a file called ``./start-(your system name).sh`` in your root dic.

Execute ``sh ./start-(your system name).sh`` in Termux shell to start linux.

I will use ubuntu for sample

## Setting up your Vim

Vim is a light-weight text editior which made for Linux.

Commonly, Vim is installed in Ubuntu,So you can simply start vim by: ``vim``

It will create a new file, press``i``to edit mode，when you compelte, press``esc`` and type``:wq`` to save and exit,use``:q``to just exit.

If you are not strict at Styles, well done! Enjoy your Vim!

If you want to customize your vim, please continue.

Vim's Global Config commonly at ``/etc/vim/vimrc`` or ``/etc/vimrc``, able to every user. User's customize configration is at``~/.vimrc``。

Just use ``vim /your/path/to/file ``to open a file.

I suggest you to exit these configs：

``syntax on`` Enable syntax High-Light

``set showmode`` show current mode at the bottom

``set showcmd`` Show what command you just type at the bottom

``set encoding=utf-8`` Translate encode to UTF-8

``set t_Co=256`` use 256 color

``filetype indent on`` Enable file type check,and load the custom indentation for each language.

Vim is a editior with high-expandation,so you can change its Styles, make it more easy to use etc. You can found more information of this on Google.

## Setting up your VScode.

If you don't want to Configure a Coplex Vim,,VScode will be a better choise!

But please notice,VScode is a app with GUI,You must install ``VNC server`` and use external keyboard and mouse,  or it will be very hard to use!

[VScode](https://code.visualstudio.com/#alt-downloads) Please download Arm32 or Arm64 Version! for Ubuntu or Debian，please download file name end with .deb, execute ``dpkg -i /your/path/to/package/package.deb`` in ubuntu terminal to install VScode client.

## Install complie environment

You should install JDK or GCC to complie Java and C.

### Java
execute ``apt install openjdk-17-jdk``to install jdk17, execute``apt install openjdk-8-jdk``to install JDK8

If you need another JDK,please find another JDK's installer.

### C
execute ``apt install gcc gcc-c++`` to install gcc and g++

### Nodejs
execute ``apt install nodejs npm`` to install Nodejs and npm

### Git
execute ``apt install git`` to install git tools.

## Complete and start code！

When you completed all of these,you now have a android phone which can code! I don't suggest you to install desktop environment because linux desktop environment are also poor without a keyboard and mouse.If you still want to install, install it in AnLinux with ``Windows Manager``!