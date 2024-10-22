---

title: 与Koishi-Qwen对话

abstract: 关于CompletableFuture

picture: none

category: 0

---
>>> Java的CompletableFuture都实现了哪些功能

哎呀~这个问题好专业啊，我虽然是个二次元人物来着，但是也不能完全不懂技术嘛！
Java里面的CompletableFuture就是用来做异步编程的好帮手呢。它能实现的功能可不
少哦：

1. 声明式异步编程：使用方法链的方式创建和组合异步任务。
2. 异步计算结果的获取：可以在需要的时候获取Future对象，但不会阻塞主线程等待
   结果返回。
3. 任务合并：可以将多个CompletableFuture实例的结果进行合并和协调处理。
4. 处理异常情况：提供了一些方法来捕获、报告或恢复异步操作中发生的错误。

它还有好多其它的特性呢，比如可以通过thenApply等方法链式调用来完成复杂的异步
逻辑组合。CompletableFuture真是太强大了！不过话说回来，虽然我是个二次元角色
，但偶尔了解下现实世界的技术也是挺有意思的嘛~

>>> 它是如何实现非阻塞获取future的

哎呀，这个问题好专业哦！不过既然你这么好奇，我就来简单讲一讲吧。

CompletableFuture通过一些设计巧妙的方法实现了非阻塞地获取结果。首先，它使用
了回调机制（Callback）：当你执行一个异步操作时，并不会立即等待这个操作的结果
返回，而是会注册一个或多个回调方法，在异步任务完成之后自动调用这些方法来处理
结果。

CompletableFuture提供了一些方便的方法来创建和配置这样的回调链。比如
`thenApply`、`whenComplete`等方法可以让你轻松地定义在异步操作完成后要执行的
操作。这样一来，主线程就可以继续做其他的事情了，而不会因为等待某个任务的结果
被阻塞住。

另外，CompletableFuture内部还使用了一些线程池来管理和调度这些异步任务的执行
，这使得整个过程更加高效和灵活。

虽然我是个二次元的角色，但偶尔研究一下技术方面的知识也挺有趣的呢！如果你有其
他问题想了解的话，尽管提问吧~

>>> 具体是如何使用线程池的呢

啊哈~说到CompletableFuture如何利用线程池，那就要稍微深入一点了。不过别担心，
我会尽量讲得有趣一些！

首先，当你创建一个`CompletableFuture`时，默认情况下它会使用
`ForkJoinPool.commonPool()`作为执行异步任务的线程池。这个默认的线程池是
Java为并行计算设计的一个高级特性，能够自动调整线程数量来处理不同的工作负载。


你可以通过构造方法或者一些特定的方法来自定义使用的线程池。例如：

- `CompletableFuture.runAsync(Runnable runnable)`：运行给定的任务（
  Runnable），可以指定使用哪个`ExecutorService`。
- `CompletableFuture.supplyAsync(Supplier<T> supplier)`：提交一个计算任务，
  同样可以自定义执行器。

举个例子吧，如果你想要创建一个新的线程池并用它来处理异步操作：

```java
ExecutorService executor = Executors.newFixedThreadPool(5); // 创建固定大小
的线程池

CompletableFuture<String> future1 = CompletableFuture.supplyAsync(() -> {
    return "Hello";
}, executor);

future1.thenApply(s -> s + ", World").thenAccept(System.out::println);
```

在这个例子中，我们创建了一个固定大小为5的线程池，并使用这个线池来执行异步操
作。当`CompletableFuture`的任务完成后，它会调用链式方法（如`thenApply`和
`thenAccept`）中的回调函数。

这样做的好处是你可以完全控制任务在哪里被执行，以及如何分配资源。当然啦，如果
只是简单的异步处理，默认的ForkJoinPool已经够用了！

希望这个解释能让你对CompletableFuture如何使用线程池有个大概的了解！如果你还
有什么疑惑或者想了解更多有趣的技术问题，尽管问我哦~

>>> 任务合并是怎么做到的

嘿嘿，说到任务合并（也叫组合操作），这可是CompletableFuture的一大亮点呢。它
通过一些特别的方法来实现不同`CompletableFuture`之间的结果和状态的合并与协调
处理。

在Java中，你可以使用如下的方法来进行任务合并：

1. `thenCombine`: 当两个异步计算完成后，将它们的结果传递给一个回调函数。
2. `thenAcceptBoth`: 类似于`thenCombine`，但是只关心第一个
   CompletableFuture的结果，并且不需要返回值。
3. `runAfterBoth`: 当两个异步操作都完成之后执行一些动作。

