<template>
	<div class="gallery-page">
		<!-- Bandwidth Warning Modal -->
		<transition name="modal-fade">
			<div v-if="showWarn" class="cyber-warn-overlay font-mono">
				<div class="warn-bento-card">
					<div class="warn-head">
						<span>[ 流量消耗提示 / DATA NOTICE ]</span>
					</div>
					<p class="warn-lead">
						画廊包含多张高分辨率原图与现场合照，预估需要加载 &gt;30MB 流量。
					</p>
					<div class="warn-actions">
						<button class="metro-btn primary" @click="goLoadImages">继续加载 &rarr;</button>
						<button class="metro-btn" @click="abortToHome">返回主页</button>
					</div>
				</div>
			</div>
		</transition>

		<!-- Main Gallery Metro Wall -->
		<div v-if="!showWarn" class="gallery-main-container">
			<!-- Section Header -->
			<div class="section-header">
				<h1 class="section-title">画廊 <span class="sub">Gallery</span></h1>
				<span class="section-meta font-mono">共 {{ galleryItems.length }} 张照片</span>
			</div>

			<!-- Metro Bento Photo Grid -->
			<div class="metro-photo-grid">
				<div
					v-for="(item, idx) in galleryItems"
					:key="idx"
					class="photo-metro-tile"
					:class="idx === 0 || idx === 3 ? 'span-2' : ''"
					@click="openLightbox(idx)"
				>
					<div class="photo-img-wrapper">
						<img
							:src="item.thumbnailUrl || item.imageUrl"
							:alt="item.description"
							class="photo-img"
							loading="lazy"
						/>
						<div class="photo-overlay"></div>
					</div>

					<!-- Hover Metadata Pill -->
					<div class="photo-meta-bar">
						<div class="meta-date font-mono">{{ item.date }}</div>
						<div class="meta-desc">{{ item.description }}</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Lightbox Modal -->
		<transition name="modal-fade">
			<div v-if="lightboxIndex !== null" class="lightbox-backdrop" @click.self="closeLightbox">
				<div class="lightbox-window">
					<div class="lightbox-header font-mono">
						<span>[ {{ lightboxIndex + 1 }} / {{ galleryItems.length }} ]</span>
						<button class="close-lb-btn" @click="closeLightbox">[✕ 关闭]</button>
					</div>
					<div class="lightbox-body">
						<img
							:src="galleryItems[lightboxIndex].imageUrl"
							:alt="galleryItems[lightboxIndex].description"
							class="lb-img"
						/>
					</div>
					<div class="lightbox-footer font-mono">
						<button class="metro-btn" @click="prevPhoto">&lt;&lt; 上一张</button>
						<div class="lb-caption">
							<span class="lb-date">{{ galleryItems[lightboxIndex].date }}</span>
							<span class="lb-desc">{{ galleryItems[lightboxIndex].description }}</span>
						</div>
						<button class="metro-btn" @click="nextPhoto">下一张 &gt;&gt;</button>
					</div>
				</div>
			</div>
		</transition>

		<VisitCounter />
	</div>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import VisitCounter from "@/components/VisitCounter.vue";

const router = useRouter();
const showWarn = ref(true);
const lightboxIndex = ref(null);

const galleryItems = ref([
	{
		imageUrl: 'https://bucket.glowingstone.cn/instant_blue.jpg',
		thumbnailUrl: 'https://bucket.glowingstone.cn/instant_blue_thumb.jpg',
		description: 'Instant Blue背景图，因为这首歌喜欢上了秘封',
		date: '第138季 日与秋与木之年'
	},
	{
		imageUrl: 'https://bucket.glowingstone.cn/synchro0.jpg',
		thumbnailUrl: 'https://bucket.glowingstone.cn/synchro0.jpg',
		description: '圆梦Synchro0，感谢朋友投喂',
		date: '第140季 星与春与土之年 & 2025/1/11'
	},
	{
		imageUrl: 'https://bucket.glowingstone.cn/koishi_with_xnu.jpeg',
		thumbnailUrl: 'https://bucket.glowingstone.cn/instant_blue_thumb.jpg',
		description: '恋恋和XNU，摄于SUES长宁校区',
		date: '第139季 月与冬与金之年 & 2024/10/29'
	},
	{
		imageUrl: 'https://bucket.glowingstone.cn/hifuu_city2.jpg',
		thumbnailUrl: 'https://bucket.glowingstone.cn/hifuu_city2_thumb.jpg',
		description: '秘封...',
		date: '2024/10'
	},
	{
		imageUrl: 'https://bucket.glowingstone.cn/kotlin_shanghai_0703.jpg',
		thumbnailUrl: 'https://bucket.glowingstone.cn/kotlin_shanghai_0703_thumb.jpg',
		description: 'Kotlin上海用户组在24年7月的聚会，我与上海KUG的朋友们',
		date: '2024/07'
	},
	{
		imageUrl: 'https://bucket.glowingstone.cn/shanghai_srbs1.jpg',
		thumbnailUrl: 'https://bucket.glowingstone.cn/shanghai_srbs1_thumb.jpg',
		description: '森罗万象10/5上海 one man live，有幸到场',
		date: '2024/10'
	},
	{
		imageUrl: 'https://bucket.glowingstone.cn/shanghai_srbs2.png',
		thumbnailUrl: 'https://bucket.glowingstone.cn/shanghai_srbs2_thumb.png',
		description: '森罗万象10/5上海 one man live，合照',
		date: '2024/10'
	},
	{
		imageUrl: 'https://bucket.glowingstone.cn/kotlin_shanghai_0301.jpg',
		thumbnailUrl: 'https://bucket.glowingstone.cn/kotlin_shanghai_0301_thumb.jpg',
		description: 'KUG Shanghai X Jetbrains，3/1线下活动的合影',
		date: '2024/03'
	},
]);

