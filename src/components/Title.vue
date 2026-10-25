<template>
	<div class="apple-specs-section">
		<!-- Section Header -->
		<div class="specs-header">
			<span class="apple-badge">ENGINEERING & ARCHITECTURE</span>
			<h2 class="specs-heading">Designed by Glowingstone</h2>
			<p class="specs-subheading">专注 JVM 虚拟机运行机制、高并发分布式系统与底层体系结构设计。</p>
		</div>

		<!-- Bento Grid -->
		<div class="specs-bento-grid">
			<!-- Signature Card (Hero Span) -->
			<div class="bento-card col-span-12 signature-card" @click="replaySignature">
				<div class="signature-wrapper">
					<img :key="signatureKey" src="@/assets/me.svg" class="signature-svg" alt="Glowingstone signature" />
				</div>
				<div class="signature-caption">
					<span class="caption-brand">GLOWSTONE 萤石</span>
					<span class="caption-divider">·</span>
					<span class="caption-desc">JVM &amp; System Architecture Engineer</span>
				</div>
			</div>

			<!-- Quote Card -->
			<div class="bento-card col-span-7 quote-card">
				<div class="card-inner-header">
					<span class="card-label">PHILOSOPHY</span>
				</div>
				<blockquote class="apple-quote">
					<span v-html="formattedQuote"></span>
				</blockquote>
				<div class="quote-cite">— Pink Floyd 《The Wall》</div>
			</div>

			<!-- Focus Card -->
			<div class="bento-card col-span-5 focus-card">
				<div class="card-inner-header">
					<span class="card-label">CORE FOCUS</span>
				</div>
				<h3 class="focus-title">系统级程序设计与虚拟机演进</h3>
				<p class="focus-desc">深入研究 JVM 垃圾回收器、即时编译技术、内存模型以及 Linux 原生高性能网络 IO 栈。</p>
				<div class="apple-tags-row">
					<span class="apple-spec-tag">Java</span>
					<span class="apple-spec-tag">Kotlin</span>
					<span class="apple-spec-tag">Rust</span>
					<span class="apple-spec-tag">Linux</span>
					<span class="apple-spec-tag">JVM Internal</span>
					<span class="apple-spec-tag">Spring Boot</span>
				</div>
			</div>

			<!-- GitHub Activity Card -->
			<div class="bento-card col-span-12 github-card">
				<div class="card-inner-header">
					<span class="card-label">OPEN SOURCE CONTRIBUTIONS</span>
					<a href="https://github.com/glowingstone124" target="_blank" rel="noopener" class="apple-inline-link">
						GitHub @glowingstone124 &rarr;
					</a>
				</div>
				<div class="github-scroll-box">
					<iframe
						class="github-frame"
						frameborder="0"
						src="https://git-graph.vercel.app/embed/glowingstone124?showColorLegend=true&showWeekdayLabels=true&showMonthLabels=true&showTotalCount=true&blockMargin=6&blockRadius=4&blockSize=17&fontSize=14&weekStart=0&year=2024"
						title="GitHub Contributions"
					></iframe>
				</div>
			</div>

			<!-- Workstation Rig Card -->
			<div class="bento-card col-span-6 workstation-card">
				<div class="card-inner-header">
					<span class="card-label">HARDWARE ENVIRONMENT</span>
				</div>
				<div class="spec-rows">
					<div class="spec-row">
						<span class="spec-key">便携终端</span>
						<span class="spec-value">MacBook Pro 16" (M3 Max · 36GB 统一内存)</span>
					</div>
					<div class="spec-row">
						<span class="spec-key">主力工作站</span>
						<span class="spec-value">AMD Ryzen 9 9900X + 192GB DDR5 平台</span>
					</div>
					<div class="spec-row">
						<span class="spec-key">系统偏好</span>
						<span class="spec-value">Debian / Rocky Linux / macOS Sequoia</span>
					</div>
				</div>
			</div>

			<!-- Code Metrics Card -->
			<div class="bento-card col-span-6 metrics-card">
				<div class="card-inner-header">
					<span class="card-label">CODE METRICS</span>
				</div>
				<div class="metric-bars">
					<div class="metric-item">
						<div class="metric-labels">
							<span class="lang-name">Java (Core &amp; JVM)</span>
							<span class="lang-lines">&ge; 25,000 行</span>
						</div>
						<div class="apple-progress-track">
							<div class="apple-progress-fill" style="width: 95%;"></div>
						</div>
					</div>

					<div class="metric-item">
						<div class="metric-labels">
							<span class="lang-name">Kotlin (JVM &amp; Coroutines)</span>
							<span class="lang-lines">~9,000 行</span>
						</div>
						<div class="apple-progress-track">
							<div class="apple-progress-fill" style="width: 70%; background: #6e3ff3;"></div>
						</div>
					</div>

					<div class="metric-item">
						<div class="metric-labels">
							<span class="lang-name">JavaScript / TypeScript</span>
							<span class="lang-lines">~3,000 行</span>
						</div>
						<div class="apple-progress-track">
							<div class="apple-progress-fill" style="width: 45%; background: #0071e3;"></div>
						</div>
					</div>

					<div class="metric-item">
						<div class="metric-labels">
							<span class="lang-name">C / C++ / Systems</span>
							<span class="lang-lines">~2,000 行</span>
						</div>
						<div class="apple-progress-track">
							<div class="apple-progress-fill" style="width: 35%; background: #34c759;"></div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
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
	computed: {
		formattedQuote() {
			if (!this.msg) {
				return "I got electric light, and I got second sight.<br>Got amazing powers of observation.<br>And that is how I know,<br>When I try to get through on the telephone to you,<br>There'll be nobody home.";
			}
			return this.msg.replace(/\n/g, '<br>');
		},
	},
	methods: {
		replaySignature() {
			this.signatureKey += 1;
		},
	},
};
</script>

