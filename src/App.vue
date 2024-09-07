<template>
	<Nav/>
	<div class="main-frame">
		<router-view></router-view>
	</div>
	<Footer Footermsg="design & code by glowingstone124"/>
</template>
<script>
import Nav from './components/Nav.vue';
import Footer from './components/Footer.vue';

export default {
	name: 'App',
	components: {
		Nav,
		Footer
	},
}
</script>
<style scoped>
@import "@/css/main.css";

.main-frame {
	padding-top: 30px;
	max-width: 100vw;
}
</style>