const goLoadImages = () => {
	showWarn.value = false;
};

const abortToHome = () => {
	router.push('/');
};

const openLightbox = (index) => {
	lightboxIndex.value = index;
};

const closeLightbox = () => {
	lightboxIndex.value = null;
};

const prevPhoto = () => {
	if (lightboxIndex.value > 0) {
		lightboxIndex.value--;
	} else {
		lightboxIndex.value = galleryItems.value.length - 1;
	}
};

const nextPhoto = () => {
	if (lightboxIndex.value < galleryItems.value.length - 1) {
		lightboxIndex.value++;
	} else {
		lightboxIndex.value = 0;
	}
};
</script>

<style scoped>
.gallery-page {
	width: 100%;
	min-height: 100vh;
}

.gallery-main-container {
	max-width: var(--max-content-width);
	margin: 0 auto 120px;
	padding: 0 40px;
	box-sizing: border-box;
}

/* Warn Modal */
.cyber-warn-overlay {
	position: fixed;
	top: 0;
	left: 0;
	width: 100vw;
	height: 100vh;
	background: rgba(0, 0, 0, 0.95);
	z-index: 3000;
	display: flex;
	align-items: center;
	justify-content: center;
	padding: 32px;
	box-sizing: border-box;
}

.warn-bento-card {
	max-width: 580px;
	width: 100%;
	background: #121212;
	border: 1px solid #404040;
	padding: 48px;
	text-align: center;
}

.warn-head {
	color: #ffffff;
	font-size: 1.05rem;
	margin-bottom: 24px;
}

.warn-lead {
	color: #d4d4d4;
	font-size: 1.05rem;
	line-height: 1.8;
	margin: 0 0 32px;
}

.warn-actions {
	display: flex;
	gap: 18px;
	justify-content: center;
}

/* Metro Photo Grid */
.metro-photo-grid {
	display: grid;
	grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
	gap: 28px;
}

.photo-metro-tile {
	position: relative;
	border: 1px solid #262626;
	overflow: hidden;
	height: 340px;
	cursor: pointer;
	background: #121212;
	transition: border-color 0.15s ease;
}

.photo-metro-tile.span-2 {
	grid-column: span 2;
}

.photo-metro-tile:hover {
	border-color: #ffffff;
}

.photo-img-wrapper {
	position: absolute;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
}

.photo-img {
	width: 100%;
	height: 100%;
	object-fit: cover;
	display: block;
}

.photo-overlay {
	position: absolute;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
	background: linear-gradient(to top, rgba(0, 0, 0, 0.9) 0%, rgba(0, 0, 0, 0.2) 60%, transparent 100%);
}

.photo-meta-bar {
	position: absolute;
	bottom: 0;
	left: 0;
	width: 100%;
	padding: 24px;
	box-sizing: border-box;
	z-index: 2;
}

.meta-date {
	font-size: 0.82rem;
	color: #a3a3a3;
	margin-bottom: 8px;
}

.meta-desc {
	font-size: 1.05rem;
	color: #ffffff;
	line-height: 1.45;
	font-weight: 500;
}

/* Lightbox */
.lightbox-backdrop {
	position: fixed;
	top: 0;
	left: 0;
	width: 100vw;
	height: 100vh;
	background: rgba(0, 0, 0, 0.95);
	z-index: 4000;
	display: flex;
	align-items: center;
	justify-content: center;
	padding: 32px;
	box-sizing: border-box;
}

.lightbox-window {
	max-width: 1080px;
	width: 100%;
	max-height: 90vh;
	background: #121212;
	border: 1px solid #404040;
	overflow: hidden;
	display: flex;
	flex-direction: column;
}

.lightbox-header {
	display: flex;
	justify-content: space-between;
	align-items: center;
	padding: 16px 24px;
	background: #0a0a0a;
	border-bottom: 1px solid #262626;
	font-size: 0.88rem;
	color: #ffffff;
}

.close-lb-btn {
	background: transparent;
	border: 1px solid #404040;
	color: #ffffff;
	padding: 6px 12px;
	cursor: pointer;
	font-family: inherit;
}

.close-lb-btn:hover {
	background: #ffffff;
	color: #000000;
}

.lightbox-body {
	padding: 24px;
	display: flex;
	align-items: center;
	justify-content: center;
	background: #000000;
	overflow: hidden;
	max-height: 65vh;
}

.lb-img {
	max-width: 100%;
	max-height: 60vh;
	object-fit: contain;
}

.lightbox-footer {
	display: flex;
	justify-content: space-between;
	align-items: center;
	padding: 20px 28px;
	background: #0a0a0a;
	border-top: 1px solid #262626;
	font-size: 0.88rem;
}

.lb-caption {
	display: flex;
	flex-direction: column;
	align-items: center;
	text-align: center;
	padding: 0 20px;
}

.lb-date {
	font-size: 0.82rem;
	color: #a3a3a3;
}

.lb-desc {
	font-size: 1rem;
	color: #ffffff;
}

@media screen and (max-width: 768px) {
	.photo-metro-tile.span-2 {
		grid-column: span 1;
	}
	.gallery-main-container {
		padding: 0 20px;
	}
}
</style>
