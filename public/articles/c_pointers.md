---
title: C系列指针剖析

abstract: 所谓的指针究竟是什么？计算机是怎么处理你的每一个魔法操作的？

picture: none

category: 0

ai_generated: false
---
## 写在最前

说到C和指针，相信大部分人都不陌生，相信如果你有一定的C/C++实践的话，也可以轻松地使用指针完成许多操作。不过你有没有想过，指针是怎么工作的？为什么我们需要指针？这篇文章也许可以给你一些启发。

在本文中，我们将主要关注C语言/x86 ASM。使用到的代码会在本文中随附。

对于架构不敏感的部分，本文的代码运行在MacOS arm64，对于必须使用x86的部分，本文使用Orbstack虚拟的Rocky Linux 9 x86。

## 内存的表示方式

内存使用二进制来存储数据，也就是0和1，一个0或者1所代表的也就是一个`bit`，又叫做比特，而8个`bit`的组合我们叫做`byte`(字节)，它和我们接下来讲到的”寻址“息息相关。

计算机本身并不聪明，它并不知道你的代码、数据、甚至操作系统应该被放在内存的哪个位置。为了解决这个问题，计算机架构中引入了一个“内存映射”机制，最常见的就是分页（paging）系统。它就像一本巨大的通讯录，记录了哪些“虚拟地址”实际指向了“物理地址”的哪一块区域。有了这套机制，程序才能使用看似统一连续的地址空间去访问实际的物理内存。

在现代的 `64-bit x86` 架构中，每个地址代表一个字节，也就是说，一个地址`0x1000`指的就是内存中第`4096`个`byte`的位置（从0开始算起），如果你不确定`0x1000`代表着什么，请首先了解二进制，十进制和十六进制互相转换的关系。

虽然我们通常都说一个CPU的架构是"64位"，但是CPU并不会使用64位去作为地址，不然你获得的就是一个2^64 = 16EB的巨大寻址空间，这很显然超过了目前计算机的需求，所以目前的两家主流x86处理器厂商，也就是Intel和AMD,使用低48位来进行寻址。

那么，剩下的高位部分应该填写什么？为此，x86-64引入了`Canonical Address`（规范地址）的概念。

> 当然在后来他们又发明了各种地址拓展机制，不过我们不在这里讨论了，毕竟这不是本文的重点--而且大部分计算机教材还在讨论8086的老旧段地址寻址模式

如果一个地址是Canonical的，那么它的高位(48-63)应当和47位一致，让我们看两个例子：

```
(1)0x00007FFFFFFFFFFF

(2)0x0000F00000000000
```

很显然，(1)是合法地址，而(2)是不合法的。如果你尝试访问/指向一个非canonical地址，会引发`#GP General Protection Fault`异常。

所以你大概也发现了，实际上内存的每个地址都是一个64位的`unsigned long long`，或者说人话-`64位的无符号整数`。

让我们来编写一个C程序:

```C
#include <stdio.h>

int main() {
	int *ptr;
	unsigned int number;
	unsigned long long longnumber;
	printf("sizeof(ptr)= %zu\n", sizeof(ptr));
	printf("size of a unsigned int: %zu\n", sizeof(number));
	printf("size of a unsigned long long: %zu\n", sizeof(longnumber));
	return 0;
}
```

它的输出(在我的arm64 mac上)

```
sizeof(ptr)= 8
size of a unsigned int: 4
size of a unsigned long long: 8
```

你注意到什么没？没错，指针也是用同样的64位long long来标定内存地址的。当然，32bit的系统上，sizeof(ptr)的输出就是4，相信你马上就可以知道为什么。

了解了这些最基本的（吹牛用）前置知识，接下来我们看看关于指针的部分。

## 指针初探：怎么这么麻烦？

