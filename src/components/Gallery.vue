<script setup>
import { ref, onMounted } from 'vue';

const galleryItems = ref([
	{
		imageUrl: 'https://storage.glowingstone.cn/download/instant_blue.jpg',
		thumbnailUrl: 'https://storage.glowingstone.cn/download/instant_blue_thumb.jpg',
		description: 'Instant Blue背景图，因为这首歌喜欢上了秘封',
		date: '第138季 日与秋与木之年'
	},
	{
		imageUrl: 'https://storage.glowingstone.cn/download/kotlin_shanghai_0703.jpg',
		thumbnailUrl: 'https://storage.glowingstone.cn/download/kotlin_shanghai_0703_thumb.jpg',
		description: 'Kotlin上海用户组在24年7月的聚会，我与上海KUG的朋友们',
		date: '2024/07'
	},
	{
		imageUrl: 'https://storage.glowingstone.cn/download/shanghai_srbs1.jpg',
		thumbnailUrl: 'https://storage.glowingstone.cn/download/shanghai_srbs1_thumb.jpg',
		description: '森罗万象10/5上海 one man live，有幸到场',
		date: '2024/10'
	},
	{
		imageUrl: 'https://storage.glowingstone.cn/download/shanghai_srbs2.png',
		thumbnailUrl: 'https://storage.glowingstone.cn/download/shanghai_srbs2_thumb.png',
		description: '森罗万象10/5上海 one man live，合照',
		date: '2024/10'
	},
	{
		imageUrl: 'https://storage.glowingstone.cn/download/kotlin_shanghai_0301.jpg',
		thumbnailUrl: 'https://storage.glowingstone.cn/download/kotlin_shanghai_0301_thumb.jpg',
		description: 'KUG Shanghai X Jetbrains，3/1线下活动的合影',
		date: '2024/03'
	},
]);

const imagesRefs = ref([]);
const loadingStates = ref(galleryItems.value.map(() => true));

// 图片加载完成时，更新加载状态
const onImageLoad = (index) => {
	loadingStates.value[index] = false;
};

// 懒加载逻辑
const lazyLoadImages = () => {
	const observer = new IntersectionObserver((entries) => {
		entries.forEach(entry => {
			if (entry.isIntersecting) {
				const img = entry.target;
				img.src = img.dataset.src;
				observer.unobserve(img);
			}
		});
	});

	imagesRefs.value.forEach(img => {
		if (img) {
			observer.observe(img);
		}
	});
};

onMounted(() => {
	lazyLoadImages();
});
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
					<div v-if="loadingStates[index]" class="loading-animation">
						<span>Loading...</span>
					</div>
					<img
						:data-src="item.imageUrl"
						:src="item.thumbnailUrl"
						:ref="el => imagesRefs[index] = el"
						:class="{ lazyload: true, hidden: loadingStates[index] }"
						alt="Gallery Image"
						@load="onImageLoad(index)"
					/>
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
	position: relative;
}

.image-section img {
	border-radius: 30px;
	min-width: 60vw;
	max-width: 60vw;
	margin: 0 auto;
	height: auto;
	object-fit: contain;
	transition: opacity 0.3s ease;
}

.image-section img.hidden {
	opacity: 0;
}

.loading-animation {
	position: absolute;
	top: 50%;
	left: 50%;
	transform: translate(-50%, -50%);
	color: #fff;
	font-size: 2rem;
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
	font-size: 1.8rem;
	width: 30vw;
}

.description-section p {
	font-weight: 200;
	margin: 0;
}

.smaller {
	font-size: 1.2rem;
	margin-top: 0.2rem;
}
</style>