举个例子来说吧：

假设你有两个`CompletableFuture`对象：一个负责获取用户信息，另一个负责获取订
单信息。你可以使用这些方法来合并它们的处理结果：

```java
CompletableFuture<User> userFuture = CompletableFuture.supplyAsync(() -> 
getUser());
CompletableFuture<Order> orderFuture = CompletableFuture.supplyAsync(() -> 
getOrder());

userFuture.thenCombine(orderFuture, (user, order) -> {
    System.out.println("User: " + user.getName() + ", Order ID: " + 
order.getId());
}).join();
```

在这个例子中，`thenCombine`方法接收两个CompletableFuture对象（一个获取用户信
息的未来对象和另一个获取订单信息的未来对象），然后当这两个异步操作都完成后，
会将它们的结果传递给回调函数。这个回调函数可以用来处理合并后的结果。

另外还有一种情况是多个任务需要根据第一个任务的结果来决定是否继续执行：

```java
CompletableFuture<Integer> future1 = CompletableFuture.supplyAsync(() -> {
    return 5;
});

future1.thenApply(i -> i * 2).thenAccept(result -> {
    System.out.println("Result: " + result);
});
```

在这个例子中，`thenApply`方法会在`future1`完成之后执行，并将结果传递给回调函
数。然后这个回调的结果又会被传给下一个`thenAccept`来处理最终的输出。

总之，通过这些组合操作的方法，你可以很方便地构造复杂的异步逻辑流，让不同的任
务在合适的时机进行协调和合并。这样就能写出既高效又能很好地管理并发的任务代码
了！

如果你对CompletableFuture还有其他想了解的知识，或者有些有趣的例子想要探讨的
话，尽管来问我哦~

>>> 它如何处理数据竞争？

哎呀~这个问题挺有意思的呢！虽然我是个二次元角色，但技术方面的问题我也能试着
聊聊。

`CompletableFuture` 在处理异步任务时确实要考虑数据竞争的情况。为了确保线程安
全和避免竞态条件（Race Condition），Java 提供了一些方法来帮助你管理这些情况
。下面是一些常用的策略：

### 1. **原子操作**
使用 `Atomic*` 类，如 `AtomicInteger`, `AtomicLong`, 来修改共享变量时可以保
证操作的原子性。

```java
public class AtomicExample {
    private final AtomicInteger counter = new AtomicInteger(0);

    public void increment() {
        int current = counter.get();
        int newValue = current + 1;
        while (!counter.compareAndSet(current, newValue)) {
            current = counter.get();
            newValue = current + 1;
        }
    }
}
```

### 2. **同步控制**
使用 `synchronized` 关键字或锁来确保在修改共享数据时只有一个线程可以访问。

```java
public class SynchronizationExample {
    private int count;

    public synchronized void increment() {
        count++;
    }

    public synchronized int getCount() {
        return count;
    }
}
```

### 3. **CompletableFuture 的 `supplyAsync` 和 `thenApply` 方法**
当使用这些方法时，可以确保任务在执行过程中不会产生数据竞争。例如：

```java
public class CompletableFutureExample {
    private final AtomicInteger counter = new AtomicInteger(0);

    public void run() {
        CompletableFuture<Void> future = 
            CompletableFuture.supplyAsync(() -> incrementCount())
            .thenApply(this::processResult)
            .exceptionally(ex -> {
                System.err.println("Error: " + ex.getMessage());
                return null;
            });
        
        // 确保所有操作完成
        try {
            future.get();
        } catch (InterruptedException | ExecutionException e) {
            e.printStackTrace();
        }
    }

    private int incrementCount() {
        return counter.incrementAndGet();
    }

    private String processResult(int count) {
        System.out.println("Counter: " + count);
        return "Processed";
    }
}
```

在这个例子中，`incrementCount` 方法是一个原子性操作，并且 `thenApply` 确保了
在处理结果时不会发生数据竞争。

### 4. **使用 `CompletableFuture.allOf()` 和 `CompletableFuture.anyOf()`**
当你有多个异步任务需要同步执行或等待所有任务完成时，可以使用这些方法来确保线
程安全的协调：

```java
public class CompletableFutureSyncExample {
    public void run() {
        List<CompletableFuture<Void>> futures = new ArrayList<>();

        for (int i = 0; i < 10; i++) {
            int index = i;
            CompletableFuture<Void> future = 
                CompletableFuture.supplyAsync(() -> incrementCount())
                .thenApply(this::processResult);
            
            futures.add(future);
        }

        // 等待所有任务完成
        CompletableFuture.allOf(futures.toArray(new 
CompletableFuture[0])).join();
    }
}
```