在学习C语言时，特别是对于有 Python 或 Java 背景的开发者，指针常常是一个让人困惑的概念。你可能会想，“为什么 C 语言要引入这么麻烦的指针机制？”这是因为，像 Python 或 Java 这样的高级语言都将内存管理和指针操作做了抽象，交给了运行时环境（JVM 或 Python 解释器）。然而，C 作为一门底层语言，它没有这样的运行时支持，而是让程序员自己负责内存的管理和指针操作。
让我们回顾一下C指针的一些基本知识：

```C
int num = 9;
int *ptr;
ptr = &num;
```
这段代码做了什么呢？

1.它创建了一个名为 num 的`int`变量，并赋值为 9。

2.然后它声明了一个指向`int`类型的指针 ptr。

3.最后，ptr 被赋值为 num 的内存地址，即 ptr 指向了 num。

如果你已经有了一些 C 语言的基础，这段代码应该不难理解。`&num` 获取了 num 的内存地址，然后 ptr 作为一个指针变量，存储了这个地址。通过 ptr，你可以间接地访问 num 的值。

> 有些开发者可能会想：指针类型是不是可以写成 void * 呢？ 其实是可以的，但这么做存在一定的风险，因此不推荐。最好还是根据需要指定指针的类型。

**为什么 `void *` 是可行的？**

实际上，所有的变量都是一串二进制数据，计算机和编译器通过特定的方式对这些二进制数据进行分割和解释，以形成不同的数据类型。`void *` 是一种通用指针类型，可以指向任何类型的数据，但由于它没有类型信息，所以在使用时需要将其转换为实际的数据类型，才能进行访问。

举个例子，你可以把指针 ptr 声明为 `void *`，然后通过类型转换来访问指向的数据：

```C
int num = 9;
void *ptr = &num;
printf("num = %d\n", *(int *)ptr); //输出9
```
同样的，你还可以通过指针来修改源变量的值：

```C
int num = 9;
int *ptr = &num;
*ptr = 5;
```

这样操作之后，num就会变成5，现在让我们更进一步，看看在汇编上的实现：

> 如果你不知道的话，通过gcc -S <你的c代码>可以生成它的asm表示。

```
.file   "main.c"
        .text
        .globl  main
        .type   main, @function
main:
.LFB0:
        .cfi_startproc
        pushq   %rbp
        .cfi_def_cfa_offset 16
        .cfi_offset 6, -16
        movq    %rsp, %rbp
        .cfi_def_cfa_register 6
        movl    $9, -12(%rbp) 
        leaq    -12(%rbp), %rax
        movq    %rax, -8(%rbp)
        movq    -8(%rbp), %rax
        movl    $5, (%rax)
        movl    $0, %eax
        popq    %rbp
        .cfi_def_cfa 7, 8
        ret
        .cfi_endproc
.LFE0:
        .size   main, .-main
        .ident  "GCC: (GNU) 11.5.0 20240719 (Red Hat 11.5.0-5)"
        .section        .note.GNU-stack,"",@progbits
```


开头的`.file "main.c"`表示了它的来源文件，`.text`代表后面的代码可以执行。现在让我们关注`main:`段，这是我们程序的主入口。

我们的第一行代码，也就是`int num = 9`对应了`movl    $9, -12(%rbp)`，将常量9存储到栈帧中的`-12(%rbp)`位置。

```
leaq    -12(%rbp), %rax
movq    %rax, -8(%rbp)
```

这两行代码首先使用`leaq`指令计算了num的地址`-12(%rbp)`，并且将它加载到了寄存器rax中，随后`movq`将rax的地址存储到`-8(%rbp)`,也就是将ptr存储在栈上`-8(%rbp)`。

在修改值的时候，

```
movq    -8(%rbp), %rax
movl    $5, (%rax)
```
首先加载了ptr的值，也就是num的地址到`rax`寄存器，然后将5写入`rax`的内存地址，也就是将num改为5.
之后往`eax`写入了0，完成了整个操作。

这里你会发现，一个int正好在栈上使用了4位，也就是前面提到的`-12(%rbp)`到`-8(%rbp)`，而ptr作为一个`unsigned long long`，占领了`-8(%rbp)`到基址`%rbp`的8个字节。

