---
title: JVM如何管理内存？

abstract: 抽象，太抽象！为了让你不紧盯内存，jvm做了什么？

picture: none

category: 0
---
Java虚拟机（JVM）作为Java程序的运行环境，其内存管理机制是保障程序运行效率和稳定性的重要基础。JVM将其管理的内存划分为多个区域，每个区域都有特定的用途，并在程序运行的不同阶段创建和销毁。了解JVM的内存管理机制对于优化Java程序的性能、理解垃圾回收原理以及解决内存泄漏问题至关重要。

为了阅读的流畅性，建议对java和C++有一个大概的了解，了解内存分配机制更好。
## JVM内存模型概述
JVM的内存模型主要包括两大类区域：线程共享区域和线程私有区域。每个区域的作用和生命周期有所不同，下面将详细介绍这些区域。

### 线程共享区域
1. 方法区（Method Area）
   方法区是所有线程共享的内存区域，主要用于存储以下信息：

- 类信息：包括类的名称、访问修饰符、字段和方法等。
- 常量：例如字符串常量和基本类型常量。
- 静态变量：属于类的静态字段。
- JIT编译后的代码：用于提高代码执行效率。

在JDK 8之前，方法区被实现为永久代（PermGen）。从JDK 8开始，方法区被元空间（Metaspace）替代。元空间与永久代不同，它存储在本地内存中，大小不再受堆内存的限制，可以根据需求动态调整。

2. 堆（Heap）
   堆是JVM管理的内存中最大的一块区域，几乎所有的对象实例及其数组都在堆中分配。堆空间是垃圾收集器管理的主要区域，因此堆空间又常被称为GC堆（Garbage Collected Heap）。根据Java虚拟机规范，Java堆可以是物理上不连续的内存空间，但逻辑上必须是连续的。

    堆内存通常被划分为不同的代（Generations）：

- 新生代（Young Generation）：新创建的对象首先被分配到新生代。新生代又分为伊甸区（Eden）和两个幸存者区（Survivor）。
- 老年代（Old Generation）：经历多次垃圾回收仍然存活的对象会被移动到老年代。
- 永久代（PermGen）/元空间（Metaspace）：存储类元数据，从JDK 8开始被元空间替代。

### 线程私有区域
1. 程序计数器（Program Counter Register）
   程序计数器是线程私有的一个较小内存区域，记录当前线程所执行的字节码指令的地址。多线程的情况下，通过切换线程保存各线程的执行状态。

2. 虚拟机栈（Java Virtual Machine Stack）
   虚拟机栈是线程私有的区域，生命周期与线程相同。每个方法被调用时，JVM会为其创建一个栈帧，用于存储局部变量、操作数栈、动态链接和方法返回地址等信息。随着方法的执行，栈帧不断进出栈。

3. 本地方法栈（Native Method Stack）
   本地方法栈与虚拟机栈类似，但它为本地方法服务。它主要用于执行本地方法，特别是在调用操作系统的底层C/C++代码时使用。

## JVM的垃圾回收机制

垃圾回收 `Garbage Collection`是JVM的一项关键特性，负责自动管理内存，回收不再使用的对象。JVM中的垃圾回收主要集中在堆内存上，通常采用以下几种回收算法：

- 标记-清除算法：标记出需要回收的对象，随后清除这些对象。缺点是容易导致内存碎片。
- 标记-整理算法：在标记阶段后，将存活的对象向一端移动，然后清理掉边界之外的内存。
- 复制算法：将存活的对象复制到另一个空间，然后清空原来的空间。该算法通常用于新生代。
- 分代收集算法：将堆内存划分为新生代和老年代，根据对象的生命周期进行不同的回收策略。

## 示例
了解了这些前置基础知识后，我们来看一个例子:
```Java
public class MemoryManagementExample {
    public static void main(String[] args) {
        // 创建对象
        Person person = new Person("Alice", 25);
        person.sayHello();

        // 调用静态方法
        printInfo();
    }

    static void printInfo() {
        // 静态变量存储在方法区
        System.out.println("This is a static method.");
    }
}

class Person {
    String name;
    int age;

    public Person(String name, int age) {
        this.name = name;
        this.age = age;
    }

    void sayHello() {
        // 局部变量存储在栈中
        String greeting = "Hello, my name is " + name + " and I am " + age + " years old.";
        System.out.println(greeting);
    }
}

```
### 栈:

