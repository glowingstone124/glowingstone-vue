<template>
	<div class="archive-container">
		<!-- Section Header -->
		<div class="section-header">
			<h1 class="section-title">文章 <span class="sub">Articles</span></h1>
			<span class="section-meta font-mono">{{ filteredArticles.length }} / {{ articles.length }} 篇</span>
		</div>

		<!-- Filter & Search Console -->
		<div class="filter-console font-mono">
			<div class="search-input-box">
				<span class="search-label">搜索 / SEARCH:</span>
				<input
					v-model="searchQuery"
					type="text"
					placeholder="输入关键词实时筛选文章..."
					class="search-input"
				/>
				<button v-if="searchQuery" class="clear-btn" @click="searchQuery = ''">[清空]</button>
			</div>

			<!-- Category Filter Buttons -->
			<div class="category-tabs">
				<button
					v-for="cat in categories"
					:key="cat"
					class="cat-filter-btn"
					:class="{ active: selectedCategory === cat }"
					@click="selectedCategory = cat"
				>
					<span class="btn-bracket">[</span>
					{{ cat }}
					<span class="count-tag">({{ getCategoryCount(cat) }})</span>
					<span class="btn-bracket">]</span>
				</button>
			</div>
		</div>

		<!-- Bento Grid of Articles -->
		<div v-if="filteredArticles.length > 0" class="articles-bento-grid">
			<ArticleCard
				v-for="article in filteredArticles"
				:key="article.id"
				:article="article"
			/>
		</div>

		<!-- Empty Search State -->
		<div v-else class="empty-state font-mono">
			<p class="empty-msg">[ 未找到与 "{{ searchQuery }}" 匹配的文章 ]</p>
			<button class="metro-btn" @click="resetFilters">[重置筛选条件]</button>
		</div>

		<VisitCounter />
	</div>
</template>

<script>
import { ref, computed, onMounted } from 'vue';
import path from 'path';
import fm from "front-matter";
import ArticleCard from "@/components/ArticleCard.vue";
import VisitCounter from "@/components/VisitCounter.vue";

export default {
	name: 'ArticleList',
	components: { VisitCounter, ArticleCard },
	setup() {
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
							category = '未知。。。';
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
.archive-container {
	max-width: var(--max-content-width);
	margin: 0 auto 120px;
	padding: 0 40px;
	box-sizing: border-box;
}

/* Filter Console */
.filter-console {
	background: #121212;
	border: 1px solid #262626;
	padding: 32px 36px;
	margin-bottom: 48px;
	display: flex;
	flex-direction: column;
	gap: 22px;
}

.search-input-box {
	display: flex;
	align-items: center;
	gap: 14px;
	background: #0a0a0a;
	border: 1px solid #404040;
	padding: 14px 20px;
	transition: border-color 0.15s ease;
}

.search-input-box:focus-within {
	border-color: #ffffff;
}

.search-label {
	color: #a3a3a3;
	font-size: 0.9rem;
}

.search-input {
	flex-grow: 1;
	background: transparent;
	border: none;
	outline: none;
	color: #ffffff;
	font-family: inherit;
	font-size: 1rem;
}

.search-input::placeholder {
	color: #525252;
}

.clear-btn {
	background: transparent;
	border: none;
	color: #a3a3a3;
	cursor: pointer;
	font-family: inherit;
	font-size: 0.85rem;
}

.clear-btn:hover {
	color: #ffffff;
}

.category-tabs {
	display: flex;
	flex-wrap: wrap;
	gap: 12px;
}

.cat-filter-btn {
	background: #1a1a1a;
	border: 1px solid #333333;
	color: #a3a3a3;
	padding: 10px 22px;
	font-family: inherit;
	font-size: 0.88rem;
	cursor: pointer;
	display: flex;
	align-items: center;
	gap: 8px;
	transition: all 0.15s ease;
}

.cat-filter-btn .btn-bracket {
	color: #525252;
}

.cat-filter-btn:hover {
	color: #ffffff;
	border-color: #737373;
}

.cat-filter-btn.active {
	background: #ffffff;
	color: #000000;
	border-color: #ffffff;
	font-weight: 500;
}

.cat-filter-btn.active .btn-bracket {
	color: #000000;
}

.count-tag {
	font-size: 0.8rem;
	opacity: 0.8;
}

/* Articles Bento Grid */
.articles-bento-grid {
	display: grid;
	grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
	gap: 32px;
	margin-bottom: 40px;
}

/* Empty State */
.empty-state {
	text-align: center;
	padding: 80px 20px;
	background: #121212;
	border: 1px dashed #404040;
	margin: 20px 0 40px;
}

.empty-msg {
	color: #a3a3a3;
	font-size: 1rem;
	margin-bottom: 24px;
}

@media screen and (max-width: 768px) {
	.archive-container {
		padding: 0 20px;
	}
	.articles-bento-grid {
		grid-template-columns: 1fr;
	}
}
</style>
