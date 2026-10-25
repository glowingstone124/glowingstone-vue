<template>
	<div class="apple-about-page">
		<div class="about-container">
			<!-- Section Header -->
			<div class="page-header">
				<span class="apple-badge">BIOGRAPHY &amp; PROFILE</span>
				<h1 class="page-title">关于萤石</h1>
				<p class="page-desc">JVM 虚拟机狂热者、后端与系统级程序设计、独立开发与音乐爱好。</p>
			</div>

			<!-- Bento Grid -->
			<div class="about-bento-grid">
				<!-- 1. Operator Bio Tile (Span 7) -->
				<div class="bento-tile col-span-7 bio-tile">
					<div class="tile-header">
						<span class="tile-tag">WHO AM I</span>
					</div>
					<div class="bio-main">
						<div class="bio-avatar-row">
							<img src="https://avatars.githubusercontent.com/u/163746181" alt="Avatar" class="bio-avatar" />
							<div>
								<h2 class="bio-title">glowingstone <span class="sub-name">萤石</span></h2>
								<p class="bio-subtitle">现居上海 · JVM &amp; 系统级架构设计</p>
							</div>
						</div>
						<p class="bio-paragraph">
							主要专注于研究 JVM 虚拟机内部机制、Linux/POSIX 规范环境下的高并发网络服务与底层程序架构。业余时间在持续弹奏与研习吉他音乐。
						</p>
						<div class="bio-badges">
							<span class="apple-badge-pill">性能调优</span>
							<span class="apple-badge-pill creatives">JVM 痴子</span>
							<span class="apple-badge-pill developers">开源探索</span>
						</div>
					</div>
				</div>

				<!-- 2. Infrastructure & Site Architecture Tile (Span 5) -->
				<div class="bento-tile col-span-5 infra-tile">
					<div class="tile-header">
						<span class="tile-tag">ABOUT THIS SITE</span>
					</div>
					<div class="infra-list">
						<div class="infra-item">
							<span class="i-key">前端框架</span>
							<span class="i-val">Vue 3.5 (Composition API)</span>
						</div>
						<div class="infra-item">
							<span class="i-key">视觉风格</span>
							<span class="i-val">Apple Newsroom 极简美学</span>
						</div>
						<div class="infra-item">
							<span class="i-key">网络架构</span>
							<span class="i-val">Cloudflare Pages &amp; Workers</span>
						</div>
						<div class="infra-item">
							<span class="i-key">数学排版</span>
							<span class="i-val">KaTeX + Marked Engine</span>
						</div>
					</div>

					<div class="github-repo-card" @click="redirectToGithub">
						<img
							src="https://socialify.git.ci/glowingstone124/glowingstone-vue/image?description=1&descriptionEditable=%E5%9C%A8github%E4%B8%8A%E6%89%BE%E5%88%B0%E6%88%91&font=Inter&language=1&name=1&owner=1&pattern=Plus&stargazers=1&theme=Light"
							alt="GitHub Banner"
							class="github-banner-img"
						/>
					</div>
				</div>

				<!-- 3. Music & Guitar Interests Tile (Span 7) -->
				<div class="bento-tile col-span-7 music-tile">
					<div class="tile-header">
						<span class="tile-tag">MUSIC &amp; GUITAR</span>
					</div>
					<p class="music-intro">
						前卫摇滚与布鲁斯忠实拥趸，热爱 70 至 80 年代经典欧美摇滚乐，最崇尚 Pink Floyd 与 Led Zeppelin。国内摇滚偏好万能青年旅店等厚重叙事乐队。日常使用 Fender Stratocaster Vintage 练习。
					</p>
					<div class="music-cards-grid">
						<div class="music-mini-card">
							<span class="m-title">Pink Floyd</span>
							<span class="m-sub">《The Wall》, 《Animals》</span>
						</div>
						<div class="music-mini-card">
							<span class="m-title">Led Zeppelin</span>
							<span class="m-sub">Hard Rock &amp; Blues</span>
						</div>
						<div class="music-mini-card">
							<span class="m-title">万能青年旅店</span>
							<span class="m-sub">《冀西南林路行》</span>
						</div>
						<div class="music-mini-card">
							<span class="m-title">Fender Stratocaster</span>
							<span class="m-sub">Vintage Reissue</span>
						</div>
					</div>
				</div>

				<!-- 4. Social Uplink & Contact Tile (Span 5) -->
				<div class="bento-tile col-span-5 comms-tile">
					<div class="tile-header">
						<span class="tile-tag">GET IN TOUCH</span>
					</div>
					<p class="comms-intro">欢迎就系统技术、开源或音乐进行交流：</p>
					<div class="contact-links-list">
						<a href="https://github.com/glowingstone124" target="_blank" rel="noopener" class="contact-row-btn">
							<div class="btn-left">
								<span class="btn-brand">GitHub</span>
								<span class="btn-handle">@glowingstone124</span>
							</div>
							<span class="btn-arrow">&rarr;</span>
						</a>

						<a href="https://steamcommunity.com/profiles/76561199236874427/" target="_blank" rel="noopener" class="contact-row-btn">
							<div class="btn-left">
								<span class="btn-brand">Steam</span>
								<span class="btn-handle">个人资料库</span>
							</div>
							<span class="btn-arrow">&rarr;</span>
						</a>

						<a href="https://x.com/QuantumOriginal" target="_blank" rel="noopener" class="contact-row-btn">
							<div class="btn-left">
								<span class="btn-brand">X / Twitter</span>
								<span class="btn-handle">@QuantumOriginal</span>
							</div>
							<span class="btn-arrow">&rarr;</span>
						</a>

						<a href="mailto:hanseroffcial@outlook.com" class="contact-row-btn">
							<div class="btn-left">
								<span class="btn-brand">Email</span>
								<span class="btn-handle">hanseroffcial@outlook.com</span>
							</div>
							<span class="btn-arrow">&rarr;</span>
						</a>
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
.apple-about-page {
	width: 100%;
	padding: 40px 0 80px;
}

