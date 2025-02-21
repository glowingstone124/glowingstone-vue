<script setup>
import { onMounted, ref } from "vue";

let visitCount = ref(0);

async function incrementVisitCount(){
	try {
		const response = await fetch("https://misc.glowingstone.cn/increment", {
		});

		if (!response.ok) {
			throw new Error(`Failed to fetch visit count: ${response.statusText}`);
		}

		const data = await response.json();
		const resultData = JSON.parse(data.result);
		visitCount.value = resultData.value;

		console.log("Visit count:", visitCount.value);
		visitCount.value++;

		console.log("Updated visit count:", visitCount.value);
	} catch (error) {
		console.error("Error:", error);
	}
}

async function fetchVisitCount(){
	try {
		const response = await fetch("https://misc.glowingstone.cn/count", {})
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
	<p class="counter">已经有{{ visitCount }}人访问了本站</p>
</template>

<style scoped>
.counter {
	text-align: center;
}
</style>
