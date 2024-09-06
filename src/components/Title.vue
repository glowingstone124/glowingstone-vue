<template>
  <div class="hello">
    <div class="title">
        <div class="intro">
            <h1 class="tth1"><span style="font-weight: 200;">Glowstone </span>萤石</h1>
            <h3>{{ msg }}</h3>
        </div>
    </div>
    <div class="myself">
        <div class="intro">
            <h1 class="msh1">关于我</h1>
            <h3>从小学<span>四年级</span>开始学习 <span>Java</span></h3>
            <h3>拥有<span>六年</span>Java编码经验，正在学习<span>Kotlin, Rust, TypeScript</span>和<span>Functional Programming</span>，程序的最终目标是以<span>简单，优雅</span>的方式高效处理问题。</h3>
            <h3>技术栈：</h3>
            <img src="https://img.shields.io/badge/Java-ED8B00?style=for-the-badge&logo=openjdk&logoColor=white">
            <img src="https://img.shields.io/badge/Kotlin-0095D5?&style=for-the-badge&logo=kotlin&logoColor=white">
            <img src="https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vue.js&logoColor=4FC08D">
            <img src="https://img.shields.io/badge/Spring-6DB33F?style=for-the-badge&logo=spring&logoColor=white">
            <img src="https://img.shields.io/badge/HTML5-E34F26?style=for-the-badge&logo=html5&logoColor=white">
            <img src="https://img.shields.io/badge/Node.js-43853D?style=for-the-badge&logo=node.js&logoColor=white">
			<img src="https://img.shields.io/badge/Rust-orange?style=for-the-badge&logo=rust">
        </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Title',
  props: {
    msg: String
  }
}
</script>

<style scoped>
@import "@/css/main.css";
</style>