栈遵循后进先出原则，当一个方法调用结束后，对应的栈空间和其中的局部变量会被销毁，不由GC管理。

当`MemoryManagementExample.main()`方法被调用时，JVM会在栈中为这个方法创建一个栈帧。

在方法`main`中，局部变量`person`（引用堆中的Person）存储在栈中。

当`Person.sayHello()`方法被调用时，`greeting`字符串局部变量也会被存储在栈中。

方法调用结束后，对应的栈帧会被销毁，栈中的局部变量随之被清除。

### 堆（Heap）:

堆负责所有对象实例和数组，几乎所有动态分配的内存都在这里。

`new Person("Alice", 25)`会在堆中分配内存，用于存储Person对象实例。

`name`和`age`两个实例变量也会作为`Person`对象的一部分存储在堆中。

堆内存由GC负责管理，当`person`对象不再被引用时，它的内存会被垃圾收集器回收。

### 元空间(Meta Space):

元空间使用操作系统的物理内存而不是JVM的内存，所以元空间的大小不受限于JVM的堆内存限制，而是受到操作系统内存大小的约束。

元空间是动态的，JVM可以根据需要自动扩展或收缩元空间的内存。

如果元空间的内存不足，JVM会尝试扩展元空间；如果元空间的内存使用量较少，JVM会进行垃圾回收以释放不再使用的内存。

在JVM启动时，元空间会加载类的信息，当一个类被加载时，类的结构、方法信息、字段信息等会被存储在元空间中，在这里，

`Person`类的信息（如类的结构、方法、常量池），

`MemoryManagementExample.printInfo()`方法，

静态变量和常量（如printInfo方法的静态内容）都会放在元空间中。


## GC原理

我们会发现，在整个程序的执行过程中，GC进行了大部分的操作。

垃圾收集的主要目标是释放程序中不再使用的对象所占用的内存。这些对象可能是由于程序逻辑错误、对象生命周期结束等原因不再被引用。GC的工作是确保这些对象的内存空间可以被重新分配，以提高内存使用效率。

我将详细介绍几种常用的GC算法。

###  标记-清除算法（Mark-Sweep）

这种算法会从根对象开始，遍历内存中所有可以达到的对象，并且标记它们为活动对象。在标记完成之后，它将遍历堆中的所有对象，并且回收未标记的对象。

###  标记-整理算法（Mark-Compact）

这种算法的标记部分和标记-清除一致，不同之处在于它会将所有活动的对象移动到内存的一端，随后清理所有可用范围外的内存。

### 复制算法（Copying）

它会将内存分为`From`和`To`区域，将所有活动的对象迁移到`To`区域，随后清理`From`区域。

### 分代收集算法（Generational Collection）

它会将堆内存分为多个代（通常是新生代和老年代）：

- 新生代（Young Generation）：存放新创建的对象。新生代包括一个Eden区和两个Survivor区。
- 老年代（Old Generation）：存放长时间存活的对象。

还是来看一个例子：

```Java
public class MarkSweepExample {
    public static void main(String[] args) {
        // 创建对象
        Person person1 = new Person("Alice", 30);
        Person person2 = new Person("Bob", 25);

        // person1 和 person2 现在都引用了 Person 对象
        
        // 打断引用
        person1 = null;
        person2 = null;

        // 强制触发垃圾回收
        System.gc();
        
        // 程序结束
    }

    static class Person {
        String name;
        int age;
        Person(String name, int age) {
            this.name = name;
            this.age = age;
        }
    }
}

```

当 person1 和 person2 被设为 null 时，之前引用的 Person 对象就不再被任何活动的引用所持有。

对于 Mark-Sweep,在GC过程中，这些不再被引用的 Person 对象会被标记为垃圾对象并被清除。

而对于 Mark-Compact,在整理阶段，活动对象会被移动到内存的前端。由于 person1 和 person2 不再引用这些对象，它们会被清除，而剩余的对象会被移动到堆的开始位置，以消除内存碎片。

```Java 
public class GenerationalGCExample {
    public static void main(String[] args) {
        // 创建对象
        for (int i = 0; i < 1000; i++) {
            new Person("Person" + i, i);
        }
        Person usefulPerson = new Person("Wang", 15);
        // 强制触发垃圾回收
        System.gc();
        System.out.println(usefulPerson.name);
        // 程序结束
    }

    static class Person {
        String name;
        int age;
        Person(String name, int age) {
            this.name = name;
            this.age = age;
        }
    }
}
```

