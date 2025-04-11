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

### 关于NULL宏

实际上，C在[C23](https://en.cppreference.com/w/c/language/nullptr)之前没有C++中的`nullptr`，所以NULL是一个规范化的宏，用于定义空指针。

GCC的[stddef.h](https://github.com/gcc-mirror/gcc/blob/master/gcc/ginclude/stddef.h)定义了NULL：

```C
#if defined (_STDDEF_H) || defined (__need_NULL)
#undef NULL		/* in case <stdio.h> has defined it. */
#ifdef __GNUG__
#define NULL __null
#else   /* G++ */
#ifndef __cplusplus
#define NULL ((void *)0)
#else   /* C++ */
#define NULL 0
#endif  /* C++ */
#endif  /* G++ */
#endif	/* NULL not defined and <stddef.h> or need NULL.  */
#undef	__need_NULL
```

我们会发现这里的`NULL`实际上是一个指向0的`void`指针。或者是0。因为0地址实际上是非法的，所以将它作为`NULL`很合适。

但是由于`NULL`作为一个宏在各个实现之间并不一致，有时为0有时为`void* 0`，所以非常不建议使用`NULL`,使用类型为`nullptr_t`的`nullptr`才是安全并且正确的做法。
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

## 不定长struct：还是来看堆

刚刚我们了解了栈内存和堆内存的不同，也简单熟悉了`malloc()`函数和`free()`函数的基本操作，接下来我们看这么一个场景。

```C
typedef struct {
    int count;
    char data[];
} Packet;
```

这是一个不定长的struct，原因在于里面的`char data[]`是一个不定长的`char`数组，它不会占据`sizeof(data)`的字节数量。

现在让我们分配10字节数据：

```C
Packet* p = malloc(sizeof(packet) + 10);
```

这个时候，p是一个在栈上的指针，因为它的大小固定，而结构体本身不定长，所以它指向堆上的结构体。

让我们来看看它的asm表示：
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
	movl	$14, %edi
	call	malloc
	movq	%rax, -8(%rbp)
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

通过我们前面的讲解，你很快就可以发现，这里我们直接malloc了14个字节的单元将返回值保存到`%rax`，然后将`%rax`保存到栈上的`-8(%rbp)`也就是我们的指针p，印证了我们刚刚的结论。

接下来，让我们来试试别的:

```C
p->data[10] = 'g';
```

在这里，我们向data的第11项写入了一个`char`值，在汇编中体现为：
```
movq    -8(%rbp), %rax  
movb    $103, 14(%rax)
```

这里的问题在于，我们在前面的`malloc`过程中只分配了10个字节的长度给data数组，而第11位我们并没有定义，这就会引发我们说的数组越界。它会操作这部分不属于它的内存。

通过我们上一部分的例子，你应当已经知道随意操作不属于你的内存会带来什么后果：错误覆盖别的数据，产生`Segmentation Fault`，等等。。。

接下来我们来看看如何使用`realloc`函数来解决这个问题。

```
p = realloc(p, sizeof(Packet) + 11);
```
通过这个操作，我们将p扩容到了11长度，再往第11个索引写入char就是安全的做法了。

这里我们提到的完整代码：

```C
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
	int count;
	char data[];
} Packet;

int main() {
	const char *msg = "cirnobaka";
	size_t len = strlen(msg) + 1;

	Packet *p = malloc(sizeof(Packet) + len);
	strcpy(p->data, msg);

	p = realloc(p, sizeof(Packet) + len + 1);
	p->data[10] = 'g';
	p->data[11] = '\0';
	free(p);
	return 0;
}
```
> 在C中，字符串结尾需要用\0来表示，所以虽然我们的msg只有9个字节，它仍然占用了10个字节的内存空间。

我们来看看它的汇编表示，主要关注realloc部分。

```
        .file   "main.c"
        .text
        .section        .rodata
.LC0:
        .string "%s"
        .text
        .globl  main
        .type   main, @function
main:
.LFB6:
        .cfi_startproc
        pushq   %rbp
        .cfi_def_cfa_offset 16
        .cfi_offset 6, -16
        movq    %rsp, %rbp
        .cfi_def_cfa_register 6
        subq    $16, %rsp
        movl    $14, %edi
        call    malloc
        movq    %rax, -8(%rbp)
        movq    -8(%rbp), %rax
        addq    $4, %rax
        movabsq $7737573865533106531, %rdx
        movq    %rdx, (%rax)
        movw    $97, 8(%rax)
        movq    -8(%rbp), %rax
        movl    $15, %esi
        movq    %rax, %rdi
        call    realloc
        movq    %rax, -8(%rbp)
        movq    -8(%rbp), %rax
        movb    $103, 14(%rax)
        movq    -8(%rbp), %rax
        addq    $4, %rax
        movq    %rax, %rsi
        movl    $.LC0, %edi
        movl    $0, %eax
        call    printf
        movl    $0, %eax
        leave
        .cfi_def_cfa 7, 8
        ret
        .cfi_endproc
.LFE6:
        .size   main, .-main
        .ident  "GCC: (GNU) 11.5.0 20240719 (Red Hat 11.5.0-5)"
        .section        .note.GNU-stack,"",@progbits
```

在这里，我们首先初始化数据，在
``` 
movl    $15, %esi
movq    %rax, %rdi
call    realloc
```
这里，我们`realloc`了内存大小，然后在
```
movb    $103, 14(%rax)
```
将g(ASCII = 103)写入了第11个字节。

到这里我们了解了`malloc`和`realloc`在汇编层面是如何使用的，相信你很快就会发现，C代码其实基本上都可以和汇编一一对应。

### 手写string实现

在C甚至是C++中，手写`string`实现确实并不罕见，所以我们也来实现一下，这里主要是起总结作用，没什么新内容引入。

我们的string实现目标是管理一个`char`数组，并且支持基本的字符串操作，比如构造，析构，复制，链接，访问字符等等。

让我们先来定义结构体：

```C
typedef struct {
    char *data;
    size_t length;
    size_t capacity;
} String;
```

这里我们定义了一个char指针而不是char数组，因为我们希望String这个struct是定长的。

让我们首先实现init:

```C
void string_init(String* str, const char *initial_data) {
    str->length = strlen(initial_data);
    str->capacity = str->length + 1;
    str->data = (char*)malloc(str->capacity);
    if (str->data) {
        strcpy(str->data, initial_data);
    }
}
```

这个函数比较好理解，我们通过strlen获得了初始数据的长度并且把它赋值给length变量，然后添上一位给`\0`作为容量capacity，再通过`malloc` capacity并且转换为`char*`指针在堆上开辟内存空间，并将initial_data通过`strcpy`函数写入这个空间。

有了init，我们继续来摧毁它：

```C
void string_free(String str) {
    if (str->data) {
        free(str->data);
        str->data = NULL;
    }
    str->length = 0;
    str-> capacity = 0;
}
```

这个代码free了堆上的空间，然后将栈上的长度部分设置为0。

然后是两个工具函数，没啥特别的

```C
size_t string_length(const String *str) {
    return str->length;
}

const char* string_data(const String *str) {
    return str->data;
}
```
接下来是resize，这里用到`realloc`

```C
void string_resize(String *str, size_t new_capacity) {
    if (new_capacity > str->capacity) {
        str->data = (char *)realloc(str->data, new_capacity);
        str->capacity = new_capacity;
    }
}
```

前面也解释过这段代码的意义，它直接`realloc`了原有的内存大小到新的capacity，然后将指针移动了过去。

之后的`append`和`copy`函数也很简单：

```C
void string_append(String *str, const char *suffix) {
    size_t suffix_length = strlen(suffix);
    size_t new_length = str->length + suffix_length;

    string_resize(str, new_length + 1);
    strcpy(str->data + str->length, suffix);
    str->length = new_length;
}

void string_copy(String *str, const String *source) {
    string_resize(str, source->length + 1);
    strcpy(str->data, source->data);
    str->length = source->length;
}
```
这里主要也是运用了`realloc`来重新分配内存空间。

最后的`string_at`函数就是对index的操作，这里进行了一个简单的操作来防止访问错误的内存：

```C
char string_at(const String *str, size_t index) {
    if (index < str->length) {
        return str->data[index];
    }
    return '\0';
}
```

到这里，我们对于简单不定长struct的分析就基本结束了。

## 指针的复杂用法

本节中，我们主要关注一些比较高阶的指针用法，并且分析它们的作用。

### 指向函数的指针

在汇编中，一个函数其实就是一段代码段，而代码段也是内存，所以，一个指针也可以指向一个函数。当我们创建一个指向函数的指针时，这个指针实际上存储的是某个函数的地址，使用它可以间接地调用该函数。

让我们看一个例子：

```
int add(int a, int b) {
    return a + b;
}
```

这是一个普通的函数，我们创建一个指向该函数的指针：

```
int (*func_ptr)(int,int);
func_ptr = &add;

int result = func_ptr(2,3);
```

你会发现我们就像调用add一样调用了这个指针，并且获得了add的调用结果。

我们来看看汇编：

```
	.file	"main.c"
	.text
	.globl	add
	.type	add, @function
add:
.LFB6:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	movl	%edi, -4(%rbp)
	movl	%esi, -8(%rbp)
	movl	-4(%rbp), %edx
	movl	-8(%rbp), %eax
	addl	%edx, %eax
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE6:
	.size	add, .-add
	.globl	main
	.type	main, @function
main:
.LFB7:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movq	$add, -8(%rbp)
	movq	-8(%rbp), %rax
	movl	$3, %esi
	movl	$2, %edi
	call	*%rax
	movl	%eax, -12(%rbp)
	movl	$0, %eax
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE7:
	.size	main, .-main
	.ident	"GCC: (GNU) 11.5.0 20240719 (Red Hat 11.5.0-5)"
	.section	.note.GNU-stack,"",@progbits
```
`add`部分没啥特别的，就是我们刚实现的简单函数，我们主要关注`main`部分。

我们发现，首先汇编使用`movq $add, -8(%rbp)`将add函数的地址入栈，然后将3和2两个常量放入寄存器，最后`call *%rax`通过指针调用了add函数，和我们之前提到的调用方式一模一样。

所以，它在底层上和直接调用是等价的，但是在工程上有很大用处，因为我们可以将一个函数指针作为参数传入。

```
int add(int a, int b) {
    return a + b;
}
int multiply(int a, int b) {
    return a * b;
}
int process(int a, int b, int (*operation)(int,int)) {
    return operation(a,b);
}
```

这个简单的例子让我们可以通过传入不同的函数指针执行不同的操作，例如回调函数。由于实际上的实现方法都差不多，我们就不在这里讨论了。

### 多级指针

你也许见过`int *****p`这样的梗图，而它就是一个多级指针。让我们来举个例子：

```C
int a = 9;
int *p = &a;
int **pp = &p;
```
经典的叠叠乐，在这里我们需要解引用两次来获取变量a的值。虽然看上去很复杂，实际上它的原理很简单：
```

pp -------> p --------> a


```
通过这个关系图，你就会发现实际上它就和数学中的换元差不多，一直抽丝剥茧就能得到最终结果。

我们来看看它的应用吧：

```C
int **arr;
int rows = 3, cols = 4;
arr = (int **)malloc(rows * sizeof(int *));
for (int i = 0; i < rows; i++) {
    arr[i] = (int *)malloc(cols * sizeof(int));
}
```
在这个例子中，arr 是一个双级指针，它指向一个包含多个指针的数组，每个指针都指向一个整数数组（即二维数组的每一行）。

我们来看看它的asm实现：

```
        .file   "main.c"
        .text
        .globl  main
        .type   main, @function
main:
.LFB6:
        .cfi_startproc
        pushq   %rbp
        .cfi_def_cfa_offset 16
        .cfi_offset 6, -16
        movq    %rsp, %rbp
        .cfi_def_cfa_register 6
        pushq   %rbx
        subq    $40, %rsp
        .cfi_offset 3, -24
        movl    $3, -24(%rbp)
        movl    $4, -28(%rbp)
        movl    -24(%rbp), %eax
        cltq
        salq    $2, %rax
        movq    %rax, %rdi
        call    malloc
        movq    %rax, -40(%rbp)
        movl    $0, -20(%rbp)
        jmp     .L2
.L3:
        movl    -28(%rbp), %eax
        cltq
        salq    $2, %rax
        movl    -20(%rbp), %edx
        movslq  %edx, %rdx
        leaq    0(,%rdx,8), %rcx
        movq    -40(%rbp), %rdx
        leaq    (%rcx,%rdx), %rbx
        movq    %rax, %rdi
        call    malloc
        movq    %rax, (%rbx)
        addl    $1, -20(%rbp)
.L2:
        movl    -20(%rbp), %eax
        cmpl    -24(%rbp), %eax
        jl      .L3
        movl    $0, %eax
        movq    -8(%rbp), %rbx
        leave
        .cfi_def_cfa 7, 8
        ret
        .cfi_endproc
.LFE6:
        .size   main, .-main
        .ident  "GCC: (GNU) 11.5.0 20240719 (Red Hat 11.5.0-5)"
        .section        .note.GNU-stack,"",@progbits
```
在这里，它首先计算了需要`malloc`的内存大小，然后调用了`malloc`，在`.LFB6`段最后进入循环：

初始化 `-20(%rbp)` 为0，用作循环计数器。 在循环中，每次迭代都会分配内存并存储在`-40(%rbp)`指向的内存区域中`-28(%rbp)`表示列数`-20(%rbp)`是行数索引。`leaq 0(,%rdx,8), %rcx`计算了偏移量，乘以 8 是因为在这里我们分配的是指针，指针占有8字节。最后`leaq (%rcx,%rdx), %rbx`将偏移量和 malloc 返回的指针结合，计算出目标内存地址。

实际上这里的多重指针表现在汇编上也并不复杂，这段代码的难点在于循环。

### 作用域和所有权问题

一提到手动内存管理，这两个问题就一直是大头，它们可能导致许多问题。

指针作为一个变量，它本身的作用域和普通变量一样，看一个C代码:

```C

```
