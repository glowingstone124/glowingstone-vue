<script setup>
import ArticleList from '@/components/Mobile/ArticleList.vue';
</script>

<template>
	<div class="mobile-page font-mono">
		<div class="mobile-header">
			<h1>Glowstone 萤石</h1>
			<p class="mobile-sub">移动端快速访问视图</p>
		</div>
		<ArticleList />
	</div>
</template>

<style scoped>
.mobile-page {
	max-width: 600px;
	margin: 0 auto;
	padding: 20px;
	box-sizing: border-box;
}

.mobile-header {
	border-bottom: 1px solid #262626;
	padding-bottom: 16px;
	margin-bottom: 20px;
}

.mobile-header h1 {
	font-size: 1.6rem;
	color: #ffffff;
	margin: 0 0 6px;
}

.mobile-sub {
	color: #a3a3a3;
	font-size: 0.85rem;
	margin: 0;
}
</style>
