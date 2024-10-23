<template>
	<div class="article" :class="{ ai: article.ai_generated }" @click="redirectToArticle">
		<div class="image-container" :style="{ backgroundImage: `url(${article.imagePath})` }">
			<div class="article-info">
				<h2 class="artc">{{ article.title }}</h2>
				<h2 class="abstract">{{ article.abstract }}</h2>
				<p class="category"><img src="@/assets/category.svg">{{ article.category }}</p>
			</div>
		</div>
	</div>
</template>

<script>
export default {
	name: 'ArticleCard_Next',
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
	min-height: 14vh;
	margin-top: 8vh;
	transition: background-color 0.3s, scale 0.3s;
	text-align: left;
	position: relative;
}

.article .ai {
	background-color: #507e58;
}

.article .ai:hover {
	background-color: #459454;
}

.image-container {
	background-size: cover;
	background-position: center;
	width: 100%;
	height: 400px;
	border-radius: 30px;
	transition: transform 0.3s;
	position: relative;
	display: flex;
	flex-direction: column;
	justify-content: flex-end;
}
.image-container:hover {
	backdrop-filter: blur(20px);
}

.article-info {
	padding-left: 40px;
	backdrop-filter: blur(0px);
	background-color: #1f1f1f50;
	transition: all 0.3s ease-in-out;
	border-radius: 30px;
	height:100%;
	position: absolute;
	bottom: 0;
	left: 0;
	right: 0;
}
.article-info:hover {
	backdrop-filter: blur(20px);
	background-color: #1f1f1f60;
}

.article:hover {
	backdrop-filter: blur(20px);
}

.artc {
	margin-top: 40px;
	font-weight: 200;
	font-size: 3rem;
}

.abstract {
	margin-top: 5px;
}


.category {
	align-items: center;
	color: #fffdfd;
	font-weight: 100;
	display: flex;
	padding: 0;
}

</style>
