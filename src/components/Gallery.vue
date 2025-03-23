<script setup>
import { ref, onMounted } from 'vue';
import VisitCounter from "@/components/VisitCounter.vue";
const showWarn = ref(true)
const galleryItems = ref([
	{
		imageUrl: 'https://storage.glowingstone.cn/download/instant_blue.jpg',
		thumbnailUrl: 'https://storage.glowingstone.cn/download/instant_blue_thumb.jpg',
		description: 'Instant Blue背景图，因为这首歌喜欢上了秘封',
		date: '第138季 日与秋与木之年'
	},
	{
		imageUrl: 'https://storage.glowingstone.cn/download/synchro0.jpg',
		thumbnailUrl: 'https://storage.glowingstone.cn/download/synchro0.jpg',
		description: '圆梦Synchro0，感谢朋友投喂',
		date: '第140季 星与春与土之年 & 2025/1/11'
	},
	{
		imageUrl: 'https://storage.glowingstone.cn/download/koishi_with_xnu.jpeg',
		thumbnailUrl: 'https://storage.glowingstone.cn/download/instant_blue_thumb.jpg',
		description: '恋恋和XNU，摄于SUES长宁校区',
		date: '第139季 月与冬与金之年 & 2024/10/29'
	},
	{
		imageUrl: 'https://storage.glowingstone.cn/download/hifuu_city2.jpg',
		thumbnailUrl: 'https://storage.glowingstone.cn/download/hifuu_city2_thumb.jpg',
		description: '秘封...',
		date: '2024/10'
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

const onImageLoad = (index) => {
	loadingStates.value[index] = false;
};

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

const redirect = () => {
	window.location.href = "/"
}
onMounted(() => {
	//lazyLoadImages();
});

const goLoadImages = () => {
	showWarn.value = false;
	setInterval(() => {
		lazyLoadImages();
	}, 300)
}
</script>

<template>
	<div class="warn" v-if="showWarn">
		<h1>该页面含有大量图片。</h1>
		<p>预估需要消耗>30MB流量，如果您不希望加载这些内容，请点击取消。</p>
		<button class="btn confirm" @click="goLoadImages">继续</button>
		<button class="btn cancel" @click="redirect">取消</button>
	</div>
	<div class="gallery-container" v-if="!showWarn">
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
	<VisitCounter/>
</template>


<style scoped>

.btn {
	margin: 0.8rem;
	border-radius: 0.8rem;
	padding:20px 40px;
}
.confirm{
	background-color: #3f7ce8;
	border: none;
	color: white;
}
.cancel {
	border: 1px solid white;
	background-color: transparent;
	color: white;
}
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

.warn {
	position: fixed;
	top:0;
	left: 0;
	width: 100vw;
	height: 100vh;
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
	text-align: center;
	background: rgba(26, 26, 26, 0.4);
	color: white;
	z-index: 9999;
	backdrop-filter: blur(20px);
	p{
		margin: 0;
	}
}
</style>
