---

title: 打造美观的api

abstract: 本教程指导如何让您的API更加美观，易用。

---
# 打造美观的api

## 非常典型的错误示范...

在开始之前，我们先来看几个例子

你是一名前端工程师，今天你的甲方给了你一个接口，让你获取图片并且在页面中显示

```https://example.com/huoqutupianlianjie```

你捏着鼻子接受了这个任务，向指定的url发送了一个请求，url返回了：

```https://example.com/jtk/tupian.png``` (同时HTTP response code是888，你从来没有听说过这个code)

我的天哪...这个示例简直令人血压升高。我们来看看它犯下了哪些错误/不规范的的地方：

- endpoint极其不规范(指用拼音命名而且全部小写)
- 返回体并没有用json包装 （_RESTFUL并没有规定一定使用某种返回体格式，但是json作为一种标准的结构化数据格式，更易于读取和处理。_）
- 不遵守HTTP Response Code规范

但是没有办法，这个接口是甲方给的，你只能继续硬着头皮再用这个api请求一次

结果甲方似乎忘记上传图片了，```https://example.com/jtk/tupian.png```返回了404，可是Response Code还是200，返回的String是五个汉字：

**资源不存在**

你气得想摔键盘，你学到的那些http规范在这里没有发挥任何作用，但是活不能不干，你只能手动判断equals:
``` JavaScript
if (response !== "资源不存在") {
    // ...
} else {
    Document.getElementById("image").src = response
}
```
...如果我是这个程序员我想我会直接辞职。

## 救回这个api

当然，钱还是要赚的。恰好你认识甲方的后端工程师，于是你轰炸了三天他的微信，他终于决定依照你的建议重写这个api，但是在重写之前，他强烈要求你给他做个示范。

*既然你要我重写我肯定要知道干什么*

那么好，我们来从零开始重写api。

一个 _标准的_ RESTful API应该: 

- 在URL设计上遵循 动词(verb) + 宾语(object) 原则 同时全部小写

    例如刚刚的那个例子，请求url应当为 `GET /picture`
- 服务端返回的状态码必须精确合理

    在第一次请求中，服务端正确返回了值，请求成功。此时应当返回 200 (HTTP OK)状态码方便前后端对接。

    在第二次请求中，服务端没有找到该图片但是返回了HTTP OK，正确的做法是返回 404 NOT FOUND.

    对于状态码释义，请参考[WIKI](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes)
- 对于需要提供参数的值，参数应当为  全小写  使用下划线

    例如使用`example.com/picture?id=9&image_source=png`代表返回id为9，格式为png的图片。

- 对于不同类型的Content，应当在header中声明Content_Type.

    例如`Content-Type: application/json`