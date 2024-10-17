<template>
	<div class="contents">
		<h1>此页面仍然在开发/This page is still in dev</h1>
	</div>
	<VisitCounter/>
</template>

<script>
import VisitCounter from "@/components/VisitCounter.vue";

export default {
	name: 'NavBtn' ,
	components: {VisitCounter}
}
</script>

<style scoped>
@import "@/css/main.css";

.contents {
	padding-top: 20vh;
}
</style>
