<template>
	<div class="newsroom-archive-page">
		<div class="archive-container">
			<!-- Section Header -->
			<div class="archive-header">
				<div class="header-titles">
					<span class="apple-badge">NEWSROOM ARCHIVE</span>
					<h1 class="archive-title">全部文章与报道</h1>
					<p class="archive-desc">
						探索关于 JVM 底层原理、系统架构设计、网络服务以及日常生活的最新思考。
					</p>
				</div>
				<div class="header-count-badge">
					共 {{ filteredArticles.length }} 篇文章
				</div>
			</div>

			<!-- Filter & Search Console -->
			<div class="filter-bar">
				<!-- Search Capsule Input -->
				<div class="search-box">
					<svg class="search-icon" width="16" height="16" viewBox="0 0 16 16" fill="none">
						<path d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z" fill="currentColor"/>
					</svg>
					<input
						v-model="searchQuery"
						type="text"
						placeholder="搜索文章标题、摘要或关键词..."
						class="search-input"
					/>
					<button v-if="searchQuery" class="clear-search-btn" @click="searchQuery = ''" aria-label="Clear search">✕</button>
				</div>

				<!-- Category Filter Pills -->
				<div class="category-pills">
					<button
						v-for="cat in categories"
						:key="cat"
						class="category-pill"
						:class="{ active: selectedCategory === cat }"
						@click="selectedCategory = cat"
					>
						{{ cat }}
						<span class="cat-count">{{ getCategoryCount(cat) }}</span>
					</button>
				</div>
			</div>

			<!-- Bento / 3-Column Grid of Articles -->
			<div v-if="filteredArticles.length > 0" class="articles-grid">
				<ArticleCard
					v-for="article in filteredArticles"
					:key="article.id"
					:article="article"
				/>
			</div>

			<!-- Empty Search State -->
			<div v-else class="empty-state">
				<div class="empty-icon-box">
					<svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
						<circle cx="11" cy="11" r="8"></circle>
						<line x1="21" y1="21" x2="16.65" y2="16.65"></line>
					</svg>
				</div>
				<h3 class="empty-title">未找到匹配的内容</h3>
				<p class="empty-desc">没有找到与 "{{ searchQuery }}" 相关的文章，建议尝试其他关键词或更换分类。</p>
				<button class="apple-btn-pill primary" @click="resetFilters">重置筛选条件</button>
			</div>

			<VisitCounter />
		</div>
	</div>
</template>

<script>
import { ref, computed, onMounted, watch } from 'vue';
import { useRoute } from 'vue-router';
import path from 'path';
import fm from "front-matter";
import ArticleCard from "@/components/ArticleCard.vue";
import VisitCounter from "@/components/VisitCounter.vue";

export default {
	name: 'ArticleList',
	components: { VisitCounter, ArticleCard },
	setup() {
		const route = useRoute();
		const articles = ref([]);
		const searchQuery = ref('');
		const selectedCategory = ref('全部');

		const categories = ['全部', '技术', '闲聊', '时事', '文章/小说'];

		const getCategoryCount = (cat) => {
			if (cat === '全部') return articles.value.length;
			return articles.value.filter((a) => a.category === cat).length;
		};

		const resetFilters = () => {
			searchQuery.value = '';
			selectedCategory.value = '全部';
		};

		// Sync query parameter from URL (e.g. /archive?q=JVM or /archive?category=技术)
		watch(
			() => route.query,
			(query) => {
				if (query.q) {
					searchQuery.value = query.q;
				}
				if (query.category && categories.includes(query.category)) {
					selectedCategory.value = query.category;
				}
			},
			{ immediate: true }
		);

		const filteredArticles = computed(() => {
			return articles.value.filter((article) => {
				const matchesCat =
					selectedCategory.value === '全部' ||
					article.category === selectedCategory.value;

				const q = searchQuery.value.trim().toLowerCase();
				if (!q) return matchesCat;

				const matchesQuery =
					article.title.toLowerCase().includes(q) ||
					article.abstract.toLowerCase().includes(q) ||
					article.id.toLowerCase().includes(q);

				return matchesCat && matchesQuery;
			});
		});

		onMounted(async () => {
			try {
				const files = require.context("@root/public/articles/", true, /\.md$/);
				const filelist = files.keys();

				for (const key of filelist) {
					const data = fm(files(key));
					const fileName = path.basename(key);

					let imagePath = '/default.jpg';
					const picture = data.attributes.picture;
					const ai = data.attributes.ai_generated;
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
						case 3:
							category = '文章/小说';
							break;
						default:
							category = '技术';
					}

					if (picture && picture !== "none") {
						imagePath = picture;
					}

					articles.value.push({
						id: fileName,
						title: data.attributes.title || fileName,
						abstract: data.attributes.abstract || '暂无摘要描述。',
						imagePath: imagePath,
						category: category,
						ai_generated: ai,
						date: data.attributes.date || '2026年10月',
					});
				}
			} catch (err) {
				console.error("Failed to load article list:", err);
			}
		});

		return {
			articles,
			searchQuery,
			selectedCategory,
			categories,
			filteredArticles,
			getCategoryCount,
			resetFilters,
		};
	},
};
</script>

