<template>
	<div class="apple-newsroom-app">
		<Nav/>
		<main class="main-frame">
			<router-view v-slot="{ Component }">
				<transition name="apple-fade" mode="out-in">
					<component :is="Component" />
				</transition>
			</router-view>
		</main>
		<Footer Footermsg="Apple Newsroom Style · Glowingstone Blog"/>
	</div>
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
@import "@/css/apple-newsroom.css";

.apple-newsroom-app {
	min-height: 100vh;
	display: flex;
	flex-direction: column;
	background-color: var(--apple-bg);
}

.main-frame {
	position: relative;
	flex: 1;
	width: 100%;
	padding-top: 108px; /* space for global nav (52px) + newsroom subnav (56px) */
	padding-bottom: 96px;
	box-sizing: border-box;
}

/* Apple Smooth Page Transition Animations */
.apple-fade-enter-active,
.apple-fade-leave-active {
	transition: opacity 0.22s ease, transform 0.22s ease;
}

.apple-fade-enter-from {
	opacity: 0;
	transform: translateY(6px);
}

.apple-fade-leave-to {
	opacity: 0;
	transform: translateY(-6px);
}
</style>
