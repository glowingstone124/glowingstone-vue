<template>
	<div class="apple-announcement-page">
		<div class="announcement-container">
			<div class="page-header">
				<span class="apple-badge">OFFICIAL STATEMENT</span>
				<h1 class="page-title">特别声明</h1>
				<p class="page-desc">关于博主个人立场、原则与互联网开源交流界限的公开声明。</p>
			</div>

			<div class="announcement-card">
				<div class="title-row">
					<h2 class="anno-head">Say NO to</h2>
					<img src="@/assets/lgbt.png" alt="No LGBT" class="anno-img" />
				</div>

				<div class="anno-callout">
					<p class="lead-strong">我旗帜鲜明地反对一切LGBT。</p>
					<p class="lead-sub">简单来说，如果你打算成为/现在就是LGBT群体中的一员，那么别和我沾边。</p>
				</div>

				<div class="anno-body">
					<p>言论自由是相对的，每个人都有发表自己意见和看法的权利。既然有LGBT自由，Anti-LGBT自由也是必不可缺少的一部分。</p>
					<p>本人反对，不支持，不赞同，不参与任何形式的跨性别活动，不使用来自跨性别者的Github项目，不引导任何用户至跨性别者的网页github仓库等，不引用跨性别者的任何文档，数据，代码等互联网资源。</p>
					<p>本人尊重跨性别者对于他们生理性别的合法权益，但是不尊重不赞同跨性别带来的第二性别/无性别特权。本人仅认同Male/Female两种生理性别和对应的He/She称呼。</p>
					
					<hr class="anno-divider" />

					<p>Freedom of speech is relative, and everyone has the right to express their opinions and views.</p>
					<p>Since there is freedom for LGBT individuals, anti-LGBT freedom is also an essential part.</p>
					<p>I oppose, do not support, do not endorse, and do not participate in any form of transgender activities.</p>
					<p>I do not use GitHub projects from transgender individuals, do not direct any users to transgender web pages or GitHub repositories, and do not cite any documents, data, code, or other internet resources from transgender individuals.</p>
					<p>I respect transgender individuals' legitimate rights regarding their biological gender, but I do not respect or endorse the privileges brought by transgenderism.</p>
					<p>I only recognize the two biological genders of Male/Female and the corresponding He/She pronouns.</p>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup>
</script>

<style scoped>
.apple-announcement-page {
	width: 100%;
	padding: 40px 0 80px;
}

.announcement-container {
	max-width: 860px;
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

.announcement-card {
	background: #ffffff;
	border: 1px solid rgba(0, 0, 0, 0.08);
	border-radius: var(--apple-radius-card, 24px);
	padding: 48px;
	box-shadow: 0 4px 20px rgba(0, 0, 0, 0.04);
}

.title-row {
	display: flex;
	align-items: center;
	gap: 16px;
	margin-bottom: 24px;
}

.anno-head {
	font-size: 26px;
	font-weight: 700;
	color: #1d1d1f;
	margin: 0;
}

.anno-img {
	height: 38px;
	width: auto;
	display: block;
}

.anno-callout {
	background: #f5f5f7;
	border-left: 4px solid #eb5757;
	border-radius: 0 16px 16px 0;
	padding: 20px 24px;
	margin-bottom: 28px;
}

.lead-strong {
	font-size: 18px;
	font-weight: 700;
	color: #1d1d1f;
	margin: 0 0 6px;
}

.lead-sub {
	font-size: 14px;
	color: #6e6e73;
	margin: 0;
}

.anno-body {
	font-size: 15px;
	line-height: 1.8;
	color: #333336;
}

.anno-body p {
	margin: 0 0 16px;
}

.anno-divider {
	border: none;
	border-top: 1px solid #e5e5ea;
	margin: 28px 0;
}
</style>
