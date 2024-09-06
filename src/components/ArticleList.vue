<template>
	<div class="content">
		<h1 class="introduction">文章列表</h1>
		<div class="article" v-for="article in articles" :key="article.id" @click="redirect(article.id)">
			<img :src="`/articles/${article.id}.jpg`" alt="文章图片" class="article-image" />
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
.article {
	border: 2px solid #869cb6;
	border-radius: 10px;
	padding: 20px 50px;
	max-width: 55vw;
	margin: auto;
	min-height: 20vh;
	margin-top: 5vh;
	transition: background-color 0.3s, scale 0.3s;
	text-align: left;
}

.article-image {
	width: 100%;
	height: auto;
	min-height: 200px;
	border-top-left-radius: 28px;
	border-top-right-radius: 28px;
	margin-bottom: 10px;
}

.article:hover {
	background-color: #435060;
	scale: 1.02;
}
.artc {
	margin-top: 10px; /* 标题与图片的间距 */
}

.abstract {
	margin-top: 5px; /* 简介与标题的间距 */
}
</style>