指针的操作本质上是对内存地址进行访问和修改。而在栈上，局部变量 num 和 ptr 被分配了特定的位置，通过偏移量可以访问它们。

但是，指针的乐趣不止于此。刚刚我们只是对它指向的基本数据进行了更改，我们不妨来更改一下变量本身。

还是刚刚的那个代码，我这次做了一些小修改，

```C
struct Numbers {
    int num;
    int num1;
};

struct Numbers nums = {9, 10};
int *ptr = &nums.num;
ptr += 1;
printf("%d\n", *ptr);  // 输出 10
```
这次它输出的值并不是num,而是num1。你可能会觉得很奇怪，为什么这里没有任何地方提到num1，但是我们依然获取到了num1的值？实际上，这和`struct`的`内存连续性`有很大关系。

对于一个`struct`，它的成员变量通常是连续的，并且遵从声明顺序。当然有时候编译器会进行内存对齐，在变量中插入无效字节来提高CPU的存取效率，但是在目前这个struct中不需要对齐。当前的struct内部内存大概长这样：
```
|----|----|
| 9  | 10 |
|----|----|
   ^
```
所以我们可以通过ptr += 1让ptr往它的后方偏移一个`sizeof(int)`，也就是4字节，让它指向num1。
```
|----|----|
| 9  | 10 |
|----|----|
       ^
```
> 对于一个指针直接+n的时候，指针实际上对于地址的偏移量是 n * sizeof(数据类型),而不是n。

了解了这些比较基础的指针玩法，相信各位已经大致了解了C相对其他语言的不同之处，接下来，我们继续聊指针。

## 进一步：堆上内存

刚刚我们聊了一些最基本的栈上数据，这些数据还比较易于处理，但是大部分程序都不可能只使用这些简单的数据，更多的日常场景发生在堆上。

首先我们来回忆下什么是堆内存。在我们上一个部分的实践中，栈帧会被自动推入并且随着程序结束被弹出，我们没有显式的管理内存。但是堆可不一样，它必须手动分配和释放，不然你的代码就会内存泄露。`JVM`采用GC来让程序员不需要操心这个，但是很明显我们现在没有GC。

好消息是，C和C++都提供了一些库函数来帮助我们管理内存，这里我们暂时先只讨论C的实现。

```C
int *ptr = (int*) malloc(sizeof(int));
*ptr = 9;
free(ptr);
```

在这个代码中，我首先通过`malloc`函数在堆上开辟了一块`sizeof(int)`大小的内存，然后将这个地址赋值给了一个指针。

之后我首先将这个内存区域的表示内容设置为9，也就是正好占满了一个int的大小，最后`delete`释放了这个资源。

让我们来看看它的汇编代码。

```
	.file	"main.c"
	.text
	.globl	main
	.type	main, @function
main:
.LFB6:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movl	$4, %edi
	call	malloc
	movq	%rax, -8(%rbp)
	movq	-8(%rbp), %rax
	movl	$9, (%rax)
	movq	-8(%rbp), %rax
	movq	%rax, %rdi
	call	free
	movl	$0, %eax
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE6:
	.size	main, .-main
	.ident	"GCC: (GNU) 11.5.0 20240719 (Red Hat 11.5.0-5)"
	.section	.note.GNU-stack,"",@progbits
```

前面的部分还是一样，让我们直接跳转到`movl $4, %edi`部分。
```
movl	$4, %edi
call	malloc
```
它调用了`malloc`函数，并且通过`edi`寄存器传递了参数4，来请求分配4字节内存，它的结果会在`%rax`中。

之后，我们使用`movq -8%(rbp) ,%rax`将指针保存到了栈上的变量。

```
movl $9, (%rax)
```
将9这个常量写入到了这个地址指向的内存空间。
```
movq -8(%rbp), %rax
movq %rax, %rdi
call free
```
这段代码将之前的指针传递给了`free`函数，释放了分配的内存，最后结束整个程序。

