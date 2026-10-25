<template>
	<div class="newsroom-home">
		<div class="newsroom-container">

			<!-- 1. HERO SECTION: "Latest News" (Image 1) -->
			<section v-if="heroArticle" class="newsroom-section latest-news-section">
				<div class="section-title-row">
					<h2 class="newsroom-section-heading">Latest News</h2>
				</div>

				<div class="hero-split-card" @click="goToArticle(heroArticle.id)">
					<!-- Hero Image (Left) -->
					<div class="hero-card-media">
						<img
							v-if="heroArticle.imagePath && heroArticle.imagePath !== 'none'"
							:src="heroArticle.imagePath"
							:alt="heroArticle.title"
							class="hero-img"
						/>
						<div v-else class="hero-img-fallback">
							<span class="fallback-badge">{{ heroArticle.category }}</span>
						</div>
					</div>

					<!-- Hero Info (Right) -->
					<div class="hero-card-content">
						<div class="hero-badge-tag">
							{{ getBadgeName(heroArticle.category, 'UPDATE') }}
						</div>
						<h1 class="hero-title">{{ heroArticle.title }}</h1>
						<p class="hero-excerpt">{{ heroArticle.abstract }}</p>
						<div class="hero-date">{{ heroArticle.date }}</div>
					</div>
				</div>
			</section>

			<!-- 2. 3-COLUMN CARD GRID: Visual Feature Cards (Image 2) -->
			<section v-if="gridArticles.length > 0" class="newsroom-section feature-grid-section">
				<div class="feature-cards-grid">
					<div
						v-for="(article, idx) in gridArticles"
						:key="article.id"
						class="feature-visual-card"
						:style="getCardBgStyle(article)"
						@click="goToArticle(article.id)"
					>
						<div class="card-gradient-overlay"></div>
						<div class="card-bottom-info">
							<span
								class="feature-pill-badge"
								:class="getPillClass(article.category, idx)"
							>
								{{ getPillText(article.category, idx) }}
							</span>
							<h3 class="feature-card-title">{{ article.title }}</h3>
							<div class="feature-card-date">{{ article.date }}</div>
						</div>
					</div>
				</div>

				<!-- "View All" Pill Button (Image 2) -->
				<div class="view-all-wrapper">
					<router-link to="/archive" class="apple-pill-btn">
						View All
					</router-link>
				</div>
			</section>

			<!-- 3. "More from Apple Newsroom" 2-COLUMN LIST (Image 3) -->
			<section v-if="moreArticles.length > 0" class="newsroom-section more-news-section">
				<div class="section-title-row">
					<h2 class="newsroom-section-heading">More from Newsroom</h2>
				</div>

				<div class="more-news-grid">
					<div
						v-for="article in moreArticles"
						:key="article.id"
						class="more-news-item"
						@click="goToArticle(article.id)"
					>
						<!-- Left Thumbnail -->
						<div class="item-thumb-box">
							<img
								v-if="article.imagePath && article.imagePath !== 'none'"
								:src="article.imagePath"
								:alt="article.title"
								class="item-thumb-img"
								loading="lazy"
							/>
							<div v-else class="item-thumb-fallback">
								<span>{{ article.category }}</span>
							</div>
						</div>

						<!-- Right Info -->
						<div class="item-info-box">
							<div class="item-category-tag">
								{{ getBadgeName(article.category, 'UPDATE') }}
							</div>
							<h4 class="item-title">{{ article.title }}</h4>
							<div class="item-date">{{ article.date }}</div>
						</div>
					</div>
				</div>
			</section>

			<!-- 4. APPLE CUPERTINO TECH SPECS & PROFILE (Designed by Glowingstone) -->
			<section class="newsroom-section specs-section">
				<Title :msg="msg"/>
			</section>

			<!-- 5. JOURNALS / QUICK READS -->
			<section class="newsroom-section journals-section">
				<Journals />
			</section>

			<VisitCounter />
		</div>
	</div>
</template>

