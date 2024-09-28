<template>
	<div class="contents">
		<img src="@root/src/assets/me.svg" class="title-signature">
		<div class="friends">
			<div class="info-box">
				<h1>我是谁？</h1>
				<p>我是glowingstone124，现居上海，是一名普通的准高中生。平时会玩玩apex，bf,cs等fps游戏，偶尔追追新番老番。</p>
				<p>技术上嘛...毫无建树，只是稍微会一些Java和Web而已，但是在努力学习。</p>
				<p>Miku厨，半个车车壬，群友们一致认为的抽象大手子</p>
				<p>(头像都是恋 <a href="https://www.pixiv.net/artworks/112973410">头像pixiv id</a></p>
			</div>

			<div class="info-box">
				<h1>关于这个网站</h1>
				<a href="https://www.pixiv.net/artworks/92320484">背景来自Pixiv</a>
				<p>这个网站是完全自建的（你没听错，没有使用任何ui库和模板，纯vue手敲）</p>
				<p>前端：Vue3</p>
				<img @click="redirectToGithub" src="https://socialify.git.ci/glowingstone124/glowingstone-vue/image?description=1&descriptionEditable=%E5%9C%A8github%E4%B8%8A%E6%89%BE%E5%88%B0%E6%88%91&font=Inter&language=1&name=1&owner=1&pattern=Plus&stargazers=1&theme=Dark" alt="">
				<p>没有后端，基于vercel部署啦</p>
			</div>

			<div class="info-box">
				<h1>爱好</h1>
				<p>游戏、编程、动漫、自媒体、顺便还会偶尔写小说</p>
				<a href="https://steamcommunity.com/profiles/76561199236874427/">Steam</a>
				<a href="https://space.bilibili.com/687290823">哗哩哗哩</a>
				<a href="https://music.163.com/#/user/home?id=5164416595">网易云</a>
			</div>

			<div class="info-box">
				<h1>音乐偏好</h1>
				<p>流行音乐以欧美，日本居多，狠狠听东方。 <br> 摇滚爱好者！最爱的是万青和Neckdeep，少女乐队番好文明！</p>
			</div>

			<div class="info-box">
				<h1>联系我</h1>
				<a href="https://qm.qq.com/q/H7Gin9dAuC"><img src="mail.svg" alt="">Mail</a>
				<a href="mailto:hanseroffcial@outlook.com"><img src="chat.svg" alt="">QQ</a>
			</div>
		</div>
	</div>
</template>

<script>
export default {
	name: 'About',
	components: {},
	methods: {
		redirectToGithub() {
			window.location.href = 'https://github.com/glowingstone124';
		}
	},
	mounted() {
		document.body.style.backgroundImage = 'url("/komeji2.jpg")';
		document.body.style.backgroundSize = 'cover';
		document.body.style.backgroundRepeat = 'no-repeat'
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
	margin:auto;
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
	border-radius: 15px;
	padding: 20px;
	margin: auto auto 10px;
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
	margin: 200px 0;
}
</style>
