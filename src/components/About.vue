<template>
	<div class="about-page-wrapper">
		<!-- Fixed Artwork Backdrop (Pink Floyd x Koishi) -->
		<div class="about-backdrop" aria-hidden="true">
			<div class="backdrop-img"></div>
			<div class="backdrop-overlay"></div>
		</div>

		<div class="about-bento-container">
			<!-- Signature Header -->
			<div class="signature-section">
				<img src="@/assets/me.svg" class="title-signature" alt="glowingstone signature" />
			</div>

			<!-- Section Header -->
			<div class="section-header">
				<h1 class="section-title">关于 <span class="sub">About</span></h1>
				<span class="section-meta font-mono">档案 // DOSSIER</span>
			</div>

		<!-- Bento Grid -->
		<div class="bento-grid about-grid">
			<!-- Operator Bio Tile (Span 7) -->
			<div class="metro-tile col-span-7">
				<h3 class="tile-title font-mono">[ 我是谁？ / Who am I ]</h3>
				<div class="bio-body">
					<div class="bio-id-row">
						<img src="https://avatars.githubusercontent.com/u/163746181" alt="Avatar" class="bio-avatar" />
						<div>
							<h2 class="bio-name">glowingstone <span class="name-sub">萤石</span></h2>
							<p class="bio-location font-mono">现居上海 // JVM &amp; 系统级架构设计</p>
						</div>
					</div>
					<p class="bio-desc">
						主要专注于研究 JVM 虚拟机运行原理、Linux/POSIX 兼容系统环境下的网络服务与系统级别程序架构，业余时间也在学习吉他。
					</p>
					<div class="bio-traits font-mono">
						<span class="trait-tag">性能优化</span>
						<span class="trait-tag">JVM 痴子</span>
						<span class="trait-tag">开源探索</span>
					</div>
				</div>
			</div>

			<!-- Infrastructure & Site Architecture Tile (Span 5) -->
			<div class="metro-tile col-span-5">
				<h3 class="tile-title font-mono">[ 关于本站 / About Site ]</h3>
				<div class="infra-body font-mono">
					<div class="infra-row">
						<span class="infra-label">前端技术:</span>
						<span class="infra-val">Vue 3.5 (Composition API)</span>
					</div>
					<div class="infra-row">
						<span class="infra-label">界面风格:</span>
						<span class="infra-val">黑白极简 Metro ASCII</span>
					</div>
					<div class="infra-row">
						<span class="infra-label">网络基础设施:</span>
						<span class="infra-val">Cloudflare Pages &amp; Workers</span>
					</div>
					<div class="infra-row">
						<span class="infra-label">排版引擎:</span>
						<span class="infra-val">KaTeX + Marked</span>
					</div>
					<div class="infra-row">
						<span class="infra-label">背景插画:</span>
						<span class="infra-val">
							<a href="https://bucket.glowingstone.cn/artworks/pf_koishi_2.png" target="_blank" rel="noopener" class="flat-link">
								Pink Floyd × Koishi &rarr;
							</a>
						</span>
					</div>

					<div class="socialify-wrap" @click="redirectToGithub">
						<img
							src="https://socialify.git.ci/glowingstone124/glowingstone-vue/image?description=1&descriptionEditable=%E5%9C%A8github%E4%B8%8A%E6%89%BE%E5%88%B0%E6%88%91&font=Inter&language=1&name=1&owner=1&pattern=Plus&stargazers=1&theme=Dark"
							alt="GitHub Banner"
							class="socialify-img"
						/>
						<span class="img-hint">[点击访问 GitHub 仓库 &rarr;]</span>
					</div>
				</div>
			</div>

			<!-- Music & Guitar Interests Tile (Span 7) -->
			<div class="metro-tile col-span-7">
				<h3 class="tile-title font-mono">[ 音乐偏好 / Music ]</h3>
				<div class="music-body">
					<p class="music-intro">
						前卫摇滚，老摇滚，最爱 70 年代至 80 年代的欧美经典乐队，最喜欢 Pink Floyd 和 Led Zeppelin。
						国内摇滚偏好 Blues 味较重的乐队，例如万能青年旅店。自己也入坑了吉他，购置了一把 Fender Vintage 练习。
					</p>

					<div class="bands-grid font-mono">
						<div class="band-card">
							<span class="band-name">Pink Floyd</span>
							<span class="band-desc">《The Wall》, 《Animals》</span>
						</div>
						<div class="band-card">
							<span class="band-name">Led Zeppelin</span>
							<span class="band-desc">Hard Rock &amp; Blues</span>
						</div>
						<div class="band-card">
							<span class="band-name">万能青年旅店</span>
							<span class="band-desc">《冀西南林路行》</span>
						</div>
						<div class="band-card">
							<span class="band-name">Fender Stratocaster</span>
							<span class="band-desc">Vintage Reissue</span>
						</div>
					</div>
				</div>
			</div>

			<!-- Social Uplink & Contact Tile (Span 5) -->
			<div class="metro-tile col-span-5">
				<h3 class="tile-title font-mono">[ 社交与联络 / Comms ]</h3>
				<div class="comms-body">
					<p class="comms-lead">公开社媒账号，欢迎友好互动：</p>
					<div class="comms-links font-mono">
						<a
							href="https://github.com/glowingstone124"
							target="_blank"
							rel="noopener"
							class="comm-btn"
						>
							<span class="btn-label">[GITHUB]</span>
							<span class="btn-target">@glowingstone124 &rarr;</span>
						</a>

						<a
							href="https://steamcommunity.com/profiles/76561199236874427/"
							target="_blank"
							rel="noopener"
							class="comm-btn"
						>
							<span class="btn-label">[STEAM]</span>
							<span class="btn-target">Steam 个人主页 &rarr;</span>
						</a>

						<a
							href="https://x.com/QuantumOriginal"
							target="_blank"
							rel="noopener"
							class="comm-btn"
						>
							<span class="btn-label">[X / TWITTER]</span>
							<span class="btn-target">@QuantumOriginal &rarr;</span>
						</a>

						<a
							href="mailto:hanseroffcial@outlook.com"
							class="comm-btn"
						>
							<span class="btn-label">[EMAIL]</span>
							<span class="btn-target">hanseroffcial@outlook.com ✉</span>
						</a>
					</div>
				</div>
			</div>
		</div>

		<VisitCounter />
	</div>
