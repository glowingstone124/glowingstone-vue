<script setup>
import { onMounted, onBeforeUnmount } from 'vue';
import { useRouter } from 'vue-router';
import ArticleList from '@/components/Mobile/ArticleList.vue'
const router = useRouter();

const checkWidth = () => {
	if (window.innerWidth > 800) {
		router.push('/');
	}
};

onMounted(() => {
	window.addEventListener('resize', checkWidth);
	checkWidth();
});

onBeforeUnmount(() => {
	window.removeEventListener('resize', checkWidth);
});
</script>

<template>
	<div class="mobile">
	<h1>Glowingstone's Blog</h1>
	<h3>此页面专为移动端用户打造，因此它的维护不在主要计划中，请使用宽度大于800px的设备来访问桌面端页面。</h3>
	<ArticleList />
	</div>
</template>

<style scoped>
.mobile {
	margin: 20px
}
</style>
