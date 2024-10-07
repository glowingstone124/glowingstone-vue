<script setup>
import { ref } from 'vue';

const galleryItems = ref([
	{ imageUrl: 'https://storage.glowingstone.cn/download/instant_blue.jpg', description: 'Instant Blue背景图，因为这首歌喜欢上了秘封', date: '第138季 日与秋与木之年' },
	{ imageUrl: 'https://storage.glowingstone.cn/download/kotlin_shanghai_0703.jpg', description: 'Kotlin上海用户组在24年7月的聚会，我与上海KUG的朋友们', date: '2024/07' },
	{ imageUrl: 'https://storage.glowingstone.cn/download/shanghai_srbs1.jpg', description: '森罗万象10/5上海 one man live，有幸到场',date: '2024/10' },
	{ imageUrl: 'https://storage.glowingstone.cn/download/shanghai_srbs2.png', description: '森罗万象10/5上海 one man live，合照', date: '2024/10' },
	{ imageUrl: 'https://storage.glowingstone.cn/download/kotlin_shanghai_0301.jpg', description: 'KUG Shanghai X Jetbrains，3/1线下活动的合影',date: '2024/03' },
])
</script>

<template>
	<div class="gallery-container">
		<div class="gallery">
			<div v-for="(item, index) in galleryItems" :key="index" class="gallery-item">
				<div class="description-section">
					<p class="smaller">{{ item.date }}</p>
					<p>{{ item.description }}</p>
				</div>
				<div class="image-section">
					<img :src="item.imageUrl" alt="Gallery Image" />
				</div>
			</div>
		</div>
	</div>
</template>

<style scoped>
.gallery-container {
	overflow-y: auto;
	display: flex;
	justify-content: center;
	align-items: center;
}

.gallery {
	margin: 70px 40px;
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
	margin-bottom: 50px;
}

.image-section {
	width: 60vw;
	height: auto;
}

.image-section img {
	border-radius: 30px;
	min-width: 60vw;
	max-width: 60vw;
	margin: 0 auto;
	height: auto;
	object-fit: contain;
}

.description-section {
	padding: 20px;
	display: flex;
	flex-direction: column;
	border-radius: 40px;
	margin-right: 15px;
	background-color: #4c618f;
	justify-content: center;
	align-items: baseline;
	font-size: 1.5rem;
	width: 30vw;
}

.description-section p {
	font-weight: 200;
	margin: 0;
}

.smaller {
	font-size: 1rem;
	margin-top: 0.2rem;
}
</style>