在这个程序中我们创建了大量的Person对象到内存，其中还含有一个会在gc之后被引用的usefulPerson。

对于Copying, 在新生代中，所有活动对象会被复制到另一个区域（To区），在这里只有usefulPerson，而旧的 From区会在之后清除。
所有对象在复制的过程中会被保留，只有在复制完成后，原来的 From区 的内存才会被释放。

对于Generational,它会将这些对象分代，
在刚开始，所有的对象都会被分配到新生代，而在gc之后，因为usefulPerson还有引用，所以Generational会倾向于将usefulPerson更改到老年代，因为它会被引用，之后会再
经历Mark-Sweep或Mark-Compact的GC过程。新生代GC通常使用复制算法来减少内存碎片，而老年代则会使用其他算法进行垃圾回收。

你可能会疑惑，为什么这里会使用age参数来定义不同的Person？这是因为jvm存在着`对象池化（Object Pooling）`,`标量替换（Scalar Replacement）`,`对象合并（Object Deduplication）`,`同步优化（Synchronization Optimization）`,`内存分配优化（Memory Allocation Optimization）`等优化技术。

## JVM的神奇优化手段

### 对象池化

对象池化是一个非常有用和易于理解的概念，它会重复使用已经创建的对象，而不是每次都创建新的对象。JVM在内部实现了一些优化策略来减少对象的重复创建：

- 整数缓存: 
    jvm会缓存在-128到127范围内的Integer对象，每次请求这个范围内的Integer对象时，JVM都会返回相同的实例，也就是说
``` Java
Integer a = 100;
Integer b = 100;
System.out.println(a == b); // 输出 true
```
（这就是某道臭名昭著的面试题来源）

- 字符串驻留：

JVM会缓存字符串字面量和String.intern()方法中使用的字符串。字符串驻留机制将字符串常量池保存在方法区（或元空间），确保相同内容的字符串使用同一对象引用。这可以减少内存使用并提高字符串比较的效率。

```Java
String s1 = "Hello";
String s2 = "Hello";
System.out.println(s1 == s2); // 输出 true

String s3 = new String("Hello");
System.out.println(s1 == s3); // 输出 false

String s4 = new String("Hello").intern();
System.out.println(s1 == s4; // 输出 true
```
对于new String("Hello")，虽然内容相同，但创建的是一个新的对象 **(Java不符合直觉的设计之一，在kotlin中直接使用==可以比较字符串，在Java中请使用equals)** 。而使用intern()方法可以将其返回字符串常量池中的对象。

### 标量替换：

标量替换的基本思想是将一个对象的字段直接替换为基本类型的局部变量，避免在堆上分配对象的内存。这样可以减少内存分配和垃圾回收的开销，从而提高程序的性能。

当一个对象包含多个字段时，标量替换会将这些字段拆分成独立的标量变量（例如整数、浮点数等）， 这些标量变量会被直接存储在寄存器或栈上，而不是在堆中分配内存。

因为GC不负责寄存器和栈，所以在提升访问速度的同时还减小了GC开销。

```Java
public class ScalarReplacementExample {
    public static void main(String[] args) {
        // 创建Point对象
        Point p = new Point(10, 20);
        // 使用Point对象
        int x = p.getX();
        int y = p.getY();
        System.out.println("x: " + x + ", y: " + y);
    }
    
    static class Point {
        private int x;
        private int y;
        
        public Point(int x, int y) {
            this.x = x;
            this.y = y;
        }
        
        public int getX() {
            return x;
        }
        
        public int getY() {
            return y;
        }
    }
}
```
经过优化后，大概会成这样：
```Java
public class ScalarReplacementOptimized {
    public static void main(String[] args) {
        // 直接使用标量变量
        int x = 10;
        int y = 20;
        System.out.println("x: " + x + ", y: " + y);
    }
}
```
它的实现机制也很特殊，通常在编译器优化阶段实现。现代JVM中的JIT Compiler会执行这一优化。JIT编译器会分析程序的运行时行为，确定是否进行标量替换。

