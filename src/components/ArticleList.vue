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
import matter from 'gray-matter';

export default {
  name: 'ArticleList',
  data() {
    return {
      articles: [],
    };
  },
  async mounted() {
    const files = require.context("@root/public/articles/", true, /\.md$/);
    files.keys().forEach(async (key) => {
      const response = await fetch(files(key));
      const content = await response.text();
      const { data } = matter(content); // Pass { excerpt: false } to avoid Buffer usage
      this.articles.push({
        id: this.articles.length + 1,
        title: data.title,
        abstract: data.abstract,
      });
    });
  },
};
</script>

<style scoped>
@import "@/css/main.css";
</style>
