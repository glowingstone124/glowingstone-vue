<template>
	<div class="content">
		<h1 class="introduction">文章列表</h1>
		<div class="article" v-for="article in articles" :key="article.id" @click="redirect(article.id)">
			<ArticleCard :article="article"/>
		</div>
	</div>
</template>

<script>
import {ref} from 'vue';
import path from 'path';
import fm from "front-matter";
import ArticleCard from "@/components/ArticleCard.vue";

export default {
	name: 'ArticleList',
	components: {ArticleCard},
	setup() {
		const articles = ref([]);
		const redirect = (articleId) => {
			window.location.href = `/article/${articleId}`;
		};

		(async () => {
			const files = require.context("@root/public/articles/", true, /\.md$/);
			const filelist = files.keys();

			for (const key of filelist) {
				const data = fm(files(key));
				const fileName = path.basename(key);

				let imagePath = '/default.jpg';
				const picture = data.attributes.picture;
				let category = ''
				switch (data.attributes.category) {
					case 0:
						category = '技术'
						break;
					case 1:
						category = '闲聊'
						break;
					case 2:
						category = '时事'
						break;
					default:
						category = '未知。。。'
				}
				if (picture && picture !== "none") {
					imagePath = picture;
				}

				const article = {
					id: fileName,
					title: data.attributes.title,
					abstract: data.attributes.abstract,
					imagePath: imagePath,
					category: category,
				};
				articles.value.push(article);
			}
		})();

		return {articles, redirect};
	}
};

</script>

<style scoped>
.article {
	background-color: #435060;
	border-radius: 30px;
	max-width: 55vw;
	margin: auto;
	padding-bottom: 15px;
	min-height: 14vh;
	margin-top: 8vh;
	transition: background-color 0.3s, scale 0.3s;
	text-align: left;
}

.article-image {

	object-fit: cover;
	width: 100%;
	height: auto;
	max-height: 300px;
	margin-bottom: 10px;
}

.article:hover {
	background-color: #5a6c80;
}

.artc {
	margin-top: 10px;
	font-weight: 200;
}

.abstract {
	margin-top: 5px;
}

.article-info {
	margin: 30px;
}

.content {
	padding-top: 10vh;
}
</style>
