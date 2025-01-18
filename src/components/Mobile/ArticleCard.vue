<template>
	<div class="article" :class="{ ai: article.ai_generated }" @click="redirectToArticle">
		<div class="image-container">
			<img :src="article.imagePath" alt="文章图片" class="article-image"/>
		</div>
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
	border-radius: 10px !important;
	margin: 1vh auto auto;
	transition: background-color 0.3s, scale 0.3s;
	text-align: left;
	position: relative;
	padding-bottom: 15px;
}
.article .ai {
	background-color: #507e58;
}
.article .ai:hover {
	background-color: #459454;
}
.image-container {
	transition: transform 0.3s;
}

.article-image {
	object-fit: cover;
	width: 100%;
	height: auto;
	max-height: 400px;
	border-radius: 10px;
	margin-bottom: 0px;
}

.article:hover .image-container {
	transform: translateY(-4px);
}

.article:hover {
	background-color: #5a6c80;
}

.artc {
	margin: 0px;
	font-weight: 400;
}


.article-info {
	border-radius: 10px !important;
	margin: 2px 15px;
}
.abstract {

}

.category {
	align-items: center;
	color: #fffdfd;
	font-weight: 100;
	display: flex;
	padding: 0;
}
</style>