<style scoped>
.newsroom-archive-page {
	width: 100%;
	padding: 40px 0 80px;
}

.archive-container {
	max-width: var(--apple-max-width, 1240px);
	margin: 0 auto;
	padding: 0 24px;
	box-sizing: border-box;
}

/* Header */
.archive-header {
	display: flex;
	justify-content: space-between;
	align-items: flex-end;
	margin-bottom: 40px;
	padding-bottom: 24px;
	border-bottom: 1px solid var(--apple-border, #e5e5ea);
	flex-wrap: wrap;
	gap: 20px;
}

.archive-title {
	font-size: 38px;
	font-weight: 700;
	color: #1d1d1f;
	margin: 8px 0 12px;
	letter-spacing: -0.02em;
}

.archive-desc {
	font-size: 16px;
	color: #86868b;
	max-width: 600px;
	line-height: 1.6;
}

.header-count-badge {
	font-size: 14px;
	font-weight: 500;
	color: #86868b;
	padding: 8px 18px;
	background: #f5f5f7;
	border-radius: 980px;
}

/* Filter Bar */
.filter-bar {
	display: flex;
	flex-direction: column;
	gap: 20px;
	margin-bottom: 40px;
}

.search-box {
	display: flex;
	align-items: center;
	gap: 12px;
	background: #f5f5f7;
	border-radius: 980px;
	padding: 12px 20px;
	border: 1px solid transparent;
	transition: all 0.2s ease;
	max-width: 580px;
}

.search-box:focus-within {
	background: #ffffff;
	border-color: #d2d2d7;
	box-shadow: 0 0 0 4px rgba(0, 113, 227, 0.12);
}

.search-icon {
	color: #86868b;
	flex-shrink: 0;
}

.search-input {
	flex: 1;
	border: none;
	background: transparent;
	font-size: 15px;
	color: #1d1d1f;
	outline: none;
}

.clear-search-btn {
	background: #e5e5ea;
	border: none;
	width: 20px;
	height: 20px;
	border-radius: 50%;
	display: flex;
	align-items: center;
	justify-content: center;
	font-size: 11px;
	color: #86868b;
	cursor: pointer;
}

.clear-search-btn:hover {
	background: #d2d2d7;
	color: #1d1d1f;
}

/* Category Pills */
.category-pills {
	display: flex;
	flex-wrap: wrap;
	gap: 10px;
}

.category-pill {
	display: inline-flex;
	align-items: center;
	gap: 6px;
	padding: 8px 18px;
	border-radius: 980px;
	font-size: 14px;
	font-weight: 500;
	background: #f5f5f7;
	color: #1d1d1f;
	border: 1px solid transparent;
	cursor: pointer;
	transition: all 0.18s ease;
}

.category-pill:hover {
	background: #e8e8ed;
}

.category-pill.active {
	background: #1d1d1f;
	color: #ffffff;
}

.cat-count {
	font-size: 12px;
	opacity: 0.7;
}

/* Articles Grid */
.articles-grid {
	display: grid;
	grid-template-columns: repeat(3, 1fr);
	gap: 28px;
}

/* Empty State */
.empty-state {
	text-align: center;
	padding: 80px 20px;
	background: #fbfbfd;
	border-radius: 24px;
	border: 1px dashed #d2d2d7;
	margin-top: 20px;
}

.empty-icon-box {
	color: #86868b;
	margin-bottom: 16px;
}

.empty-title {
	font-size: 20px;
	font-weight: 700;
	color: #1d1d1f;
	margin-bottom: 8px;
}

.empty-desc {
	font-size: 15px;
	color: #86868b;
	max-width: 480px;
	margin: 0 auto 24px;
	line-height: 1.5;
}

@media screen and (max-width: 1024px) {
	.articles-grid {
		grid-template-columns: repeat(2, 1fr);
		gap: 20px;
	}
}

@media screen and (max-width: 640px) {
	.articles-grid {
		grid-template-columns: 1fr;
	}
	.archive-title {
		font-size: 28px;
	}
}
</style>