所以我们发现，对于堆内存的分配实际上依靠`malloc`和`free`函数，其他部分实际上和栈内存是一样的，都拿到了一个指针然后对它进行修改。这主要是因为栈内存和堆内存都在同一个物理内存区域上。

但是还记得我们刚刚提到了什么吗？堆内存需要手动分配和释放，所以它会带来很多问题。

### 内存泄露

首当其冲的是内存泄露。这个问题非常常见，但是很危险。让我们来看看这个代码：

```C
int *ptr = (int*) malloc(sizeof(int));
*ptr = 9;
```
这段代码分配了一个4字节的内存，但是在赋值完成而且没有任何程序使用的情况下，它没有被回收，这会导致它一直驻留在内存区域中。

通常它不会干扰程序的运行，但是如果大量这样的旧数据保存在内存里，带来最直接的后果就是内存占用会变大很多，直到无法再alloc新的内存，带来OOM。

### 悬挂指针

悬挂指针是另一个可能遇到的问题，

```C
int *ptr = (int*) malloc(sizeof(int));
*ptr = 9;
free(ptr);
*ptr = 10;
```
这里我们首先分配了内存空间，然后赋值，清除，但是我们在最后又调用了一次赋值。

在这整个过程中，ptr的指向没有任何变化，但是它指向的那块内存空间被free了。这个操作危险的地方在于，你没有办法确定那块内存有没有被别的程序申请malloc。如果你继续往里面写入，同时另一个程序在这里存放了数据，你就会破坏这里本应存在的数据，引发严重的后果。

### 双重释放

```C
int *ptr = (int*) malloc(sizeof(int));
*ptr = 9;
free(ptr);
free(ptr);
```

在这里我们`free`了两次内存空间，在运行时引发了错误:
```
free(): double free detected in tcache 2
Aborted
```
这个错误来自`glibc`，提醒你这里产生了double free。

再看一个用法，很多人都会犯这个错误：

```C
void clear(char *str)
{
	free(str);
	str = NULL;
}
 
int main()
{
	char *str;
	str = (char *)malloc(100);
	clear(str);
	return 0;
}
```

在这里，`str=NULL`操作并不会影响到main函数的str指针。可以思考一下原因。

答案是，clear函数的传入值是str，这是一个值传递。也就是说，你传入clear函数的指针实际上是一个副本，无论如何都无法影响到你main函数的str值。

最简单的修改方式是把`clear(str)`变成`clear(&str)`,然后将clear的参数从`char *str`变更为`char **str`，这样你传入的就是指针的指针，指向main函数中的指针，从而可以修改那个值。

double free会带来很多问题，还可能被攻击者所利用。主要原理如下：

在glibc中，内存被分成很多malloc chunk。
```C
struct malloc_chunk {
  INTERNAL_SIZE_T      prev_size;  /* 前一个chunk的大小 (如果前一个已经被free)*/
  INTERNAL_SIZE_T      size;       /* 字节表示的chunk大小，包括chunk头       */
  struct malloc_chunk* fd;         /* 双向链表 -- 只在被free后才存在       */
  struct malloc_chunk* bk;
};
```
这是glibc的实现，在内存块被free之前，fd和bk这两个指针不存在，在size之后的内存空间就是用户可以使用的内存了，也就是之前我们提到的malloc返回地址。在内存被free之后，这两个指针就会构成我们非常熟悉的数据结构：**链表**。

他们分别指向之前一个和之后一个被释放的内存，当程序申请内存时，系统会遍历这个表，如果有合适的内存就将它分配给程序。

要利用这个漏洞，就需要欺骗系统来unlink一个内存块，来篡改指针。具体方法较长，在这里写有点舍本逐末，就暂时略过了。攻击者可以通过这个漏洞向内存写入任意值，甚至可以从缓冲区载入shell代码。