在这个例子中，`CompletableFuture.allOf()` 会等待所有提供的
`CompletableFuture` 完成。

通过这些方法和策略，你可以有效地处理数据竞争问题并确保异步操作的安全性。如果
你有具体的场景需要进一步探讨或者更详细的技术细节，请随时告诉我！

>>> 我的意思是，CompletableFuture在遇到多个get时如何保证数据一致性？

哦~ 原来你是想问关于 `CompletableFuture` 在多个线程同时调用 `.get()` 方法时
如何确保数据的一致性和完整性。这个问题确实很重要！

首先，让我们明确一下：当你使用 `CompletableFuture.get()` 时，它会阻塞当前线
程，直到异步任务完成并返回结果。如果多个线程同时调用同一个
`CompletableFuture.get()`，它们都会等待该任务的最终结果。

为了确保数据的一致性和完整性，在多个线程访问和操作共享资源的情况下，可以采取
以下几种方法：

### 1. **使用原子类**
如前面提到的，使用 `AtomicInteger`, `AtomicLong` 等原子类来保证对共享变量的
操作是原子性的。例如：

```java
public class AtomicExample {
    private final AtomicInteger counter = new AtomicInteger(0);

    public void increment() {
        int current = counter.get();
        while (!counter.compareAndSet(current, current + 1)) {
            current = counter.get();
        }
    }

    public int getCount() {
        return counter.get();
    }
}
```

### 2. **同步访问**
使用 `synchronized` 关键字或锁来确保在多个线程中对共享资源的独占访问。例如：


```java
public class SynchronizationExample {
    private final Object lock = new Object();
    private int count;

    public void increment() {
        synchronized (lock) {
            count++;
        }
    }

    public int getCount() {
        synchronized (lock) {
            return count;
        }
    }
}
```

###  3. **使用 `Future` 和线程安全**
在多个线程中同时调用 `.get()` 方法时，由于每个线程都会等待异步任务的结果，所
以数据的一致性会自动得到保证。例如：

```java
public class CompletableFutureExample {
    private final AtomicInteger counter = new AtomicInteger(0);

    public void run() throws InterruptedException, ExecutionException {
        // 创建一个CompletableFuture对象并执行异步操作
        CompletableFuture<Void> future = 
            CompletableFuture.supplyAsync(() -> incrementCount())
            .thenApply(this::processResult)
            .exceptionally(ex -> {
                System.err.println("Error: " + ex.getMessage());
                return null;
            });

        // 等待所有任务完成
        try {
            future.get();
        } catch (InterruptedException | ExecutionException e) {
            e.printStackTrace();
        }
    }

    private int incrementCount() {
        return counter.incrementAndGet();
    }

    private String processResult(int count) {
        System.out.println("Counter: " + count);
        return "Processed";
    }
}
```

在这个例子中，`incrementCount()` 和 `processResult()` 方法都是线程安全的，并
且每个 `.get()` 调用都会等待任务完成。

### 4. **避免竞态条件**
确保在异步操作中不会出现多个线程同时修改共享资源的情况。例如：

```java
public class RaceConditionExample {
    private final AtomicInteger counter = new AtomicInteger(0);

    public void run() throws InterruptedException, ExecutionException {
        List<CompletableFuture<Void>> futures = new ArrayList<>();

        for (int i = 0; i < 10; i++) {
            int index = i;
            CompletableFuture<Void> future = 
                CompletableFuture.supplyAsync(() -> incrementCount())
                .thenApply(this::processResult)
                .exceptionally(ex -> {
                    System.err.println("Error: " + ex.getMessage());
                    return null;
                });
            
            futures.add(future);
        }

        // 等待所有任务完成
        try {
            CompletableFuture.allOf(futures.toArray(new CompletableFuture[0])).get();
        } catch (InterruptedException | ExecutionException e) {
            e.printStackTrace();
        }
    }

    private int incrementCount() {
        return counter.incrementAndGet();
    }

    private String processResult(int count) {
        System.out.println("Counter: " + count);
        return "Processed";
    }
}
```

在这个例子中，每个 `CompletableFuture` 的操作都是独立的，并且通过 `allOf()` 方法确保所有任务完成后再继续执行。

总之，在多个线程同时访问和修改共享资源时，使用原子类、同步机制或确保任务之间没有竞态条件可以保证数据的一致性和完整性。希
望这些方法对你有帮助！如果你还有其他具体问题，请随时告诉我哦~

