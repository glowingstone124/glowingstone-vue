---
title: "Blog进化"
category: 0
abstract: "过去的几个月里，我都对博客做了些什么？"
picture: none
---
# Blog进化

自从我在23年末用vue搓了个博客之后，很长一段时间都没有去管它，最近突然想起来自己还有个博客，遂打算写点什么，打开blog，看到主页便释怀的死了。

这是一个行内公式：$V_{sphere}=\frac{4}{3}\pi r^3$，

[![pAYvIgK.png](https://s21.ax1x.com/2024/10/13/pAYvIgK.png)](https://imgse.com/i/pAYvIgK)

在自己没有前端经验的时候写出来的这一坨屎既不好看又不好用，于是为了日后的文章着想，**重构博客样式**这一伟大的计划便被提上了日程。初步重构结束之后便有了这一篇文章，权当总结。

首当其冲的是对页面进行评估，当时的博客总共包含了以下页面：
- 主页（包含全部文章的卡片）
- 友链
- 关于
## 主页
主页既然作为博客的门面，最先优化的自然是主页。当时的主页其实非常简陋，一个简单的title之下便是几篇文章，用户一眼就看到了头，折叠式导航栏也并不好用，用户导航到其他页面时必须先点按钮，完全是在大屏幕上非常抽象的设计。

于是，整个导航栏便进行了重新设计：

[![pAYxKrF.png](https://s21.ax1x.com/2024/10/13/pAYxKrF.png)](https://imgse.com/i/pAYxKrF)

将导航栏移动到了顶部，直接点击对应的组件就能快速移动，同时减少占用空间。背景做了个blur，配合半透明的背景色，可以同时应付纯色的内容页面和较为花哨的关于页面。

而博客也有了个名字：“Glowstone 萤石”

首页也做了个大改造，与之前的分区域背景色不同，这次背景变成了一个深色整体，title真正占了100vh，第一屏只显示标题。

同时删除了旧的文章大全，把文章分到了专门的存档页面，首页只留下部分pinned的文章，真正让首页起到导览作用。

顺便搬了一部分简单的自我介绍，字体颜色上也做了简单区分，效仿Metro风格做了个磁贴，放点短平快内容。

## 友链
然后便是友情链接部分了，这一部分其实设计上变化不大，就是微调了一些样式而已，顺便加入了我和博客的信息，方便单向添加友链（bushi

## 关于

关于部分是大改动，原来的关于长这样：

[![pAYxGP1.png](https://s21.ax1x.com/2024/10/13/pAYxGP1.png)](https://imgse.com/i/pAYxGP1)

简直是丑的没边了。。。

介绍页面和其他页面性质不同，更能体现用户个性，于是最先进行的其实是加了张背景图*（恋恋

还添加了个手写的动态svg作为标题，将以往的一大块内容做了div分割，flex实际上分了两条，毛玻璃+一些简单控件，更加优雅和年轻。

## 存档

存档页面参考了很大一部份的Material3设计，图片大头带圆角，顺便加了个简单的标签方便用户区分，还顺便裁剪了图片大小，保证相对一致的体验，点击一下就能跳转到对应文章。

## 画廊

画廊页面其实本来是没有的，后来发现有些照片很喜欢也很有意义，于是专门增加了这个组件。

画廊也是Material3风格，因为我真的很喜欢Material3。

还是分两块，左侧放日期和介绍，右侧照片主体，顺便做了lazyload，还自己用ktor搓了个图床，部署在一台很垃圾的香港服务器上。

## 文章页

外在的优化完了，文章的阅览体验也同样重要。

在视觉体验上，所有的文字都不是简单的白色，在明度上降低了一部分，更加适合阅读，不同字号的font weight也统一变成了细体，更加优雅（并非

在code block里面，我选择了`Fira Code`字体，使用`highlight.js`进行了着色。

顺便修了之前front-matter会莫名其妙显示的问题，感谢`gray-matter`和`marked`库实现md内容解析。

文章页的优化方向是：层级鲜明，阅览轻松。（还有酷）

# 总结

主要修改方向是易用，规范化和美观，目前第一阶段看下来也是比较成功的，体验还不错。

后续还会更新，因为优化还远远没有结束，许多样式上的小问题和优化还在planning当中。