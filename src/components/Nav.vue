<template>
	<div v-if="isDesktop" class="navigator">
		<ul id="navigation" class="navul">
			<div class="nav-title">
				<img src="https://avatars.githubusercontent.com/u/163746181" alt="Logo" class="nav-logo"/>
				<h1>Glowstone 萤石</h1>
			</div>

			<li class="navli" :class="{ active: $route.path === '/' }"><router-link to="/"><img src="@/assets/home.svg"><span>主页</span></router-link></li>
			<li class="navli" :class="{ active: $route.path === '/friends' }"><router-link to="/friends"><img src="@/assets/friends.svg"><span>友链</span></router-link></li>
			<li class="navli" :class="{ active: $route.path === '/about' }"><router-link to="/about"><img src="@/assets/about.svg"><span>关于</span></router-link></li>
			<li class="navli" :class="{ active: $route.path === '/archive' }"><router-link to="/archive"><img src="@/assets/archive.svg"><span>存档</span></router-link></li>
			<li class="navli" :class="{ active: $route.path === '/gallery' }"><router-link to="/gallery"><img src="@/assets/photo.svg"><span>画廊</span></router-link></li>
			<li class="navli" :class="{ active: $route.path === '/announcement' }"><router-link to="/announcement"><img src="@/assets/special.svg"><span>特别声明</span></router-link></li>
		</ul>
	</div>
</template>

<script>
export default {
	name: 'Nav',
	data() {
		return {
			isDesktop: window.innerWidth >= 800,
		};
	},
	mounted() {
		window.addEventListener('resize', this.checkWidth);
	},
	beforeUnmount() {
		window.removeEventListener('resize', this.checkWidth);
	},
	methods: {
		checkWidth() {
			this.isDesktop = window.innerWidth >= 800;
		},
	},
}
</script>

<style scoped>
@import "@/css/main.css";

li a {
	display: flex;
	flex-direction: row;
	column-gap: 0.5em;
}

.nav-title {
	display: flex;
	align-items: center;
	padding: 10px 20px;
	color: #d8d6d6;
}
.navigator {
	max-height: 5vh;
}
.nav-title h1 {
	font-size: 20px;
	font-weight: 100;
	margin-left: 10px;
}

.nav-logo {
	border-radius: 20px;
	width: 60px;
	height: auto;
	margin-right: 10px;
}
</style>
