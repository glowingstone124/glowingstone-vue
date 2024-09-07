<template>
	<div class="navigator">
		<ul id="navigation" class="navul">
			<div class="nav-title">
				<img src="https://avatars.githubusercontent.com/u/163746181" alt="Logo" class="nav-logo"/>
				<h1>Glowstone 萤石</h1>
			</div>

			<li class="navli"><a href="/"><img src="@/assets/home.svg"><span>主页</span></a></li>
			<li class="navli"><a href="/friends"><img src="@/assets/friends.svg"><span>友链</span></a></li>
			<li class="navli"><a href="/about"><img src="@/assets/about.svg"><span>关于</span></a></li>
		</ul>
	</div>
</template>

<script>
export default {
	name: 'Nav'
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
@media (max-width: 768px) {
	.nav-title {
		flex-direction: column;
		text-align: center;
	}

	.nav-title h1 {
		display: none;
		margin-left: 0;
		margin-top: 10px;
	}

	.nav-logo {
		display: none;
		width: 50px;
	}

	.navul {
		padding: 0;
	}

	.navli {
		display: block;
		text-align: center;
		padding: 10px 0;
		transition: background-color 0.1s linear;
	}

	.navli a {
		flex-direction: column;
	}

	.navli img {
		width: 30px;
		height: auto;
	}

	.navli span {
		margin-top: 5px;
		font-size: 14px;
	}
}

@media (max-width: 480px) {
	.nav-title h1 {
		font-size: 16px;
	}

	.nav-logo {
		width: 40px;
	}

	.navli span {
		font-size: 12px;
	}
}
</style>
