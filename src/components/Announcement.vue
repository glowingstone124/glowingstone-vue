<template>
	<div class="announcement-page">
		<div class="section-header">
			<h1 class="section-title">特别声明 <span class="sub">Notice</span></h1>
			<span class="section-meta font-mono">公示 / ANNOUNCEMENT</span>
		</div>

		<div class="announcement-card font-mono">
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
</template>

<script setup>
</script>

<style scoped>
.announcement-page {
	max-width: var(--max-content-width);
	margin: 0 auto var(--section-spacing);
	padding: 0 32px;
	box-sizing: border-box;
}

.announcement-card {
	background: #121212;
	border: 1px solid #262626;
	border-left: 4px solid #ffffff;
	padding: 48px;
}

.title-row {
	display: flex;
	align-items: center;
	gap: 20px;
	margin-bottom: 28px;
}

.anno-head {
	font-size: 2.2rem;
	color: #ffffff;
	margin: 0;
	font-weight: 600;
}

.anno-img {
	height: 60px;
	width: auto;
	display: block;
}

.anno-callout {
	background: #171717;
	border: 1px solid #333333;
	padding: 20px 24px;
	margin-bottom: 32px;
}

.lead-strong {
	font-size: 1.25rem;
	color: #ffffff;
	font-weight: 600;
	margin: 0 0 8px;
}

.lead-sub {
	font-size: 0.95rem;
	color: #a3a3a3;
	margin: 0;
}

.anno-body {
	color: #d4d4d4;
	font-size: 0.95rem;
	line-height: 1.85;
}

.anno-body p {
	margin-bottom: 16px;
}

.anno-divider {
	border: none;
	height: 1px;
	background: #262626;
	margin: 28px 0;
}

@media screen and (max-width: 768px) {
	.announcement-card {
		padding: 28px 20px;
	}
	.title-row {
		flex-direction: column;
		align-items: flex-start;
		gap: 12px;
	}
	.anno-head {
		font-size: 1.8rem;
	}
}
</style>
