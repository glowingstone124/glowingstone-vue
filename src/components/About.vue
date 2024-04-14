<template>
    <div class="contents">
      <h1 class="cagetory">关于我</h1>
      <div class="friends">
        <h1>我是谁？</h1>
        <p>我是glowingstone124，现居上海，是一名普通的准高中生。平时会玩玩apex，bf,cs等fps游戏，偶尔追追新番老番。</p>
        <p>技术上嘛...毫无建树，只是稍微会一些Java和Web而已，但是在努力学习。</p>
        <p>Miku厨，半个车车壬，群友们一致认为的抽象大手子</p>
        <p>(Github头像是芙兰，qq是恋恋)</p>
        <h1>关于这个网站</h1>
        <p>这个网站是完全自建的（你没听错，没有使用任何ui库和模板，纯vue手敲</p>
        <p>前端：Vue3</p>
        <img @click="redirectToGithub" src="https://socialify.git.ci/glowingstone124/glowingstone-vue/image?description=1&descriptionEditable=%E5%9C%A8github%E4%B8%8A%E6%89%BE%E5%88%B0%E6%88%91&font=Inter&language=1&name=1&owner=1&pattern=Plus&stargazers=1&theme=Dark">
        <p>没有后端，基于vercel部署啦</p>
        <h1>爱好</h1>
        <p>游戏、编程、动漫、自媒体、顺便还会偶尔写小说</p>
        <a href="https://steamcommunity.com/profiles/76561199236874427/">Steam帐号在此</a>
        <a href="https://space.bilibili.com/687290823">哗哩哗哩(故意的)账号</a>
        <a href="https://music.163.com/#/user/home?id=5164416595">网易云账号</a>
        <h2>音乐爱好</h2>
        <p>流行音乐以欧美，日本居多，还听不少acg <br> 摇滚爱好者！最爱的是万青和Neckdeep，还补了kon和孤独摇滚</p>
        <h1>联系我</h1>
        <p>qq:1294915648</p>
        <p>邮箱：hanseroffical@outlook.com(忽略这个地址，小孩子不懂事乱起的)</p>
    </div>
    </div>
    </template>
    <script>
    export default {
      name: 'About',
      components: {
        },
    methods: {
    redirectToGithub() {
      window.location.href = 'https://github.com/glowingstone124'; // 替换成你的Github地址
    }
  }
      }
    </script>
    
    <style scoped>
    @import "@/css/main.css";
    .contents{
      margin-top: 6vh;
    }
    .cagetory{
      font-size: 5em;
      font-weight: 500;
    }
    .friends{
      margin-top: 3vh;
    }
    p{
      margin-left: 25px;
      padding: 5px;
    }
    a {
      background-color: rgb(136, 160, 189);
      border-radius: 15px;
      padding: 10px;
      margin: 5px;
      text-decoration: none;
      transition: background-color 0.3s;
    }
    a:hover{
      background-color: rgb(107, 126, 150);
    }
    img{
        max-width: 40vw;
    }
    </style>
    