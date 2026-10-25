<template>
	<div class="gallery-page">
		<!-- Bandwidth Warning Modal -->
		<transition name="apple-fade">
			<div v-if="showWarn" class="warn-overlay">
				<div class="apple-dialog-card">
					<div class="dialog-icon-box">
						<svg width="36" height="36" viewBox="0 0 24 24" fill="none" stroke="#0071e3" stroke-width="2">
							<rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
							<circle cx="8.5" cy="8.5" r="1.5"></circle>
							<polyline points="21 15 16 10 5 21"></polyline>
						</svg>
					</div>
					<h3 class="dialog-title">画廊原图流量提示</h3>
					<p class="dialog-desc">
						画廊包含多张高分辨率原图与现场摄影，预估需要消耗约 30MB 流量。请确认是否继续加载？
					</p>
					<div class="dialog-actions">
						<button class="apple-btn-pill primary" @click="goLoadImages">继续浏览画廊</button>
						<button class="apple-btn-pill" @click="abortToHome">返回首页</button>
					</div>
				</div>
			</div>
		</transition>

		<!-- Main Gallery Container -->
		<div v-if="!showWarn" class="gallery-main-container">
			<!-- Section Header -->
			<div class="page-header">
				<span class="apple-badge">PHOTO GALLERY &amp; MOMENTS</span>
				<h1 class="page-title">画廊与摄影特辑</h1>
				<p class="page-desc">定格生活光影、技术峰会与漫游足迹（共 {{ galleryItems.length }} 帧摄影作品）。</p>
			</div>

			<!-- Apple Photo Bento Grid -->
			<div class="apple-photo-grid">
				<div
					v-for="(item, idx) in galleryItems"
					:key="idx"
					class="apple-photo-card"
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
						<div class="photo-gradient-overlay"></div>
					</div>

					<div class="photo-info-bar">
						<span class="photo-pill-date">{{ item.date }}</span>
						<p class="photo-caption">{{ item.description }}</p>
					</div>
				</div>
			</div>
		</div>

		<!-- Apple Lightbox Modal -->
		<transition name="apple-fade">
			<div v-if="lightboxIndex !== null" class="lightbox-backdrop" @click.self="closeLightbox">
				<div class="lightbox-window">
					<button class="lightbox-close-btn" @click="closeLightbox">✕</button>

					<div class="lightbox-image-box">
						<img
							:src="galleryItems[lightboxIndex].imageUrl"
							:alt="galleryItems[lightboxIndex].description"
							class="lightbox-img"
						/>
					</div>

					<div class="lightbox-info-row">
						<div>
							<span class="lb-date">{{ galleryItems[lightboxIndex].date }}</span>
							<p class="lb-desc">{{ galleryItems[lightboxIndex].description }}</p>
						</div>
						<div class="lb-counter">
							{{ lightboxIndex + 1 }} / {{ galleryItems.length }}
						</div>
					</div>
				</div>
			</div>
		</transition>

		<VisitCounter />
	</div>
</template>

<script>
import VisitCounter from "@/components/VisitCounter.vue";

export default {
	name: 'Gallery',
	components: { VisitCounter },
	data() {
		return {
			showWarn: true,
			lightboxIndex: null,
			galleryItems: [
				{
					imageUrl: "https://bucket.glowingstone.cn/xianxia.png",
					thumbnailUrl: "https://bucket.glowingstone.cn/xianxia.png",
					date: "2024-05",
					description: "长宁旧街道，傍晚散步时的街景与路灯。",
				},
				{
					imageUrl: "/komeji2.jpg",
					thumbnailUrl: "/komeji2.jpg",
					date: "2024-09",
					description: "古明地恋插画写真特辑。",
				},
				{
					imageUrl: "/default.jpg",
					thumbnailUrl: "/default.jpg",
					date: "2024-08",
					description: "博客早期默认封面摄影。",
				},
				{
					imageUrl: "/talk-about-coroutines.md.jpg",
					thumbnailUrl: "/talk-about-coroutines.md.jpg",
					date: "2024-09",
					description: "Kotlin 协程架构专题特写。",
				},
			],
		};
	},
	methods: {
		goLoadImages() {
			this.showWarn = false;
		},
		abortToHome() {
			this.$router.push('/');
		},
		openLightbox(index) {
			this.lightboxIndex = index;
		},
		closeLightbox() {
			this.lightboxIndex = null;
		},
	},
};
</script>

<style scoped>
.gallery-page {
	width: 100%;
	padding: 40px 0 80px;
}

.gallery-main-container {
	max-width: var(--apple-max-width, 1240px);
	margin: 0 auto;
	padding: 0 24px;
	box-sizing: border-box;
}

.page-header {
	margin-bottom: 36px;
}

.page-title {
	font-size: 38px;
	font-weight: 700;
	color: #1d1d1f;
	margin: 8px 0 12px;
	letter-spacing: -0.02em;
}

