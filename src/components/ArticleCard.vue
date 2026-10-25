<template>
	<article
		class="apple-newsroom-card"
		@click="navigateToArticle"
	>
		<!-- Card Media -->
		<div class="card-media-wrapper">
			<img
				v-if="article.imagePath && article.imagePath !== 'none'"
				:src="article.imagePath"
				:alt="article.title"
				class="card-img"
				loading="lazy"
			/>
			<div v-else class="card-placeholder-gradient" :style="getGradientStyle(article.category)">
				<span class="placeholder-tag">{{ article.category || 'NEWS' }}</span>
			</div>
		</div>

		<!-- Card Content -->
		<div class="card-content">
			<div class="card-meta">
				<span class="card-category-badge">{{ getBadgeText(article.category) }}</span>
				<span v-if="article.date" class="card-date">{{ article.date }}</span>
			</div>

			<h3 class="card-title">{{ article.title }}</h3>

			<p class="card-abstract">{{ article.abstract }}</p>

			<div class="card-footer">
				<span class="read-more-link">
					阅读全文
					<svg width="14" height="14" viewBox="0 0 16 16" fill="none" class="arrow-icon">
						<path d="M6 3.5l4.5 4.5L6 12.5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
					</svg>
				</span>
				<span v-if="article.ai_generated" class="ai-indicator" title="此文章包含AI辅助内容">
					AI
				</span>
			</div>
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

		const getBadgeText = (cat) => {
			if (!cat) return 'UPDATE';
			if (cat === '技术') return 'TECH';
			if (cat === '闲聊') return 'STORIES';
			if (cat === '时事') return 'PRESS RELEASE';
			if (cat === '文章/小说') return 'CREATIVE';
			return cat.toUpperCase();
		};

		const getGradientStyle = (cat) => {
			if (cat === '技术') {
				return { background: 'linear-gradient(135deg, #0f2027 0%, #203a43 50%, #2c5364 100%)' };
			}
			if (cat === '时事') {
				return { background: 'linear-gradient(135deg, #1e3c72 0%, #2a5298 100%)' };
			}
			if (cat === '闲聊') {
				return { background: 'linear-gradient(135deg, #434343 0%, #000000 100%)' };
			}
			return { background: 'linear-gradient(135deg, #2b5876 0%, #4e4376 100%)' };
		};

		return {
			navigateToArticle,
			getBadgeText,
			getGradientStyle,
		};
	},
};
</script>

<style scoped>
.apple-newsroom-card {
	background: #ffffff;
	border-radius: var(--apple-radius-card, 24px);
	overflow: hidden;
	display: flex;
	flex-direction: column;
	cursor: pointer;
	position: relative;
	border: 1px solid rgba(0, 0, 0, 0.08);
	box-shadow: 0 4px 20px rgba(0, 0, 0, 0.04);
	transition: transform 0.28s cubic-bezier(0.16, 1, 0.3, 1), box-shadow 0.28s cubic-bezier(0.16, 1, 0.3, 1), border-color 0.28s ease;
}

.apple-newsroom-card:hover {
	transform: translateY(-6px);
	box-shadow: 0 16px 36px rgba(0, 0, 0, 0.1);
	border-color: rgba(0, 0, 0, 0.14);
}

/* Card Media */
.card-media-wrapper {
	width: 100%;
	height: 220px;
	overflow: hidden;
	position: relative;
	background-color: #f5f5f7;
}

.card-img {
	width: 100%;
	height: 100%;
	object-fit: cover;
	display: block;
	transition: transform 0.35s cubic-bezier(0.16, 1, 0.3, 1);
}

.apple-newsroom-card:hover .card-img {
	transform: scale(1.04);
}

.card-placeholder-gradient {
	width: 100%;
	height: 100%;
	display: flex;
	align-items: center;
	justify-content: center;
	color: rgba(255, 255, 255, 0.85);
	font-weight: 700;
	font-size: 16px;
	letter-spacing: 0.1em;
}

.placeholder-tag {
	padding: 6px 14px;
	background: rgba(255, 255, 255, 0.15);
	backdrop-filter: blur(10px);
	border-radius: 980px;
	border: 1px solid rgba(255, 255, 255, 0.2);
}

/* Card Content */
.card-content {
	padding: 24px 24px 20px;
	display: flex;
	flex-direction: column;
	flex: 1;
}

.card-meta {
	display: flex;
	align-items: center;
	justify-content: space-between;
	margin-bottom: 10px;
}

.card-category-badge {
	font-size: 11px;
	font-weight: 700;
	letter-spacing: 0.06em;
	color: #86868b;
	text-transform: uppercase;
}

.card-date {
	font-size: 12px;
	color: #86868b;
}

.card-title {
	font-size: 20px;
	font-weight: 700;
	line-height: 1.3;
	color: #1d1d1f;
	margin: 0 0 10px;
	letter-spacing: -0.015em;
	display: -webkit-box;
	-webkit-line-clamp: 2;
	-webkit-box-orient: vertical;
	overflow: hidden;
	transition: color 0.15s ease;
}

.apple-newsroom-card:hover .card-title {
	color: #0071e3;
}

.card-abstract {
	font-size: 14px;
	line-height: 1.55;
	color: #515154;
	margin: 0 0 18px;
	display: -webkit-box;
	-webkit-line-clamp: 3;
	-webkit-box-orient: vertical;
	overflow: hidden;
	flex: 1;
}

/* Card Footer */
.card-footer {
	display: flex;
	align-items: center;
	justify-content: space-between;
	padding-top: 14px;
	border-top: 1px solid #f0f0f2;
}

.read-more-link {
	font-size: 13px;
	font-weight: 500;
	color: #0071e3;
	display: inline-flex;
	align-items: center;
	gap: 4px;
	transition: gap 0.15s ease;
}

.apple-newsroom-card:hover .read-more-link {
	gap: 8px;
}

.arrow-icon {
	transition: transform 0.15s ease;
}

.ai-indicator {
	font-size: 10px;
	font-weight: 700;
	padding: 2px 8px;
	border-radius: 980px;
	background: #f5f5f7;
	color: #86868b;
	border: 1px solid #e5e5ea;
}
</style>