逃逸分析（Escape Analysis）：JIT编译器使用逃逸分析来判断对象的生命周期。如果对象的引用没有逃逸出方法或线程，JIT编译器可能会将其标量替换。

内联（Inlining）：JIT编译器可能会将对象的方法内联，从而有更多机会进行标量替换。例如，将对象方法的调用替换为直接的标量变量访问。

### 对象合并：

对象合并是一种优化技术，旨在减少内存中小对象的数量，通过将多个小对象合并为一个大对象或合并对象的字段，来减少内存分配和管理的开销。这种技术可以减少内存碎片，提高内存的使用效率，并减轻垃圾回收的负担。

对于一些相似的对象，jvm将多个对象的公共字段合并为一个对象，并使用该对象来存储所有相关信息。这可以减少对象的数量，从而减少内存的分配和管理开销。

同时还存在对象池，它可以重用对象实例，而不是每次都创建新的对象，对象池可以减少频繁的对象创建和销毁，从而减少内存开销和垃圾回收的负担。

```Java
public class OriginalExample {
    public static void main(String[] args) {
        // 创建多个小对象
        for (int i = 0; i < 1000; i++) {
            new SmallObject(i, "Object" + i);
        }
    }
    
    static class SmallObject {
        int id;
        String name;
        
        SmallObject(int id, String name) {
            this.id = id;
            this.name = name;
        }
    }
}
```
在优化后会变成类似这样：
```Java
public class OptimizedExample {
    public static void main(String[] args) {
        // 合并字段
        CoalescedObject coalescedObject = new CoalescedObject(1000);
        for (int i = 0; i < 1000; i++) {
            coalescedObject.setField(i, "Object" + i);
        }
    }
    
    static class CoalescedObject {
        int[] ids;
        String[] names;
        
        CoalescedObject(int size) {
            ids = new int[size];
            names = new String[size];
        }
        
        void setField(int index, String name) {
            ids[index] = index;
            names[index] = name;
        }
    }
}

```

### 同步优化

同步优化是一种用于提高多线程程序性能的技术，特别是处理共享资源时的同步操作。同步优化的目标是减少锁的竞争，降低锁的开销，并提高程序的并发性能。

- 锁消除（Lock Elimination）：

锁消除是指JVM在编译阶段检测到锁操作实际上不会导致竞争时，将锁操作消除，
如果一个锁在程序的某个部分没有被共享，那么JVM可以优化掉这个锁操作。

- 锁粗化（Lock Coarsening）：

锁粗化是指将多个连续的锁操作合并为一个大的锁区域，以减少锁的获取和释放次数，这有助于减少锁的频繁操作，提高程序的执行效率。

- 偏向锁（Biased Locking）：

偏向锁是一种优化技术，旨在减少在单线程环境下锁的竞争，
偏向锁会将锁的拥有者标记为当前线程，以避免在同一线程中重复获取锁。

- 轻量级锁（Lightweight Locking）：

轻量级锁是为了减少锁竞争的开销而设计的。

在锁竞争不激烈的情况下，轻量级锁使用自旋锁来避免使用重量级锁的开销。

- 自适应自旋（Adaptive Spinning）：

自适应自旋是一种自适应技术，能够根据锁的竞争情况调整自旋的时间。
如果锁的竞争较轻，自旋的时间会更长；如果锁的竞争较重，则会尽快进入阻塞状态。
~~虽然这东西和内存没什么关系 但是既然实现上也能优化内存所以就提了~~

### 内存分配优化技术

内存分配优化是提高程序性能和内存使用效率的重要手段。JVM使用多种优化技术来改进内存分配过程，这些技术旨在减少内存碎片、提高内存分配速度、降低垃圾回收开销等。

- 空闲列表（Free List）

空闲列表是一种用于管理小对象的分配技术。通过维护一个空闲对象的链表，可以快速分配和释放小对象，从而减少内存碎片和分配开销。

它为每种大小的对象维护一个空闲列表， 在对象分配时，从相应大小的空闲列表中取出对象。 释放对象时，将其返回到空闲列表中。

- 内存对齐（Memory Alignment）

内存对齐用于优化内存访问速度和减少内存碎片，通过确保数据结构在内存中的对齐，可以提高CPU访问内存的效率，并减少缓存失效的可能性。

一般会确保数据结构按特定的对齐边界进行分配，例如4字节或8字节对齐，在对象分配时来调整对象的起始地址以满足对齐要求。

