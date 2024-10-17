<script setup>
import { onMounted, ref } from "vue";

let visitCount = ref(0);

async function incrementVisitCount(){
	try {
		const response = await fetch("https://lenient-dove-46659.upstash.io/get/visitcount", {
			headers: {
				Authorization: "Bearer AbZDASQgYTZmZGM5MTctZjYyNi00YzdjLWE3ZWUtZWQyZjVhNzBhMzA0ZmNjZjMzZDRkMWFiNGYwYmIwMDQ2Yjg1NWNhMmJmYzU=" // 请替换为你的实际授权令牌
			}
		});

		if (!response.ok) {
			throw new Error(`Failed to fetch visit count: ${response.statusText}`);
		}

		const data = await response.json();
		const resultData = JSON.parse(data.result);
		visitCount.value = resultData.value; // 更新ref的值

		console.log("Visit count:", visitCount.value);
		visitCount.value++;

		const updateResponse = await fetch("https://lenient-dove-46659.upstash.io/set/visitcount", {
			method: "POST",
			headers: {
				Authorization: "Bearer AbZDASQgYTZmZGM5MTctZjYyNi00YzdjLWE3ZWUtZWQyZjVhNzBhMzA0ZmNjZjMzZDRkMWFiNGYwYmIwMDQ2Yjg1NWNhMmJmYzU=", // 请替换为你的实际授权令牌
				"Content-Type": "application/json"
			},
			body: JSON.stringify({
				value: visitCount.value
			})
		});

		if (!updateResponse.ok) {
			throw new Error(`Failed to update visit count: ${updateResponse.statusText}`);
		}

		console.log("Updated visit count:", visitCount.value);
	} catch (error) {
		console.error("Error:", error);
	}
}

onMounted(() => {
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
