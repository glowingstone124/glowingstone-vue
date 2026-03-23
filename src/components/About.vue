<template>
	<div class="contents">
		<img src="@root/src/assets/me.svg" class="title-signature">
		<div class="friends">
			<div class="info-box">
				<h1>我是谁？</h1>
				<p>我是glowingstone，现居上海。我主要专注于JVM和Linux/POSIX兼容系统的程序设计和架构，也正在学习吉他。</p>
			</div>

			<div class="info-box">
				<h1>关于这个网站</h1>
				<a href="https://www.pixiv.net/artworks/92320484">背景来自Pixiv</a>
				<p>这个网站是完全自建的，使用了Vue3。</p>
				<p>前端：Vue3</p>
				<img @click="redirectToGithub" src="https://socialify.git.ci/glowingstone124/glowingstone-vue/image?description=1&descriptionEditable=%E5%9C%A8github%E4%B8%8A%E6%89%BE%E5%88%B0%E6%88%91&font=Inter&language=1&name=1&owner=1&pattern=Plus&stargazers=1&theme=Dark" alt="">
				<p>后端目前采用Cloudflare Pages。</p>
			</div>

			<div class="info-box">
				<h1>社媒账号</h1>
				<p>这里有我的一些公开社媒账号，欢迎添加好友友好互动！</p>
				<a href="https://steamcommunity.com/profiles/76561199236874427/">Steam</a>
				<a href="https://x.com/QuantumOriginal">X</a>
			</div>

			<div class="info-box">
				<h1>听歌</h1>
				<p>前卫摇滚，老摇滚，最爱70年代80年代的欧美乐队，最喜欢PinkFloyd和Led Zeppelin。<br>
					中国摇滚也听一些，喜欢Blues味重的乐队比如万青。<br>
					自己也入坑了吉他，买了把Fender练点Vintage玩玩而已
				</p>
			</div>

			<div class="info-box">
				<h1>联系我</h1>
				<a href="mailto:hanseroffcial@outlook.com"><img src="chat.svg" alt="">Mail</a>
			</div>
		</div>
	</div>
	<VisitCounter/>
</template>

<script>
import VisitCounter from "@/components/VisitCounter.vue";

export default {
	name: 'About',
	components: {VisitCounter},
	methods: {
		redirectToGithub() {
			window.location.href = 'https://github.com/glowingstone124';
		},
		setRandomBackground() {
			const images = [
				'https://bucket.glowingstone.cn/komeji2.jpg',
				'https://bucket.glowingstone.cn/koishi.jpg'
			];
			const randomImage = images[Math.floor(Math.random() * images.length)];
			document.body.style.backgroundImage = `url("${randomImage}")`;
		}
	},
	mounted() {
		this.setRandomBackground();
		document.body.style.backgroundSize = 'cover';
		document.body.style.backgroundRepeat = 'no-repeat';
		document.body.style.backgroundAttachment = 'fixed';
	},
	beforeUnmount() {
		document.body.style.backgroundImage = '';
	}
}
</script>

<style scoped>
@import "@/css/main.css";

.contents {
	background-size: cover;
	background-position: center;
	margin-top: 8vh;
	margin-left: 10vw;
	margin-right: 10vw;
}

.category {
	font-size: 5em;
	font-weight: 500;
}

img {
	margin: auto;
}

.friends {
	display: flex;
	flex-direction: row;
	flex-wrap: wrap;
	justify-content: space-between;
	margin-top: 3vh;
}

.info-box {
	background-color: rgba(168, 179, 196, 0.31);
	backdrop-filter: blur(20px);
	border-radius: 10px;
	padding: 45px;
	margin: 20px;
	flex: 1;
	max-width: 50%;
	min-width: 30%;
	transition: all 0.2s ease-in-out;
}

.info-box:hover {
	transform: translateY(-10px);
	box-shadow: rgba(50, 50, 93, 0.3) 0px 20px 40px -5px,
	rgba(0, 0, 0, 0.3) 0px 10px 24px -8px;
}

p {
	padding: 5px;
}

a {
	background-color: rgb(136, 160, 189);
	border-radius: 50px;
	padding: 25px;
	margin: auto auto 20px;
	text-decoration: none;
	transition: background-color 0.3s;
	display: flex;
	justify-content: center;
	align-items: center;
	max-height: 30px;
}

a:hover {
	background-color: rgb(107, 126, 150);
}

img {
	max-width: 100%;
	cursor: pointer;
	margin: 10px 0;
	border-radius: 10px;
}

.title-signature {
	margin: 300px 100px 200px -250px;
}
h1 {
	margin-bottom: 20px;
}
</style>
