<template>
	<section class="metro-hero-section">
		<!-- Hero Handwritten Signature Header -->
		<div class="hero-signature-card">
			<span class="card-crosshair ch-tl">+</span>
			<span class="card-crosshair ch-tr">+</span>
			<span class="card-crosshair ch-bl">+</span>
			<span class="card-crosshair ch-br">+</span>

			<div class="signature-frame" title="[点击重新书写签名]" @click="replaySignature">
				<img :key="signatureKey" src="@/assets/me.svg" class="hero-signature-img" alt="glowingstone signature" />
			</div>
			<div class="signature-sub font-mono">
				GLOWSTONE // 萤石 // JVM &amp; SYSTEM ARCHITECTURE
			</div>
		</div>

		<!-- Metro Tiles Grid -->
		<div class="bento-grid hero-grid">
			<!-- Quote Tile -->
			<div class="metro-tile col-span-7">
				<h3 class="tile-title font-mono">[ 引言 / Quote ]</h3>
				<blockquote class="quote-text font-mono">
					<span v-html="formattedQuote"></span><span class="blinking-caret">_</span>
				</blockquote>
				<span class="quote-author font-mono">&mdash; Pink Floyd 《The Wall》</span>
			</div>

			<!-- Core Focus Tile -->
			<div class="metro-tile col-span-5">
				<h3 class="tile-title font-mono">[ 领域 / Focus ]</h3>
				<p class="tile-desc">
					专注于研究 <strong>JVM 虚拟机</strong>、<strong>分布式网络服务</strong> 与 <strong>系统级底层程序设计</strong>。
				</p>
				<div class="flat-tags font-mono">
					<span class="flat-tag">Java</span>
					<span class="flat-tag">Kotlin</span>
					<span class="flat-tag">Rust</span>
					<span class="flat-tag">Linux</span>
					<span class="flat-tag">Spring</span>
					<span class="flat-tag">Vue.js</span>
				</div>
			</div>

			<!-- GitHub Activity Tile -->
			<div class="metro-tile col-span-12">
				<div class="tile-header-row font-mono">
					<h3 class="tile-title">[ GitHub 动态 / Activity ]</h3>
					<a href="https://github.com/glowingstone124" target="_blank" rel="noopener" class="flat-link">
						@glowingstone124 &rarr;
					</a>
				</div>
				<div class="github-container">
					<iframe
						class="github-frame"
						frameborder="0"
						src="https://git-graph.vercel.app/embed/glowingstone124?showColorLegend=true&showWeekdayLabels=true&showMonthLabels=true&showTotalCount=true&blockMargin=6&blockRadius=0&blockSize=17&fontSize=16&weekStart=0&year=2024"
						title="GitHub Contributions"
					></iframe>
				</div>
			</div>

			<!-- Workstation Rig Tile -->
			<div class="metro-tile col-span-6">
				<h3 class="tile-title font-mono">[ 开发环境 / Environment ]</h3>
				<div class="spec-table font-mono">
					<div class="spec-item">
						<span class="spec-k">便携终端:</span>
						<span class="spec-v">MacBook Pro 16" (M3 Max 10P+4E / 36G)</span>
					</div>
					<div class="spec-item">
						<span class="spec-k">主力工作站:</span>
						<span class="spec-v">AMD Ryzen 9 9900X + 192GB DDR5</span>
					</div>
					<div class="spec-item">
						<span class="spec-k">Linux 偏好:</span>
						<span class="spec-v">Debian = Rocky Linux &gt; Ubuntu &gt; Arch</span>
					</div>
				</div>
			</div>

			<!-- Language Code Metrics Tile -->
			<div class="metro-tile col-span-6">
				<h3 class="tile-title font-mono">[ 代码量积累 / Metrics ]</h3>
				<div class="metrics-list font-mono">
					<div class="metric-row">
						<div class="metric-info">
							<span>Java (Core &amp; JVM)</span>
							<span>&ge; 25,000 行</span>
						</div>
						<div class="flat-progress">
							<div class="progress-bar" style="width: 95%;"></div>
						</div>
					</div>

					<div class="metric-row">
						<div class="metric-info">
							<span>Kotlin (仅JVM)</span>
							<span>~9,000 行</span>
						</div>
						<div class="flat-progress">
							<div class="progress-bar" style="width: 70%;"></div>
						</div>
					</div>

					<div class="metric-row">
						<div class="metric-info">
							<span>JavaScript / TypeScript</span>
							<span>~3,000 行</span>
						</div>
						<div class="flat-progress">
							<div class="progress-bar" style="width: 45%;"></div>
						</div>
					</div>

					<div class="metric-row">
						<div class="metric-info">
							<span>C / C++</span>
							<span>~2,000 行</span>
						</div>
						<div class="flat-progress">
							<div class="progress-bar" style="width: 35%;"></div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>
</template>

<script>
export default {
	name: 'Title',
	props: {
		msg: {
			type: String,
			default: '',
		},
	},
	data() {
		return {
			signatureKey: 0,
		};
	},
	methods: {
		replaySignature() {
			this.signatureKey += 1;
		},
	},
	computed: {
		formattedQuote() {
			if (!this.msg) {
				return "I got electric light, and I got second sight.<br>Got amazing powers of observation.<br>And that is how I know,<br>When I try to get through on the telephone to you,<br>There'll be nobody home.";
			}
			return this.msg.replace(/\n/g, '<br>');
		},
	},
};
</script>

