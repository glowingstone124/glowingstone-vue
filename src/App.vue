<template>
	<Nav/>
	<main class="main-frame">
		<router-view v-slot="{ Component }">
			<transition name="metro-fade" mode="out-in">
				<component :is="Component" />
			</transition>
		</router-view>
	</main>
	<Footer Footermsg="design & code by glowingstone124 // METRO_ASCII_B&W"/>
</template>

<script>
import Nav from './components/Nav.vue';
import Footer from './components/Footer.vue';

export default {
	name: 'App',
	components: {
		Nav,
		Footer,
	},
};
</script>

<style>
@import "@/css/main.css";
@import "@/css/metro-ascii.css";

.main-frame {
	position: relative;
	min-height: calc(100vh - 140px);
	padding-top: 130px;
	padding-bottom: 140px;
	width: 100%;
	box-sizing: border-box;
}

/* Page Transition Animations */
.metro-fade-enter-active,
.metro-fade-leave-active {
	transition: opacity 0.18s ease;
}

.metro-fade-enter-from,
.metro-fade-leave-to {
	opacity: 0;
}
</style>
