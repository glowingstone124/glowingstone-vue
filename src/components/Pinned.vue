<template>
	<section class="metro-pinned-section">
		<!-- Section Header -->
		<div class="section-header">
			<h2 class="section-title">推荐 <span class="sub">Featured</span></h2>
			<router-link to="/archive" class="metro-btn font-mono">
				全部文章 &rarr;
			</router-link>
		</div>

		<!-- Metro Cards Grid -->
		<div class="pinned-grid">
			<article
				v-for="(article, idx) in articles"
				:key="article.id"
				class="metro-pinned-card"
				:class="idx === 0 ? 'card-lead' : 'card-side'"
				@click="goToArticle(article.id)"
			>
				<div class="card-cover-box">
					<img :src="article.imagePath" :alt="article.title" class="card-cover-img" />
				</div>
				<div class="card-info">
					<div class="card-meta font-mono">
						<span class="cat-label">[{{ article.category }}]</span>
						<span v-if="article.ai_generated" class="ai-label">[AI 生成]</span>
					</div>
					<h3 class="card-title">{{ article.title }}</h3>
					<p class="card-summary">{{ article.abstract }}</p>
					<div class="card-action font-mono">
						<span>阅读文章 &rarr;</span>
					</div>
				</div>
			</article>
		</div>
	</section>
</template>

<script>
import fm from 'front-matter';
import { ref } from 'vue';
import { useRouter } from 'vue-router';

export default {
	name: 'Pinned',
	setup() {
		const router = useRouter();
		const pins = ["Travelogue_of_Changning.md", "simple_intro_of_jmm.md", "blogging-evolution.md"];
		const articles = ref([]);

		const goToArticle = (articleId) => {
			router.push(`/article/${articleId}`);
		};

		(async () => {
			try {
				for (const key of pins) {
					const file = await import(`@root/public/articles/${key}`);
					const data = fm(file.default);

					let imagePath = '/default.jpg';
					const picture = data.attributes.picture;

					if (picture && picture !== "none") {
						imagePath = picture;
					}
					let category = '';
					switch (data.attributes.category) {
						case 0:
							category = '技术';
							break;
						case 1:
							category = '闲聊';
							break;
						case 2:
							category = '时事';
							break;
						default:
							category = '杂记';
					}
					articles.value.push({
						id: key,
						title: data.attributes.title || 'Untitled',
						abstract: data.attributes.abstract || '',
						imagePath: imagePath,
						category: category,
						ai_generated: data.attributes.ai_generated,
					});
				}
			} catch (error) {
				console.error("Error loading pinned articles:", error);
			}
		})();

		return { articles, goToArticle };
	}
};
</script>

<style scoped>
.metro-pinned-section {
	max-width: var(--max-content-width);
	margin: 0 auto 120px;
	padding: 0 40px;
	box-sizing: border-box;
}

.pinned-grid {
	display: grid;
	grid-template-columns: repeat(12, 1fr);
	gap: 28px;
}

.metro-pinned-card {
	background: #121212;
	border: 1px solid #262626;
	cursor: pointer;
	overflow: hidden;
	display: flex;
	flex-direction: column;
	transition: background-color 0.15s ease, border-color 0.15s ease;
}

.metro-pinned-card:hover {
	background: #1a1a1a;
	border-color: #ffffff;
}

.metro-pinned-card.card-lead {
	grid-column: span 7;
}

.metro-pinned-card.card-side {
	grid-column: span 5;
}

.card-cover-box {
	width: 100%;
	height: 260px;
	background: #0a0a0a;
	overflow: hidden;
	border-bottom: 1px solid #262626;
}

.card-lead .card-cover-box {
	height: 320px;
}

.card-cover-img {
	width: 100%;
	height: 100%;
	object-fit: cover;
	display: block;
}

.card-info {
	padding: 36px 40px;
	display: flex;
	flex-direction: column;
	flex-grow: 1;
	justify-content: space-between;
}

.card-meta {
	display: flex;
	gap: 12px;
	font-size: 0.85rem;
	margin-bottom: 14px;
}

.cat-label {
	color: #ffffff;
}

.ai-label {
	color: #a3a3a3;
}

.card-title {
	margin: 0 0 16px;
	font-size: 1.55rem;
	font-weight: 500;
	color: #ffffff;
	line-height: 1.35;
}

.card-summary {
	margin: 0 0 24px;
	font-size: 0.98rem;
	color: #a3a3a3;
	line-height: 1.75;
	display: -webkit-box;
	-webkit-line-clamp: 2;
	-webkit-box-orient: vertical;
	overflow: hidden;
}

.card-action {
	font-size: 0.88rem;
	color: #ffffff;
	padding-top: 18px;
	border-top: 1px solid #262626;
}

.metro-pinned-card:hover .card-action span {
	text-decoration: underline;
}

@media screen and (max-width: 900px) {
	.metro-pinned-card.card-lead,
	.metro-pinned-card.card-side {
		grid-column: span 12;
	}
	.metro-pinned-section {
		padding: 0 20px;
	}
}
</style>