</div>
</template>

<script>
import VisitCounter from "@/components/VisitCounter.vue";

export default {
	name: 'About',
	components: { VisitCounter },
	methods: {
		redirectToGithub() {
			window.open('https://github.com/glowingstone124', '_blank');
		},
	},
};
</script>

<style scoped>
/* Page Wrapper */
.about-page-wrapper {
	position: relative;
	width: 100%;
	min-height: 100vh;
}

/* Fixed Artwork Backdrop */
.about-backdrop {
	position: fixed;
	top: 0;
	left: 0;
	width: 100vw;
	height: 100vh;
	z-index: 0;
	pointer-events: none;
	overflow: hidden;
}

.backdrop-img {
	width: 100%;
	height: 100%;
	background-image: url("https://bucket.glowingstone.cn/artworks/pf_koishi_2.png");
	background-size: cover;
	background-position: center;
	background-repeat: no-repeat;
	opacity: 0.38;
	filter: contrast(1.05) brightness(0.9);
}

.backdrop-overlay {
	position: absolute;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
	background: radial-gradient(circle at 50% 30%, rgba(10, 10, 10, 0.45) 0%, rgba(10, 10, 10, 0.88) 75%, #0a0a0a 100%);
}

.about-bento-container {
	position: relative;
	z-index: 1;
	max-width: var(--max-content-width);
	margin: 0 auto 120px;
	padding: 0 40px;
	box-sizing: border-box;
}

/* Frosted Tiles over Backdrop */
.metro-tile {
	background: rgba(18, 18, 18, 0.88);
	backdrop-filter: blur(14px);
	-webkit-backdrop-filter: blur(14px);
	border: 1px solid #333333;
}

.metro-tile:hover {
	background: rgba(26, 26, 26, 0.94);
	border-color: #ffffff;
}

/* Handwritten Signature Section */
.signature-section {
	display: flex;
	justify-content: center;
	align-items: center;
	padding: 85px 0 105px;
	border-bottom: 1px solid rgba(255, 255, 255, 0.15);
	margin-bottom: 60px;
}

.title-signature {
	max-width: 720px;
	width: 100%;
	height: auto;
	display: block;
}

/* Tile Titles */
.tile-title {
	margin: 0 0 24px;
	font-size: 1.05rem;
	color: #ffffff;
	font-weight: 500;
}

/* Bio Tile */
.bio-id-row {
	display: flex;
	align-items: center;
	gap: 24px;
	margin-bottom: 24px;
}

.bio-avatar {
	width: 72px;
	height: 72px;
	border: 1px solid #404040;
	object-fit: cover;
}

.bio-name {
	font-size: 1.8rem;
	margin: 0 0 6px;
	color: #ffffff;
}

.name-sub {
	color: #a3a3a3;
	font-weight: 300;
}

.bio-location {
	font-size: 0.88rem;
	color: #a3a3a3;
	margin: 0;
}

.bio-desc {
	font-size: 1.05rem;
	line-height: 1.85;
	color: #d4d4d4;
	margin-bottom: 28px;
}

.bio-traits {
	display: flex;
	flex-wrap: wrap;
	gap: 12px;
}

.trait-tag {
	font-size: 0.85rem;
	background: #0a0a0a;
	border: 1px solid #333333;
	color: #d4d4d4;
	padding: 6px 14px;
}

/* Infrastructure */
.infra-body {
	display: flex;
	flex-direction: column;
	gap: 16px;
}

.infra-row {
	display: flex;
	justify-content: space-between;
	align-items: center;
	font-size: 0.88rem;
	padding-bottom: 10px;
	border-bottom: 1px solid #262626;
}

.infra-label {
	color: #a3a3a3;
}

.infra-val {
	color: #ffffff;
}

.socialify-wrap {
	margin-top: 16px;
	cursor: pointer;
	border: 1px solid #333333;
	transition: border-color 0.15s ease;
}

.socialify-wrap:hover {
	border-color: #ffffff;
}

.socialify-img {
	width: 100%;
	display: block;
}

.img-hint {
	display: block;
	text-align: center;
	padding: 10px;
	background: #0a0a0a;
	font-size: 0.82rem;
	color: #a3a3a3;
	border-top: 1px solid #262626;
}

/* Music */
.music-intro {
	font-size: 1.05rem;
	color: #d4d4d4;
	line-height: 1.85;
	margin: 0 0 28px;
}

.bands-grid {
	display: grid;
	grid-template-columns: repeat(2, 1fr);
	gap: 16px;
}

.band-card {
	background: #0a0a0a;
	border: 1px solid #262626;
	padding: 18px 20px;
	display: flex;
	flex-direction: column;
	gap: 6px;
}

.band-name {
	font-size: 1rem;
	color: #ffffff;
	font-weight: 500;
}

.band-desc {
	font-size: 0.82rem;
	color: #a3a3a3;
}

/* Comms */
.comms-lead {
	font-size: 1rem;
	color: #d4d4d4;
	margin: 0 0 20px;
}

.comms-links {
	display: flex;
	flex-direction: column;
	gap: 12px;
}

.comm-btn {
	display: flex;
	justify-content: space-between;
	align-items: center;
	padding: 14px 20px;
	background: #0a0a0a;
	border: 1px solid #262626;
	text-decoration: none;
	color: #ffffff;
	font-size: 0.9rem;
	transition: all 0.15s ease;
}

.comm-btn:hover {
	background: #ffffff;
	color: #000000;
	border-color: #ffffff;
}

.comm-btn:hover .btn-label,
.comm-btn:hover .btn-target {
	color: #000000;
}

.btn-label {
	color: #a3a3a3;
}

.btn-target {
	color: #ffffff;
}

@media screen and (max-width: 768px) {
	.signature-section {
		padding: 48px 0 54px;
	}
	.title-signature {
		max-width: 92%;
	}
	.bands-grid {
		grid-template-columns: 1fr;
	}
	.about-bento-container {
		padding: 0 20px;
	}
}
</style>