<script>
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import path from 'path';
import fm from 'front-matter';
import Title from './Title.vue';
import Journals from './Journals.vue';
import VisitCounter from './VisitCounter.vue';

export default {
	name: 'HomePage',
	components: {
		Title,
		Journals,
		VisitCounter,
	},
	data() {
		return {
			msg: "I got electric light, and I got second sight.\n" +
				"Got amazing powers of observation.\n" +
				"And that is how I know,\n" +
				"When I try to get through on the telephone to you,\n" +
				"There'll be nobody home.\n" +
				"— Pink Floyd 《The Wall》",
		};
	},
	setup() {
		const router = useRouter();
		const heroArticle = ref(null);
		const gridArticles = ref([]);
		const moreArticles = ref([]);

		const goToArticle = (articleId) => {
			router.push(`/article/${articleId}`);
		};

		const getBadgeName = (cat, fallback = 'UPDATE') => {
			if (!cat) return fallback;
			if (cat === '技术') return 'UPDATE';
			if (cat === '时事') return 'PRESS RELEASE';
			if (cat === '闲聊') return '⚡ QUICK READ';
			if (cat === '文章/小说') return 'PHOTOS & ESSAY';
			return cat.toUpperCase();
		};

		const getPillText = (cat, idx) => {
			if (idx === 0) return 'DEVELOPERS';
			if (idx === 1) return 'CREATIVES';
			if (idx === 2) return 'ENVIRONMENT';
			return cat || 'STORIES';
		};

		const getPillClass = (cat, idx) => {
			if (idx === 0) return 'pill-dev';
			if (idx === 1) return 'pill-creative';
			if (idx === 2) return 'pill-env';
			return 'pill-default';
		};

		const getCardBgStyle = (article) => {
			if (article.imagePath && article.imagePath !== 'none') {
				return {
					backgroundImage: `url(${article.imagePath})`,
					backgroundSize: 'cover',
					backgroundPosition: 'center',
				};
			}
			return {
				background: 'linear-gradient(135deg, #1d1d1f 0%, #3a3a3c 100%)',
			};
		};

		onMounted(async () => {
			try {
				const files = require.context("@root/public/articles/", true, /\.md$/);
				const filelist = files.keys();
				const loaded = [];

				for (const key of filelist) {
					const data = fm(files(key));
					const fileName = path.basename(key);

					let imagePath = '/default.jpg';
					const picture = data.attributes.picture;
					let category = '技术';

					switch (data.attributes.category) {
						case 0: category = '技术'; break;
						case 1: category = '闲聊'; break;
						case 2: category = '时事'; break;
						case 3: category = '文章/小说'; break;
						default: category = '技术';
					}

					if (picture && picture !== "none") {
						imagePath = picture;
					}

					loaded.push({
						id: fileName,
						title: data.attributes.title || fileName,
						abstract: data.attributes.abstract || '探索深度思考与系统设计架构。',
						imagePath: imagePath,
						category: category,
						ai_generated: data.attributes.ai_generated,
						date: data.attributes.date || 'October 20, 2026',
					});
				}

				// Prioritize articles with real pictures for hero & 3-column grid
				const withImages = loaded.filter(a => a.imagePath && a.imagePath !== '/default.jpg');
				const others = loaded.filter(a => !withImages.includes(a));
				const ordered = [...withImages, ...others];

				if (ordered.length > 0) {
					heroArticle.value = ordered[0];
				}
				if (ordered.length > 1) {
					gridArticles.value = ordered.slice(1, 4);
				}
				if (ordered.length > 4) {
					moreArticles.value = ordered.slice(4, 10);
				}
			} catch (err) {
				console.error("Failed to load newsroom articles:", err);
			}
		});

		return {
			heroArticle,
			gridArticles,
			moreArticles,
			goToArticle,
			getBadgeName,
			getPillText,
			getPillClass,
			getCardBgStyle,
		};
	},
};
</script>

