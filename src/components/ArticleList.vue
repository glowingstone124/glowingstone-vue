<template>
  <div class="content">
    <h1 class="introduction">文章列表</h1>
    <div class="article" v-for="article in articles" :key="article.id" @click="redirect(article.id)">
      <h2 class="artc">{{ article.title }}</h2>
      <h2 class="abstract">{{ article.abstract }}</h2>
    </div>
  </div>
</template>

<script>
import path from 'path'; // 引入path模块

export default {
  name: 'ArticleList',
  data() {
    return {
      articles: [],
    };
  },
  async mounted() {
    const files = require.context("@root/public/articles/", true, /\.md$/);
    const fm = require('front-matter');
    const filelist = files.keys();

    for (const key of filelist) {
      const data = fm(files(key));
      const fileName = path.basename(key);
      this.articles.push({
        id: fileName,
        title: data.attributes.title,
        abstract: data.attributes.abstract,
      });
    }
  },
  methods: {
    redirect(articleId) {
      this.$router.push(`/article/${articleId}`);
    },
  },
};
</script>

<style scoped>
@import "@/css/main.css";
</style>
