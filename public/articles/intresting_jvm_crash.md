---
title: 记一次有趣的JVM Crash

abstract: linux VS zgc，你们知道吗？

picture: none

category: 0

ai_generated: false

---

## 起因

我的一个JVM应用突然莫名其妙的出现了JVM Crash，报错信息很简单也很经典：

```
#
# A fatal error has been detected by the Java Runtime Environment:
#
#  Internal Error (xPhysicalMemoryBacking_linux.cpp:722), pid=1, tid=12
#  fatal error: Failed to map memory (Not enough space)
#
# JRE version: OpenJDK Runtime Environment Temurin-22.0.2+9 (22.0.2+9) (build 22.0.2+9)
# Java VM: OpenJDK 64-Bit Server VM Temurin-22.0.2+9 (22.0.2+9, mixed mode, sharing, tiered, compressed class ptrs, z gc, linux-amd64)
# Problematic frame:
# V  [libjvm.so+0x108788a]  XPhysicalMemoryBacking::unmap(unsigned long, unsigned long) const+0x4a
#
# Core dump will be written. Default location: Core dumps may be processed with "/usr/lib/systemd/systemd-coredump %P %u %g %s %t %c %h" (or dumping to /workspace/core.1)
#
# If you would like to submit a bug report, please visit:
#   https://github.com/adoptium/adoptium-support/issues
#
```

看上去又是一个经典的OOM报错，然而这次的情况不一样：我给JVM设置了-Xmx64G的参数，而我的宿主机拥有180GB的空余内存可以alloc，所以首先就可以排除这个错误的通常结局：内存不够。

那么事情就很蹊跷了，让我们分析一下：

## 分析

出于本能，我第一时间查了系统的 `dmesg` 和 `core dump`，没有发现 swap 被打爆、oom-killer 触发或者超限崩溃的迹象。

于是我换个角度看问题：既然不是内存不足，那是不是系统限制了“映射内存”的能力？

JVM Crash 日志中有一行关键字让我警觉起来：

```
fatal error: Failed to map memory (Not enough space)
```

这证明，jvm是在尝试map内存时崩溃的，那么既然排除了内存本身不够，让我们把目光转向map本身--问题会不会出在map上？当我排查jvm及其周边配置的时候，发现在jvm的启动配置内使用了ZGC。

因为这个服务需求低STW时间和大堆，在技术选型时我们一致选用了ZGC这个适合大堆和低延迟的GC，问题就出在这。

ZGC和传统的G1GC不同，它采用了一种新的内存分配模式： `Region-based Heap Layout`。它会把整个 Java 堆拆分为许多逻辑区域（Region）， 
他们的大小都在 2MB 到 16MB， 且都是通过 mmap() 分配的匿名内存页 ，每个 Region 都是“独立分配、独立回收、独立迁移”的单位。

由于我的堆大小是64G，所以光是这些Region就要占用 64GB / 2MB = 32768个mmap数量，并且ZGC在运行时还会给标记区域
，Remap页面，
C2编译区，
CodeCache，
线程栈，
Safepoint页等等等都mmap一段内存，而linux内核通过一个参数限制了mmap的总数量：```vm.max_map_count```，很不巧，我没有动过这个数字，它是65530。所以，解决这个问题的方法非常简单：加大这个数量即可。

## 解决

仅需一行指令：

```bash
sysctl -w vm.max_map_count=2147483647
```

就能解决这个问题，再也不会遇到mmap数量不够的bug了。

## 总结

这个坑非常隐蔽，但却足够致命：

这个无论如何看上去是OOM，实则是mmap的问题，是因为ZGC 的 mmap 策略和传统 GC 不同，依赖大量虚拟内存映射，触发了linux限制。 因此，如果你在生产环境中使用 ZGC（尤其是容器化部署），建议立即检查并设置 vm.max_map_count，不然某一天就会踩到我踩过的坑：180G 空闲内存却因为“Not enough space”崩溃，听起来像个冷笑话，但 JVM 不会笑。