<style scoped>
.newsroom-home {
	width: 100%;
	background-color: var(--apple-bg, #ffffff);
	padding: 20px 0 60px;
}

.newsroom-container {
	max-width: var(--apple-max-width, 1240px);
	margin: 0 auto;
	padding: 0 24px;
	box-sizing: border-box;
}

.newsroom-section {
	margin-bottom: 72px;
}

.section-title-row {
	margin-bottom: 24px;
}

.newsroom-section-heading {
	font-size: 32px;
	font-weight: 700;
	color: #1d1d1f;
	letter-spacing: -0.02em;
	margin: 0;
}

/* ==========================================================================
   1. HERO SPLIT CARD (Image 1)
   ========================================================================== */
.hero-split-card {
	display: grid;
	grid-template-columns: 1.25fr 1fr;
	background: #ffffff;
	border-radius: var(--apple-radius-hero, 28px);
	overflow: hidden;
	cursor: pointer;
	border: 1px solid rgba(0, 0, 0, 0.08);
	box-shadow: 0 4px 24px rgba(0, 0, 0, 0.05);
	transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1), box-shadow 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

.hero-split-card:hover {
	transform: translateY(-4px);
	box-shadow: 0 16px 40px rgba(0, 0, 0, 0.09);
}

.hero-card-media {
	width: 100%;
	min-height: 400px;
	max-height: 480px;
	overflow: hidden;
	background: #f5f5f7;
}

.hero-img {
	width: 100%;
	height: 100%;
	object-fit: cover;
	display: block;
	transition: transform 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}

.hero-split-card:hover .hero-img {
	transform: scale(1.03);
}

.hero-img-fallback {
	width: 100%;
	height: 100%;
	display: flex;
	align-items: center;
	justify-content: center;
	background: linear-gradient(135deg, #2c3e50, #3498db);
}

.fallback-badge {
	color: #ffffff;
	font-weight: 700;
	font-size: 18px;
	padding: 8px 20px;
	background: rgba(255, 255, 255, 0.2);
	backdrop-filter: blur(8px);
	border-radius: 980px;
}

.hero-card-content {
	padding: 48px 48px;
	display: flex;
	flex-direction: column;
	justify-content: center;
}

.hero-badge-tag {
	font-size: 12px;
	font-weight: 700;
	letter-spacing: 0.06em;
	color: #86868b;
	text-transform: uppercase;
	margin-bottom: 14px;
}

.hero-title {
	font-size: 34px;
	font-weight: 700;
	line-height: 1.2;
	color: #1d1d1f;
	margin: 0 0 16px;
	letter-spacing: -0.02em;
}

.hero-excerpt {
	font-size: 16px;
	line-height: 1.6;
	color: #515154;
	margin: 0 0 28px;
	display: -webkit-box;
	-webkit-line-clamp: 3;
	-webkit-box-orient: vertical;
	overflow: hidden;
}

.hero-date {
	font-size: 14px;
	color: #86868b;
}

/* ==========================================================================
   2. 3-COLUMN FEATURE CARDS GRID (Image 2)
   ========================================================================== */
.feature-cards-grid {
	display: grid;
	grid-template-columns: repeat(3, 1fr);
	gap: 24px;
	margin-bottom: 32px;
}

.feature-visual-card {
	position: relative;
	height: 480px;
	border-radius: var(--apple-radius-card, 24px);
	overflow: hidden;
	cursor: pointer;
	display: flex;
	align-items: flex-end;
	box-shadow: 0 4px 20px rgba(0, 0, 0, 0.06);
	transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1), box-shadow 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

.feature-visual-card:hover {
	transform: translateY(-6px);
	box-shadow: 0 16px 36px rgba(0, 0, 0, 0.14);
}

.card-gradient-overlay {
	position: absolute;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
	background: linear-gradient(to top, rgba(0, 0, 0, 0.88) 0%, rgba(0, 0, 0, 0.4) 45%, rgba(0, 0, 0, 0.05) 100%);
	pointer-events: none;
}

.card-bottom-info {
	position: relative;
	z-index: 2;
	padding: 28px 24px;
	width: 100%;
	box-sizing: border-box;
}

.feature-pill-badge {
	display: inline-block;
	padding: 4px 12px;
	border-radius: 980px;
	font-size: 11px;
	font-weight: 700;
	letter-spacing: 0.04em;
	text-transform: uppercase;
	color: #ffffff;
	margin-bottom: 12px;
}

.pill-dev {
	background: #1d1d1f;
	border: 1px solid rgba(255, 255, 255, 0.2);
}

.pill-creative {
	background: #6e3ff3;
}

.pill-env {
	background: #d85a1a;
}

.pill-default {
	background: rgba(255, 255, 255, 0.2);
	backdrop-filter: blur(6px);
}

.feature-card-title {
	font-size: 21px;
	font-weight: 700;
	line-height: 1.3;
	color: #ffffff;
	margin: 0 0 10px;
	letter-spacing: -0.015em;
	display: -webkit-box;
	-webkit-line-clamp: 3;
	-webkit-box-orient: vertical;
	overflow: hidden;
}

.feature-card-date {
	font-size: 13px;
	color: rgba(255, 255, 255, 0.7);
}

/* "View All" Button (Image 2) */
.view-all-wrapper {
	display: flex;
	justify-content: center;
	margin-top: 16px;
}

.apple-pill-btn {
	display: inline-block;
	padding: 10px 32px;
	background: #f5f5f7;
	color: #1d1d1f;
	border-radius: 980px;
	font-size: 14px;
	font-weight: 500;
	transition: background-color 0.18s ease, transform 0.15s ease;
}

.apple-pill-btn:hover {
	background: #e8e8ed;
	transform: scale(1.02);
}

/* ==========================================================================
   3. "MORE FROM APPLE NEWSROOM" 2-COLUMN LIST (Image 3)
   ========================================================================== */
.more-news-grid {
	display: grid;
	grid-template-columns: repeat(2, 1fr);
	column-gap: 48px;
}

.more-news-item {
	display: flex;
	gap: 20px;
	padding: 24px 0;
	border-top: 1px solid var(--apple-border, #e5e5ea);
	cursor: pointer;
	align-items: center;
	transition: opacity 0.15s ease;
}

.more-news-item:hover {
	opacity: 0.85;
}

.item-thumb-box {
	width: 120px;
	height: 100px;
	border-radius: var(--apple-radius-thumb, 18px);
	overflow: hidden;
	flex-shrink: 0;
	background: #f5f5f7;
}

.item-thumb-img {
	width: 100%;
	height: 100%;
	object-fit: cover;
	display: block;
}

.item-thumb-fallback {
	width: 100%;
	height: 100%;
	display: flex;
	align-items: center;
	justify-content: center;
	background: #f5f5f7;
	font-size: 12px;
	color: #86868b;
	font-weight: 600;
}

.item-info-box {
	display: flex;
	flex-direction: column;
	flex: 1;
}

.item-category-tag {
	font-size: 11px;
	font-weight: 700;
	letter-spacing: 0.05em;
	color: #86868b;
	text-transform: uppercase;
	margin-bottom: 6px;
}

.item-title {
	font-size: 18px;
	font-weight: 700;
	line-height: 1.35;
	color: #1d1d1f;
	margin: 0 0 8px;
	letter-spacing: -0.01em;
	display: -webkit-box;
	-webkit-line-clamp: 2;
	-webkit-box-orient: vertical;
	overflow: hidden;
}

.item-date {
	font-size: 13px;
	color: #86868b;
}

/* Responsive */
@media screen and (max-width: 960px) {
	.hero-split-card {
		grid-template-columns: 1fr;
	}
	.hero-card-media {
		min-height: 260px;
	}
	.hero-card-content {
		padding: 32px 24px;
	}
	.hero-title {
		font-size: 26px;
	}
	.feature-cards-grid {
		grid-template-columns: 1fr;
	}
	.feature-visual-card {
		height: 380px;
	}
	.more-news-grid {
		grid-template-columns: 1fr;
		column-gap: 0;
	}
}
</style>
