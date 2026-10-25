<template>
	<footer class="apple-footer">
		<div class="footer-container">
			<div class="footer-directory">
				<div class="directory-col">
					<h4 class="col-title">Newsroom 栏目</h4>
					<ul class="col-links">
						<li><router-link to="/">Latest News</router-link></li>
						<li><router-link to="/archive?category=技术">技术架构</router-link></li>
						<li><router-link to="/archive?category=闲聊">个人随笔</router-link></li>
						<li><router-link to="/archive?category=时事">最新动态</router-link></li>
					</ul>
				</div>
				<div class="directory-col">
					<h4 class="col-title">探索与探索</h4>
					<ul class="col-links">
						<li><router-link to="/gallery">画廊特辑</router-link></li>
						<li><router-link to="/friends">友情链接</router-link></li>
						<li><router-link to="/about">关于萤石</router-link></li>
						<li><router-link to="/announcement">特别声明</router-link></li>
					</ul>
				</div>
				<div class="directory-col">
					<h4 class="col-title">工程与开源</h4>
					<ul class="col-links">
						<li><a href="https://github.com/glowingstone124" target="_blank" rel="noopener">GitHub 个人主页</a></li>
						<li><a href="https://github.com/glowingstone124/glowingstone-vue" target="_blank" rel="noopener">博客仓库源码</a></li>
						<li><a href="https://bucket.glowingstone.cn" target="_blank" rel="noopener">静态资源存储桶</a></li>
					</ul>
				</div>
			</div>

			<div class="footer-bottom">
				<div class="copyright-line">
					Copyright &copy; 2026 glowingstone. All rights reserved.
				</div>
				<div class="legal-links">
					<router-link to="/about">隐私政策</router-link>
					<span class="sep">|</span>
					<router-link to="/about">使用条款</router-link>
					<span class="sep">|</span>
					<router-link to="/archive">文章归档</router-link>
					<span class="sep">|</span>
					<a href="https://github.com/glowingstone124" target="_blank" rel="noopener">反馈交流</a>
				</div>
			</div>
		</div>
	</footer>
</template>

<script>
export default {
	name: 'Footer',
	props: {
		Footermsg: {
			type: String,
			default: 'Apple Newsroom Style · glowingstone blog',
		},
	},
};
</script>

<style scoped>
.apple-footer {
	width: 100%;
	background: #f5f5f7;
	border-top: 1px solid #d2d2d7;
	color: #6e6e73;
	font-size: 12px;
	padding: 40px 0 32px;
	margin-top: auto;
}

.footer-container {
	max-width: var(--apple-max-width, 1240px);
	margin: 0 auto;
	padding: 0 24px;
	box-sizing: border-box;
}

.footer-directory {
	display: grid;
	grid-template-columns: repeat(4, 1fr);
	gap: 32px;
	padding-bottom: 32px;
	border-bottom: 1px solid #d2d2d7;
	margin-bottom: 20px;
}

.col-title {
	font-size: 12px;
	font-weight: 600;
	color: #1d1d1f;
	margin: 0 0 12px;
}

.col-links {
	list-style: none;
	padding: 0;
	margin: 0;
	display: flex;
	flex-direction: column;
	gap: 8px;
}

.col-links a {
	color: #6e6e73;
	text-decoration: none;
	transition: color 0.15s ease;
}

.col-links a:hover {
	color: #1d1d1f;
	text-decoration: underline;
}

.design-note {
	line-height: 1.6;
	color: #86868b;
	margin: 0;
}

.footer-bottom {
	display: flex;
	justify-content: space-between;
	align-items: center;
	flex-wrap: wrap;
	gap: 12px;
	color: #86868b;
}

.legal-links {
	display: flex;
	align-items: center;
	gap: 8px;
}

.legal-links a {
	color: #6e6e73;
	transition: color 0.15s ease;
}

.legal-links a:hover {
	color: #1d1d1f;
}

.sep {
	color: #d2d2d7;
}

@media screen and (max-width: 768px) {
	.footer-directory {
		grid-template-columns: 1fr 1fr;
		gap: 24px;
	}
	.footer-bottom {
		flex-direction: column;
		align-items: flex-start;
	}
}

@media screen and (max-width: 480px) {
	.footer-directory {
		grid-template-columns: 1fr;
	}
}
</style>
