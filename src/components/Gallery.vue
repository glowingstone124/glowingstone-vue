<script setup>
import { ref } from 'vue';

// 定义图片和介绍内容
const galleryItems = ref([
	{ imageUrl: 'https://storage.glowingstone.cn/download/instant_blue.jpg', description: 'Instant Blue背景图，因为这首歌喜欢上了秘封' },
	{ imageUrl: 'https://storage.glowingstone.cn/download/kotlin_shanghai_0703.jpg', description: 'Kotlin上海用户组在24年7月的聚会，我与上海KUG的朋友们' },
	{ imageUrl: 'https://storage.glowingstone.cn/download/shanghai_srbs1.jpg', description: '森罗万象10/5上海 one man live，有幸到场' },
	{ imageUrl: 'https://storage.glowingstone.cn/download/shanghai_srbs2.png', description: '森罗万象10/5上海 one man live，合照' },

]);
</script>

<template>
	<div class="gallery-container">
		<!-- 滑动画廊区域 -->
		<div class="gallery">
			<div v-for="(item, index) in galleryItems" :key="index" class="gallery-item">
				<div class="image-section">
					<img :src="item.imageUrl" alt="Gallery Image" />
				</div>
				<div class="description-section">
					<p>{{ item.description }}</p>
				</div>
			</div>
		</div>
	</div>
</template>

<style scoped>
/* 容器设置 */
.gallery-container {
	overflow-y: auto;
	display: flex;
	justify-content: center;
	align-items: center;
}

.gallery {
	margin: 50px 40px;
	display: flex;
	flex-direction: column;
	width: 100%;
}

.gallery-item {
	display: flex;
	flex-direction: row;
	width: 100%;
	height: auto;
	box-sizing: border-box;
	margin-bottom: 20px;
}

.image-section {
	width: 80vw;
	height: auto;
}

.image-section img {
	width: 80vw;
	height: auto;
	object-fit: contain;
}

/* 介绍区域 */
.description-section {
	padding: 20px;
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
}
</style>
