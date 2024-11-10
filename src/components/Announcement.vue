<template>
	<div class="announcement">
		<span style="flex-direction: row">
		<h1>Say NO to<img src="@/assets/lgbt.png"/></h1>
		</span>
		<h2>我旗帜鲜明地反对一切LGBT。</h2>
		<h3>简单来说，如果你打算成为/现在就是LGBT群体中的一员，那么别和我沾边。</h3>
		<div>
			<p>
				言论自由是相对的，每个人都有发表自己意见和看法的权利。既然有LGBT自由，Anti-LGBT自由也是必不可缺少的一部分。</p>

			<p>
				本人反对，不支持，不赞同，不参与任何形式的跨性别活动，不使用来自跨性别者的Github项目，不引导任何用户至跨性别者的网页github仓库等，不引用跨性别者的任何文档，数据，代码等互联网资源。</p>

			<p>
				本人尊重跨性别者对于他们生理性别的合法权益，但是不尊重不赞同跨性别带来的第二性别/无性别特权。本人仅认同Male/Female两种生理性别和对应的He/She称呼。</p>

			<p>Freedom of speech is relative, and everyone has the right to express their opinions and views.</p>

			<p>Since there is freedom for LGBT individuals, anti-LGBT freedom is also an essential part.</p>

			<p>I oppose, do not support, do not endorse, and do not participate in any form of transgender
				activities.</p>

			<p>I do not use GitHub projects from transgender individuals, do not direct any users to transgender web
				pages or GitHub repositories, and do not cite any documents, data, code, or other internet resources
				from transgender individuals.</p>

			<p>I respect transgender individuals' legitimate rights regarding their biological gender, but I do not
				respect or endorse the privileges brought by transgenderism.</p>

			<p>I only recognize the two biological genders of Male/Female and the corresponding He/She pronouns.</p>
		</div>
	</div>
</template>

<style scoped>
.announcement {
	margin-top: 100px;
	display: flex;
	align-items: center;
	justify-content: center;
	text-align: center;
	margin-left: 3vw;
	margin-right: 3vw;
	flex-direction: column;
}

.announcement h1 {
	display: flex;
	align-items: center;
	line-height: 1;
	font-weight: 100;
	font-size: 5vw;
}

.announcement img {
	height: 25vh;
	margin-left: 10px;
}

@media (max-width: 768px) {
	.announcement {
		flex-direction: column;
		margin-top: 50px;
	}

	.announcement h1 {
		font-size: 8vw;
	}

	.announcement img {
		height: 20vh;
		margin: 10px 0 0;
	}
}
</style>
<script setup lang="ts">
</script>
