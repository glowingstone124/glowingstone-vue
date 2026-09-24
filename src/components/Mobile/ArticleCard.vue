<template>
	<div class="article" @click="redirectToArticle">
		<div class="image-container">
			<img :src="article.imagePath" alt="文章图片" class="article-image"/>
		</div>
		<div class="article-info">
			<div class="meta-row font-mono">
				<span class="cat">[{{ article.category }}]</span>
				<span v-if="article.ai_generated" class="ai">[AI]</span>
			</div>
			<h2 class="artc">{{ article.title }}</h2>
			<p class="abstract">{{ article.abstract }}</p>
		</div>
	</div>
</template>

<script>
export default {
	name: 'ArticleCard',
	props: {
		article: {
			type: Object,
			required: true,
		},
	},
	methods: {
		redirectToArticle() {
			this.$router.push(`/article/${this.article.id}`);
		},
	},
};
</script>

<style scoped>
.article {
	background-color: #121212;
	border: 1px solid #262626;
	margin: 16px 0;
	text-align: left;
	cursor: pointer;
	transition: border-color 0.15s ease;
}

.article:hover {
	border-color: #ffffff;
}

.image-container {
	width: 100%;
	height: 180px;
	overflow: hidden;
	background: #000000;
	border-bottom: 1px solid #262626;
}

.article-image {
	object-fit: cover;
	width: 100%;
	height: 100%;
	display: block;
}

.article-info {
	padding: 16px;
}

.meta-row {
	display: flex;
	gap: 8px;
	font-size: 0.75rem;
	color: #a3a3a3;
	margin-bottom: 8px;
}

.cat {
	color: #ffffff;
}

.artc {
	margin: 0 0 8px;
	font-size: 1.15rem;
	color: #ffffff;
	font-weight: 500;
}

.abstract {
	margin: 0;
	font-size: 0.85rem;
	color: #a3a3a3;
	line-height: 1.6;
}
</style>
