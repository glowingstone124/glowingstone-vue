<template>
    <Nav/>
    <Title msg="High school student lives in Shanghai, personal Java & Web developer"/>
    <ArticleList :articles="articleList" />
    <NavBtn/>
    <Footer Footermsg="design & code by glowingstone124"/>
  </template>
  
  <script>
  import Title from './Title.vue'
  import ArticleList from './ArticleList.vue'
  import NavBtn from './NavBtn.vue';
  import Nav from './Nav.vue';
  import Footer from './Footer.vue';
  export default {
    name: 'App',
    components: {
      Title,
      ArticleList,
      NavBtn,
      Nav,
      Footer
    },
  }
  </script>
  <style>
  @import "@/css/main.css";
  </style>
  