<style scoped>
.apple-specs-section {
	width: 100%;
	padding: 24px 0 0;
}

.specs-header {
	margin-bottom: 28px;
}

.specs-heading {
	font-size: 32px;
	font-weight: 700;
	color: #1d1d1f;
	margin: 8px 0 10px;
	letter-spacing: -0.02em;
}

.specs-subheading {
	font-size: 16px;
	color: #86868b;
	margin: 0;
}

/* Bento Grid */
.specs-bento-grid {
	display: grid;
	grid-template-columns: repeat(12, 1fr);
	gap: 24px;
}

.col-span-12 { grid-column: span 12; }
.col-span-7 { grid-column: span 7; }
.col-span-6 { grid-column: span 6; }
.col-span-5 { grid-column: span 5; }

.bento-card {
	background: #ffffff;
	border: 1px solid rgba(0, 0, 0, 0.08);
	border-radius: var(--apple-radius-card, 24px);
	padding: 32px;
	box-shadow: 0 4px 20px rgba(0, 0, 0, 0.04);
	transition: transform 0.25s ease, box-shadow 0.25s ease;
	display: flex;
	flex-direction: column;
	justify-content: space-between;
}

.bento-card:hover {
	transform: translateY(-4px);
	box-shadow: 0 14px 32px rgba(0, 0, 0, 0.08);
}

.card-inner-header {
	display: flex;
	align-items: center;
	justify-content: space-between;
	margin-bottom: 18px;
}

.card-label {
	font-size: 11px;
	font-weight: 700;
	letter-spacing: 0.06em;
	color: #86868b;
	text-transform: uppercase;
}

.apple-inline-link {
	font-size: 13px;
	color: #0071e3;
	font-weight: 500;
}

.apple-inline-link:hover {
	text-decoration: underline;
}

/* Signature Card */
.signature-card {
	background: #fbfbfd;
	text-align: center;
	padding: 56px 32px 44px;
	align-items: center;
	cursor: pointer;
}

.signature-wrapper {
	max-width: 520px;
	width: 100%;
	margin-bottom: 24px;
	transition: transform 0.2s ease;
}

.signature-card:hover .signature-wrapper {
	transform: scale(1.02);
}

.signature-svg {
	width: 100%;
	height: auto;
	display: block;
}

.signature-caption {
	display: flex;
	align-items: center;
	gap: 12px;
	font-size: 14px;
	color: #86868b;
}

.caption-brand {
	font-weight: 600;
	color: #1d1d1f;
}

/* Quote Card */
.apple-quote {
	margin: 0 0 16px;
	font-size: 16px;
	line-height: 1.8;
	color: #1d1d1f;
	font-style: italic;
	border-left: 3px solid #0071e3;
	padding-left: 20px;
}

.quote-cite {
	font-size: 13px;
	color: #86868b;
	text-align: right;
}

/* Focus Card */
.focus-title {
	font-size: 19px;
	font-weight: 700;
	color: #1d1d1f;
	margin: 0 0 10px;
}

.focus-desc {
	font-size: 14px;
	line-height: 1.6;
	color: #515154;
	margin: 0 0 20px;
}

.apple-tags-row {
	display: flex;
	flex-wrap: wrap;
	gap: 8px;
}

.apple-spec-tag {
	padding: 6px 14px;
	background: #f5f5f7;
	border-radius: 980px;
	font-size: 12px;
	font-weight: 500;
	color: #1d1d1f;
}

/* GitHub */
.github-scroll-box {
	width: 100%;
	overflow-x: auto;
}

.github-frame {
	width: 100%;
	min-width: 800px;
	height: 190px;
	border: none;
}

/* Workstation */
.spec-rows {
	display: flex;
	flex-direction: column;
	gap: 16px;
}

.spec-row {
	display: flex;
	justify-content: space-between;
	align-items: center;
	padding-bottom: 12px;
	border-bottom: 1px solid #f0f0f2;
	font-size: 14px;
}

.spec-key {
	color: #86868b;
}

.spec-value {
	color: #1d1d1f;
	font-weight: 500;
	text-align: right;
}

/* Code Metrics */
.metric-bars {
	display: flex;
	flex-direction: column;
	gap: 16px;
}

.metric-item {
	display: flex;
	flex-direction: column;
	gap: 6px;
}

.metric-labels {
	display: flex;
	justify-content: space-between;
	font-size: 13px;
	color: #1d1d1f;
}

.lang-lines {
	color: #86868b;
}

.apple-progress-track {
	width: 100%;
	height: 6px;
	background: #f5f5f7;
	border-radius: 980px;
	overflow: hidden;
}

.apple-progress-fill {
	height: 100%;
	background: #1d1d1f;
	border-radius: 980px;
}

@media screen and (max-width: 960px) {
	.col-span-7, .col-span-6, .col-span-5 {
		grid-column: span 12;
	}
	.spec-row {
		flex-direction: column;
		align-items: flex-start;
		gap: 4px;
	}
	.spec-value {
		text-align: left;
	}
}
</style>
