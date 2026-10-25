<script setup>
import { onMounted, ref } from "vue";

let visitCount = ref(0);

async function incrementVisitCount(){
	try {
		const response = await fetch("https://blog-visit-counter.hanserofficial.workers.dev/increment", {});
		if (!response.ok) {
			throw new Error(`Failed to fetch visit count: ${response.statusText}`);
		}
	} catch (error) {
		console.error("Error:", error);
	}
}

async function fetchVisitCount(){
	try {
		const response = await fetch("https://blog-visit-counter.hanserofficial.workers.dev/count", {});
		if (!response.ok) {
			throw new Error(`Failed to fetch visit count: ${response.statusText}`);
		}
		const data = await response.json();
		visitCount.value = data.visit_count;
	} catch (error) {
		console.error("Error:", error);
	}
}

onMounted(() => {
	fetchVisitCount();
	incrementVisitCount();
});
</script>

<template>
	<div class="counter-wrap">
		<span class="counter-pill">
			<span class="live-dot"></span>
			累计浏览人次: {{ visitCount || '...' }}
		</span>
	</div>
</template>

<style scoped>
.counter-wrap {
	text-align: center;
	padding: 32px 0 16px;
}

.counter-pill {
	display: inline-flex;
	align-items: center;
	gap: 8px;
	font-size: 12px;
	color: #86868b;
	background: #f5f5f7;
	padding: 4px 14px;
	border-radius: 980px;
}

.live-dot {
	width: 6px;
	height: 6px;
	border-radius: 50%;
	background: #34c759;
}
</style>
