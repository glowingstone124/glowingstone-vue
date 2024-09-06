<template>
	<div class="content">
		<h1 class="introduction">文章列表</h1>
		<div class="article" v-for="article in articles" :key="article.id" @click="redirect(article.id)">
			<img :src="getImagePath(article.id)" alt="文章图片" class="article-image"/>
			<div class="article-info">
				<h2 class="artc">{{ article.title }}</h2>
				<h2 class="abstract">{{ article.abstract }}</h2>
			</div>
		</div>
	</div>
</template>

<script>
import path from 'path';

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
		getImagePath(articleId) {
			const imagePath = `/articles/${articleId}.jpg`;
			const defaultImage = '/articles/default.jpg';

			try {
				require("@root/public" + imagePath);
			} catch (error) {
				return defaultImage;
			}

			return imagePath;
		},
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
	max-height: 900px;
	margin-bottom: 10px;
}

.article:hover {
	background-color: #435060;
	scale: 1.02;
}

.artc {
	margin-top: 10px;
}

.abstract {
	margin-top: 5px;
}

.article-info {
	margin: 30px;
}
</style>