```C++
#include <iostream>
#include <cstddef> 

// 定义一个要求8字节对齐的结构体
struct alignas(8) AlignedStruct {
    int a;
    char b;
    double c;
};

int main() {
    // 创建一个对齐的结构体实例
    AlignedStruct obj;

    std::cout << "Address of obj: " << &obj << std::endl;
    std::cout << "Offset of a: " << offsetof(AlignedStruct, a) << std::endl;
    std::cout << "Offset of b: " << offsetof(AlignedStruct, b) << std::endl;
    std::cout << "Offset of c: " << offsetof(AlignedStruct, c) << std::endl;
    
    return 0;
    /*
    在笔者电脑上的输出：
    Address of obj: 0x16b37eef8
    Offset of a: 0
    Offset of b: 4
    Offset of c: 8
    */
}

```

- 逃逸分析（Escape Analysis）

逃逸分析是一种用于优化对象分配的技术，通过分析对象的“逃逸”范围来减少内存分配的开销。逃逸分析可以确定对象是否逃出方法或线程的范围，从而决定是否可以进行栈上分配或其他优化。

对于不逃逸的方法或线程内创建的对象，可以进行栈上分配，同时避免不必要的堆分配，减少垃圾回收负担。

- 内存池（Memory Pool）

内存池是一种将内存划分为固定大小块的技术，用于管理对象的分配和释放。通过预先分配一块内存，并将其分成多个小块，可以提高内存分配和回收的效率，它会创建一个大的内存池，将其划分为固定大小的块，
在对象分配时，从内存池中分配一个块，
在对象释放时，将块归还到内存池中，而不是释放回操作系统。
这里有一个C++实现：

```C++
#include <iostream>
#include <cstddef>
#include <cassert>
#include <new>

class MemoryPool {
public:
    explicit MemoryPool(std::size_t blockSize, std::size_t poolSize);
    ~MemoryPool();

    void* allocate();
    void deallocate(void* ptr);

private:
    struct Block {
        Block* next;
    };

    std::size_t blockSize;
    std::size_t poolSize;
    char* pool;
    Block* freeBlocks;
};

// Constructor to initialize the memory pool
MemoryPool::MemoryPool(std::size_t blockSize, std::size_t poolSize)
    : blockSize(blockSize), poolSize(poolSize), freeBlocks(nullptr) {
    // Ensure blockSize is aligned
    assert(blockSize >= sizeof(Block));

    // Allocate memory for the pool
    pool = new char[blockSize * poolSize];

    // Initialize free blocks list
    freeBlocks = reinterpret_cast<Block*>(pool);
    Block* current = freeBlocks;

    for (std::size_t i = 1; i < poolSize; ++i) {
        current->next = reinterpret_cast<Block*>(pool + i * blockSize);
        current = current->next;
    }
    current->next = nullptr;
}

// Destructor to clean up the memory pool
MemoryPool::~MemoryPool() {
    delete[] pool;
}

// Allocate a block from the pool
void* MemoryPool::allocate() {
    if (!freeBlocks) {
        throw std::bad_alloc();
    }

    Block* block = freeBlocks;
    freeBlocks = freeBlocks->next;
    return block;
}

// Deallocate a block and return it to the pool
void MemoryPool::deallocate(void* ptr) {
    Block* block = reinterpret_cast<Block*>(ptr);
    block->next = freeBlocks;
    freeBlocks = block;
}

```

- 直接内存（Direct Memory）

直接内存是一种用于绕过JVM堆内存的技术，直接在操作系统的内存中分配对象。直接内存可以提高I/O操作的效率，减少垃圾回收的开销。

```Java
import java.nio.ByteBuffer;

public class DirectMemoryExample {
    public static void main(String[] args) {
        ByteBuffer buffer = ByteBuffer.allocateDirect(1024);
        // Use buffer for direct memory operations
    }
}
```
这里相当于C中的`malloc()`。

## 总结

了解JVM内存管理和垃圾回收机制不仅对开发者优化程序性能至关重要，也可以帮助我们深入理解Java程序的执行过程。对于jvm这么一个内存上的“黑箱”，更深入的了解能够让我们排除更多的错误，了解jvm的更多实现细节。如果有任何内容上的问题，欢迎直接与笔者交流提出问题，不胜感激。