.page-desc {
	font-size: 16px;
	color: #86868b;
	margin: 0;
}

/* Apple Photo Grid */
.apple-photo-grid {
	display: grid;
	grid-template-columns: repeat(3, 1fr);
	gap: 24px;
}

.apple-photo-card {
	position: relative;
	height: 380px;
	border-radius: var(--apple-radius-card, 24px);
	overflow: hidden;
	cursor: pointer;
	box-shadow: 0 4px 20px rgba(0, 0, 0, 0.05);
	transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1), box-shadow 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

.apple-photo-card.span-2 {
	grid-column: span 2;
}

.apple-photo-card:hover {
	transform: translateY(-6px);
	box-shadow: 0 16px 36px rgba(0, 0, 0, 0.12);
}

.photo-img-wrapper {
	width: 100%;
	height: 100%;
	position: relative;
}

.photo-img {
	width: 100%;
	height: 100%;
	object-fit: cover;
	display: block;
	transition: transform 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}

.apple-photo-card:hover .photo-img {
	transform: scale(1.04);
}

.photo-gradient-overlay {
	position: absolute;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
	background: linear-gradient(to top, rgba(0, 0, 0, 0.8) 0%, rgba(0, 0, 0, 0.2) 50%, transparent 100%);
	pointer-events: none;
}

.photo-info-bar {
	position: absolute;
	bottom: 0;
	left: 0;
	width: 100%;
	padding: 24px;
	box-sizing: border-box;
	z-index: 2;
}

.photo-pill-date {
	display: inline-block;
	padding: 3px 10px;
	border-radius: 980px;
	font-size: 11px;
	font-weight: 700;
	background: rgba(255, 255, 255, 0.25);
	backdrop-filter: blur(8px);
	color: #ffffff;
	margin-bottom: 8px;
}

.photo-caption {
	color: #ffffff;
	font-size: 16px;
	font-weight: 600;
	margin: 0;
	line-height: 1.4;
	text-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
}

/* Warn Modal */
.warn-overlay {
	position: fixed;
	top: 0;
	left: 0;
	width: 100vw;
	height: 100vh;
	background: rgba(0, 0, 0, 0.4);
	backdrop-filter: blur(12px);
	display: flex;
	align-items: center;
	justify-content: center;
	z-index: 2000;
	padding: 24px;
	box-sizing: border-box;
}

.apple-dialog-card {
	background: #ffffff;
	border-radius: var(--apple-radius-card, 24px);
	padding: 40px;
	max-width: 460px;
	width: 100%;
	text-align: center;
	box-shadow: 0 20px 50px rgba(0, 0, 0, 0.15);
}

.dialog-icon-box {
	margin-bottom: 20px;
}

.dialog-title {
	font-size: 22px;
	font-weight: 700;
	color: #1d1d1f;
	margin: 0 0 12px;
}

.dialog-desc {
	font-size: 15px;
	line-height: 1.6;
	color: #6e6e73;
	margin: 0 0 28px;
}

.dialog-actions {
	display: flex;
	justify-content: center;
	gap: 16px;
}

/* Lightbox Modal */
.lightbox-backdrop {
	position: fixed;
	top: 0;
	left: 0;
	width: 100vw;
	height: 100vh;
	background: rgba(0, 0, 0, 0.85);
	backdrop-filter: blur(16px);
	display: flex;
	align-items: center;
	justify-content: center;
	z-index: 3000;
	padding: 32px;
	box-sizing: border-box;
}

.lightbox-window {
	position: relative;
	max-width: 90vw;
	max-height: 90vh;
	display: flex;
	flex-direction: column;
	align-items: center;
}

.lightbox-close-btn {
	position: absolute;
	top: -44px;
	right: 0;
	background: rgba(255, 255, 255, 0.2);
	border: none;
	color: #ffffff;
	font-size: 18px;
	width: 36px;
	height: 36px;
	border-radius: 50%;
	cursor: pointer;
	display: flex;
	align-items: center;
	justify-content: center;
	backdrop-filter: blur(6px);
}

.lightbox-image-box {
	max-height: 75vh;
	overflow: hidden;
	border-radius: 18px;
}

.lightbox-img {
	max-width: 100%;
	max-height: 75vh;
	object-fit: contain;
	display: block;
}

.lightbox-info-row {
	width: 100%;
	display: flex;
	justify-content: space-between;
	align-items: center;
	color: #ffffff;
	margin-top: 16px;
}

.lb-date {
	font-size: 12px;
	color: rgba(255, 255, 255, 0.6);
}

.lb-desc {
	font-size: 15px;
	margin: 4px 0 0;
}

.lb-counter {
	font-size: 13px;
	color: rgba(255, 255, 255, 0.6);
}

@media screen and (max-width: 860px) {
	.apple-photo-grid {
		grid-template-columns: 1fr;
	}
	.apple-photo-card.span-2 {
		grid-column: span 1;
	}
}
</style>
