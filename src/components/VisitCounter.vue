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
	<div class="counter-wrap font-mono">
		<span class="counter-text">[ 访问人次 / VISITS: {{ visitCount }} ]</span>
	</div>
</template>

<style scoped>
.counter-wrap {
	text-align: center;
	padding: 24px 0 10px;
}

.counter-text {
	font-size: 0.8rem;
	color: #737373;
	letter-spacing: 0.5px;
}
</style>