.about-container {
	max-width: var(--apple-max-width, 1240px);
	margin: 0 auto;
	padding: 0 24px;
	box-sizing: border-box;
}

.page-header {
	margin-bottom: 36px;
}

.page-title {
	font-size: 38px;
	font-weight: 700;
	color: #1d1d1f;
	margin: 8px 0 12px;
	letter-spacing: -0.02em;
}

.page-desc {
	font-size: 16px;
	color: #86868b;
	margin: 0;
}

/* Bento Grid */
.about-bento-grid {
	display: grid;
	grid-template-columns: repeat(12, 1fr);
	gap: 24px;
}

.col-span-7 { grid-column: span 7; }
.col-span-5 { grid-column: span 5; }

.bento-tile {
	background: #ffffff;
	border: 1px solid rgba(0, 0, 0, 0.08);
	border-radius: var(--apple-radius-card, 24px);
	padding: 32px;
	box-shadow: 0 4px 20px rgba(0, 0, 0, 0.04);
	display: flex;
	flex-direction: column;
	justify-content: space-between;
	transition: transform 0.25s ease, box-shadow 0.25s ease;
}

.bento-tile:hover {
	transform: translateY(-4px);
	box-shadow: 0 14px 32px rgba(0, 0, 0, 0.08);
}

.tile-header {
	margin-bottom: 20px;
}

.tile-tag {
	font-size: 11px;
	font-weight: 700;
	letter-spacing: 0.06em;
	color: #86868b;
	text-transform: uppercase;
}

/* Bio */
.bio-avatar-row {
	display: flex;
	align-items: center;
	gap: 18px;
	margin-bottom: 18px;
}

.bio-avatar {
	width: 68px;
	height: 68px;
	border-radius: 50%;
	object-fit: cover;
	border: 2px solid #e5e5ea;
}

.bio-title {
	font-size: 22px;
	font-weight: 700;
	color: #1d1d1f;
	margin: 0;
}

.sub-name {
	font-size: 16px;
	color: #86868b;
	font-weight: 400;
}

.bio-subtitle {
	font-size: 13px;
	color: #86868b;
	margin: 4px 0 0;
}

.bio-paragraph {
	font-size: 15px;
	line-height: 1.7;
	color: #515154;
	margin: 0 0 20px;
}

.bio-badges {
	display: flex;
	gap: 10px;
	flex-wrap: wrap;
}

/* Infra */
.infra-list {
	display: flex;
	flex-direction: column;
	gap: 12px;
	margin-bottom: 20px;
}

.infra-item {
	display: flex;
	justify-content: space-between;
	padding-bottom: 8px;
	border-bottom: 1px solid #f0f0f2;
	font-size: 13px;
}

.i-key {
	color: #86868b;
}

.i-val {
	color: #1d1d1f;
	font-weight: 500;
}

.github-repo-card {
	border-radius: 16px;
	overflow: hidden;
	cursor: pointer;
	border: 1px solid #e5e5ea;
	transition: transform 0.2s ease;
}

.github-repo-card:hover {
	transform: scale(1.02);
}

.github-banner-img {
	width: 100%;
	display: block;
}

/* Music */
.music-intro {
	font-size: 14px;
	line-height: 1.7;
	color: #515154;
	margin: 0 0 20px;
}

.music-cards-grid {
	display: grid;
	grid-template-columns: 1fr 1fr;
	gap: 12px;
}

.music-mini-card {
	background: #f5f5f7;
	padding: 14px 18px;
	border-radius: 16px;
	display: flex;
	flex-direction: column;
	gap: 4px;
}

.m-title {
	font-size: 14px;
	font-weight: 700;
	color: #1d1d1f;
}

.m-sub {
	font-size: 12px;
	color: #86868b;
}

/* Comms */
.comms-intro {
	font-size: 14px;
	color: #6e6e73;
	margin: 0 0 16px;
}

.contact-links-list {
	display: flex;
	flex-direction: column;
	gap: 10px;
}

.contact-row-btn {
	display: flex;
	align-items: center;
	justify-content: space-between;
	padding: 12px 18px;
	background: #f5f5f7;
	border-radius: 16px;
	transition: background-color 0.18s ease, transform 0.15s ease;
}

.contact-row-btn:hover {
	background: #e8e8ed;
	transform: translateX(4px);
}

.btn-left {
	display: flex;
	flex-direction: column;
	gap: 2px;
}

.btn-brand {
	font-size: 13px;
	font-weight: 600;
	color: #1d1d1f;
}

.btn-handle {
	font-size: 12px;
	color: #86868b;
}

.btn-arrow {
	color: #0071e3;
	font-size: 16px;
}

@media screen and (max-width: 860px) {
	.col-span-7, .col-span-5 {
		grid-column: span 12;
	}
	.music-cards-grid {
		grid-template-columns: 1fr;
	}
}
</style>
