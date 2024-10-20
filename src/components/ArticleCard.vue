<!-- ArticleCard.vue -->
<template>
	<div class="article" @click="redirectToArticle">
		<img :src="article.imagePath" alt="文章图片" class="article-image"/>
		<div class="article-info">
			<h2 class="artc">{{ article.title }}</h2>
			<h2 class="abstract">{{ article.abstract }}</h2>
			<p class="category"><img src="@/assets/category.svg">{{ article.category }}</p>
		</div>
	</div>
</template>

<script>
export default {
	name: 'ArticleCard',
	props: {
		article: {
			type: Object,
			required: true
		}
	},
	methods: {
		redirectToArticle() {
			window.location.href = `/article/${this.article.id}`;
		}
	}
};
</script>

<style scoped>
.article {
	background-color: #435060;
	border-radius: 40px;
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
	max-height: 400px;
	border-radius: 30px;
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
.category {
	align-items: center;
	color: #fffdfd;
	font-weight: 100;
	display: flex;
	padding: 0;
}
</style>
