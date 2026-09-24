<template>
	<article
		class="metro-article-card"
		@click="navigateToArticle"
	>
		<!-- Card Header Bar -->
		<div class="card-bar font-mono">
			<span class="file-id">{{ article.id }}</span>
			<span class="cat-pill">[{{ article.category }}]</span>
		</div>

		<!-- Image Frame -->
		<div class="card-media">
			<img :src="article.imagePath" :alt="article.title" class="media-img" loading="lazy" />
		</div>

		<!-- Card Body -->
		<div class="card-body">
			<h2 class="card-title">{{ article.title }}</h2>
			<p class="card-abstract">{{ article.abstract }}</p>
		</div>

		<!-- Card Footer -->
		<div class="card-footer font-mono">
			<span class="read-btn">阅读全文 &rarr;</span>
			<span v-if="article.ai_generated" class="ai-flag">[AI]</span>
		</div>
	</article>
</template>

<script>
import { useRouter } from 'vue-router';

export default {
	name: 'ArticleCard',
	props: {
		article: {
			type: Object,
			required: true,
		},
	},
	setup(props) {
		const router = useRouter();

		const navigateToArticle = () => {
			router.push(`/article/${props.article.id}`);
		};

		return { navigateToArticle };
	},
};
</script>

<style scoped>
.metro-article-card {
	background: #121212;
	border: 1px solid #262626;
	overflow: hidden;
	display: flex;
	flex-direction: column;
	justify-content: space-between;
	cursor: pointer;
	position: relative;
	transition: background-color 0.2s ease, border-color 0.2s ease, transform 0.2s ease;
}

.metro-article-card:hover {
	background: #181818;
	border-color: #ffffff;
	transform: translateY(-4px);
}

/* Header Bar */
.card-bar {
	display: flex;
	align-items: center;
	justify-content: space-between;
	padding: 14px 24px;
	background: #0a0a0a;
	border-bottom: 1px solid #262626;
	font-size: 0.82rem;
}

.file-id {
	color: #737373;
	overflow: hidden;
	text-overflow: ellipsis;
	white-space: nowrap;
	max-width: 65%;
}

.cat-pill {
	color: #ffffff;
}

/* Media */
.card-media {
	width: 100%;
	height: 220px;
	overflow: hidden;
	background: #000000;
	border-bottom: 1px solid #262626;
}

.media-img {
	width: 100%;
	height: 100%;
	object-fit: cover;
	display: block;
	transition: transform 0.4s ease, filter 0.4s ease;
}

.metro-article-card:hover .media-img {
	transform: scale(1.05);
	filter: contrast(1.1);
}

/* Body */
.card-body {
	padding: 32px 30px;
	flex-grow: 1;
}

.card-title {
	margin: 0 0 14px;
	font-size: 1.45rem;
	font-weight: 500;
	color: #ffffff;
	line-height: 1.4;
	transition: color 0.15s ease;
}

.metro-article-card:hover .card-title {
	text-decoration: underline;
}

.card-abstract {
	margin: 0;
	font-size: 0.95rem;
	color: #a3a3a3;
	line-height: 1.7;
	display: -webkit-box;
	-webkit-line-clamp: 3;
	-webkit-box-orient: vertical;
	overflow: hidden;
}

/* Footer */
.card-footer {
	display: flex;
	justify-content: space-between;
	align-items: center;
	padding: 18px 30px;
	border-top: 1px solid #262626;
	background: #0e0e0e;
	font-size: 0.85rem;
}

.read-btn {
	color: #d4d4d4;
	border: 1px solid #383838;
	padding: 4px 10px;
	font-size: 0.82rem;
	transition: all 0.2s ease;
}

.metro-article-card:hover .read-btn {
	background: #ffffff;
	color: #000000;
	border-color: #ffffff;
}

.ai-flag {
	color: #737373;
}
</style>
