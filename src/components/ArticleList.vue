<template>
  <div class="content">
    <h1 class="introduction">文章列表</h1>
    <div class="article" v-for="article in articles" :key="article.id">
      <h2 class="artc">{{ article.title }}</h2>
      <h2 class="abstract">{{ article.abstract }}</h2>
    </div>
  </div>
</template>

<script>
export default {
  name: 'ArticleList',
  data() {
    return {
      articles: [],
    };
  },
  async mounted() {
    const files = require.context("@root/public/articles/", true, /\.md$/);
    var fm = require('front-matter')
    console.log(files.keys())
    files.keys().forEach(async (key) => {
      console.log(files(key))
      const data  = fm(files(key));
      console.log(data);
      this.articles.push({
        id: this.articles.length + 1,
        title: data.attributes.title,
        abstract: data.attributes.abstract,
      });
    });
  },
};
</script>

<style scoped>
@import "@/css/main.css";
</style>