<style scoped>
.metro-hero-section {
	max-width: var(--max-content-width);
	margin: 0 auto 120px;
	padding: 0 40px;
	box-sizing: border-box;
}

/* Handwritten Signature Hero Card */
.hero-signature-card {
	background: #121212;
	border: 1px solid #262626;
	border-left: 4px solid #ffffff;
	padding: 96px 40px 84px;
	margin-bottom: 56px;
	text-align: center;
	display: flex;
	flex-direction: column;
	align-items: center;
	justify-content: center;
	position: relative;
	transition: border-color 0.2s ease;
}

.hero-signature-card:hover {
	border-color: #404040;
}

/* Card Registration Crosshairs */
.card-crosshair {
	position: absolute;
	font-size: 0.9rem;
	color: #444444;
	font-family: var(--font-mono);
	pointer-events: none;
	user-select: none;
	transition: color 0.2s ease, transform 0.25s ease;
}

.ch-tl { top: 12px; left: 16px; }
.ch-tr { top: 12px; right: 16px; }
.ch-bl { bottom: 12px; left: 16px; }
.ch-br { bottom: 12px; right: 16px; }

.hero-signature-card:hover .card-crosshair {
	color: #ffffff;
	transform: rotate(45deg);
}

.signature-frame {
	max-width: 720px;
	width: 100%;
	margin-bottom: 36px;
	cursor: pointer;
	transition: transform 0.2s ease;
}

.signature-frame:hover {
	transform: scale(1.015);
}

.hero-signature-img {
	width: 100%;
	height: auto;
	display: block;
}

.signature-sub {
	font-size: 1rem;
	color: #a3a3a3;
	letter-spacing: 2px;
}

/* Hero Grid */
.hero-grid {
	margin-top: 10px;
}

.tile-title {
	margin: 0 0 20px;
	font-size: 1.05rem;
	color: #ffffff;
	font-weight: 500;
}

.tile-header-row {
	display: flex;
	justify-content: space-between;
	align-items: baseline;
	margin-bottom: 20px;
}

.flat-link {
	color: #ffffff;
	text-decoration: none;
	font-size: 0.88rem;
}

.flat-link:hover {
	text-decoration: underline;
}

/* Quote */
.quote-text {
	margin: 0 0 18px;
	padding: 18px 22px;
	background: #0a0a0a;
	border-left: 3px solid #ffffff;
	color: #d4d4d4;
	font-size: 1rem;
	line-height: 1.8;
	font-style: italic;
}

.quote-author {
	display: block;
	font-size: 0.85rem;
	color: #737373;
	text-align: right;
}

.blinking-caret {
	display: inline-block;
	color: #ffffff;
	margin-left: 2px;
	font-weight: 700;
	animation: caret-blink 1s step-start infinite;
}

@keyframes caret-blink {
	0%, 100% { opacity: 1; }
	50% { opacity: 0; }
}

/* Focus */
.tile-desc {
	margin: 0 0 26px;
	font-size: 1.05rem;
	line-height: 1.85;
	color: #d4d4d4;
}

.tile-desc strong {
	color: #ffffff;
}

.flat-tags {
	display: flex;
	flex-wrap: wrap;
	gap: 10px;
}

.flat-tag {
	padding: 8px 16px;
	background: #0a0a0a;
	border: 1px solid #333333;
	color: #d4d4d4;
	font-size: 0.85rem;
	transition: all 0.15s ease;
}

.flat-tag:hover {
	color: #000000;
	background: #ffffff;
	border-color: #ffffff;
}

/* GitHub */
.github-container {
	width: 100%;
	overflow-x: auto;
}

.github-frame {
	width: 100%;
	min-width: 900px;
	height: 205px;
	border: none;
	background: transparent;
	display: block;
}

/* Spec */
.spec-table {
	display: flex;
	flex-direction: column;
	gap: 16px;
}

.spec-item {
	display: flex;
	gap: 16px;
	font-size: 0.92rem;
	padding-bottom: 14px;
	border-bottom: 1px solid #262626;
}

.spec-k {
	color: #a3a3a3;
	min-width: 110px;
}

.spec-v {
	color: #ffffff;
}

/* Metrics */
.metrics-list {
	display: flex;
	flex-direction: column;
	gap: 20px;
}

.metric-row {
	display: flex;
	flex-direction: column;
	gap: 8px;
}

.metric-info {
	display: flex;
	justify-content: space-between;
	font-size: 0.88rem;
	color: #a3a3a3;
}

.flat-progress {
	width: 100%;
	height: 8px;
	background: #0a0a0a;
	border: 1px solid #262626;
}

.progress-bar {
	height: 100%;
	background: #ffffff;
}

@media screen and (max-width: 768px) {
	.hero-signature-card {
		padding: 48px 20px 40px;
	}
	.signature-frame {
		max-width: 92%;
		margin-bottom: 24px;
	}
	.signature-sub {
		font-size: 0.85rem;
		letter-spacing: 1px;
	}
	.spec-item {
		flex-direction: column;
		gap: 6px;
	}
	.spec-k {
		min-width: unset;
	}
}
</style>
