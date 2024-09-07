<template>
	<div class="content">
		<h1 class="introduction">文章列表</h1>
		<div class="article" v-for="article in articles" :key="article.id" @click="redirect(article.id)">
			<img :src="article.imagePath" alt="文章图片" class="article-image"/>
			<div class="article-info">
				<h2 class="artc">{{ article.title }}</h2>
				<h2 class="abstract">{{ article.abstract }}</h2>
			</div>
		</div>
	</div>
</template>

<script>
import { reactive } from 'vue';
import path from 'path';
export default {
	name: 'ArticleList',
	setup() {
		const articles = reactive([]);

		const checkImageExistence = async (article) => {
			const imagePath = `/${article.id}.jpg`;
			const response = await fetch(window.location.protocol + "//" + window.location.host + imagePath);
			if (response.status === 200) {
				article.imagePath = imagePath;
			}
		};

		const redirect = (articleId) => {
			window.location.href = `/article/${articleId}`;
		};

		(async () => {
			const files = require.context("@root/public/articles/", true, /\.md$/);
			const fm = require('front-matter');
			const filelist = files.keys();

			for (const key of filelist) {
				const data = fm(files(key));
				const fileName = path.basename(key);
				const article = {
					id: fileName,
					title: data.attributes.title,
					abstract: data.attributes.abstract,
					imagePath: '/default.jpg'
				};
				articles.push(article);
				//await checkImageExistence(article);
			}
		})();

		return {articles, redirect};
	}
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
