<template>
  <div class="hello">
    <div class="title">
        <div class="intro">
            <h1 class="tth1">Glowingstone124's blog</h1>
            <h3>{{ msg }}</h3>
        </div>
    </div>
    <div class="myself">
        <div class="intro">
            <h1 class="msh1">About my self</h1>
            <img src="https://github-readme-stats.vercel.app/api?username=glowingstone124&theme=blue-green">
            <img src="https://github-readme-streak-stats.herokuapp.com/?user=glowingstone124&theme=blue-green">
            <h3>Started learning java at Grade 4</h3>
            <h3>Working on a Minecraft Server Project<a href="https://blog.qoriginal.vip"> Here</a></h3>
            <h3>I learned</h3>
            <img src="https://img.shields.io/badge/Java-ED8B00?style=for-the-badge&logo=openjdk&logoColor=white">
            <img src="https://img.shields.io/badge/Kotlin-0095D5?&style=for-the-badge&logo=kotlin&logoColor=white">
            <img src="https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vue.js&logoColor=4FC08D">
            <img src="https://img.shields.io/badge/Spring-6DB33F?style=for-the-badge&logo=spring&logoColor=white">
            <img src="https://img.shields.io/badge/HTML5-E34F26?style=for-the-badge&logo=html5&logoColor=white">
            <img src="https://img.shields.io/badge/Node.js-43853D?style=for-the-badge&logo=node.js&logoColor=white">
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
