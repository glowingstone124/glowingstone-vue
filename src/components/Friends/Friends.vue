<template>
	<div class="apple-friends-page">
		<div class="friends-container">
			<!-- Section Header -->
			<div class="page-header">
				<span class="apple-badge">COMMUNITY &amp; NETWORK</span>
				<h1 class="page-title">友情链接</h1>
				<p class="page-desc">相聚于互联网的独立节点与博客好友（共 {{ peerList.length }} 位博主）。</p>
			</div>

			<!-- Protocol Info Bento Tile -->
			<div class="connect-bento-card">
				<div class="connect-grid">
					<!-- Left: How to connect -->
					<div class="connect-col">
						<h3 class="col-head">申请交换友链</h3>
						<p class="col-text">
							欢迎独立博主交换友链！欢迎在群内直接联系我或发送邮件，请提供以下信息：
						</p>
						<ul class="connect-list">
							<li><span class="dot">•</span> 博客网址 (Site URL)</li>
							<li><span class="dot">•</span> 博客名称 (Site Name)</li>
							<li><span class="dot">•</span> 头像或 Logo URL (Avatar)</li>
							<li><span class="dot">•</span> 简要介绍说明 (Description)</li>
						</ul>
					</div>

					<!-- Right: My Info with Copy -->
					<div class="connect-col right-col">
						<h3 class="col-head">本站节点信息</h3>
						<div class="metadata-rows">
							<div class="meta-item">
								<span class="m-key">网址:</span>
								<span class="m-val">https://glowingstone.cn</span>
							</div>
							<div class="meta-item">
								<span class="m-key">名称:</span>
								<span class="m-val">Glowstone 萤石</span>
							</div>
							<div class="meta-item">
								<span class="m-key">图标:</span>
								<span class="m-val">https://avatars.githubusercontent.com/u/163746181</span>
							</div>
							<div class="meta-item">
								<span class="m-key">介绍:</span>
								<span class="m-val">JVM痴子，全栈小白，性能优化魔怔人。</span>
							</div>
						</div>
						<button class="apple-btn-pill" @click="copyMetadata">
							{{ copyStatus ? '✓ 已复制配置信息' : '复制节点信息 JSON' }}
						</button>
					</div>
				</div>
			</div>

			<!-- Peer Nodes Grid -->
			<div class="peers-grid">
				<FriendsComponent
					v-for="peer in peerList"
					:key="peer.name"
					:avatar="peer.avatar"
					:name="peer.name"
					:description="peer.description"
					:link="peer.link"
				/>
			</div>

			<VisitCounter />
		</div>
	</div>
</template>

<script>
import VisitCounter from "@/components/VisitCounter.vue";
import FriendsComponent from './FriendsComponent.vue';

export default {
	name: 'Friends',
	components: {
		VisitCounter,
		FriendsComponent,
	},
	data() {
		return {
			copyStatus: false,
			peerList: [
				{
					name: "枫之羽翼",
					avatar: "https://avatar.maples翼.cn/avatar.png",
					description: "记录生活点滴与技术笔记的小天地。",
					link: "https://blog.maple.cn",
				},
				{
					name: "CreeperLVR",
					avatar: "https://avatars.githubusercontent.com/u/163746181",
					description: "专注于系统底层的极客友人。",
					link: "https://github.com",
				},
				{
					name: "Nyanya",
					avatar: "https://avatars.githubusercontent.com/u/163746181",
					description: "可爱且强大的前端魔法师。",
					link: "https://github.com",
				},
			],
		};
	},
	methods: {
		copyMetadata() {
			const info = {
				name: "Glowstone 萤石",
				url: "https://glowingstone.cn",
				avatar: "https://avatars.githubusercontent.com/u/163746181",
				description: "每月更新的可能性为lim -> 0，jvm痴子，全栈小白，性能优化魔怔人。",
			};
			navigator.clipboard.writeText(JSON.stringify(info, null, 2)).then(() => {
				this.copyStatus = true;
				setTimeout(() => {
					this.copyStatus = false;
				}, 2000);
			});
		},
	},
};
</script>

<style scoped>
.apple-friends-page {
	width: 100%;
	padding: 40px 0 80px;
}

.friends-container {
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

/* Connect Card */
.connect-bento-card {
	background: #ffffff;
	border: 1px solid rgba(0, 0, 0, 0.08);
	border-radius: var(--apple-radius-card, 24px);
	padding: 36px 40px;
	box-shadow: 0 4px 20px rgba(0, 0, 0, 0.04);
	margin-bottom: 40px;
}

.connect-grid {
	display: grid;
	grid-template-columns: 1fr 1fr;
	gap: 48px;
}

.col-head {
	font-size: 18px;
	font-weight: 700;
	color: #1d1d1f;
	margin: 0 0 12px;
}

.col-text {
	font-size: 14px;
	line-height: 1.6;
	color: #6e6e73;
	margin: 0 0 16px;
}

.connect-list {
	list-style: none;
	padding: 0;
	margin: 0;
	display: flex;
	flex-direction: column;
	gap: 8px;
	font-size: 14px;
	color: #333336;
}

.dot {
	color: #0071e3;
	margin-right: 6px;
}

.metadata-rows {
	display: flex;
	flex-direction: column;
	gap: 10px;
	margin-bottom: 20px;
	background: #f5f5f7;
	padding: 16px 20px;
	border-radius: 16px;
	font-size: 13px;
}

.meta-item {
	display: flex;
	gap: 8px;
}

.m-key {
	color: #86868b;
	width: 50px;
}

.m-val {
	color: #1d1d1f;
	overflow: hidden;
	text-overflow: ellipsis;
	white-space: nowrap;
}

/* Peers Grid */
.peers-grid {
	display: grid;
	grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
	gap: 24px;
}

@media screen and (max-width: 768px) {
	.connect-grid {
		grid-template-columns: 1fr;
		gap: 32px;
	}
	.peers-grid {
		grid-template-columns: 1fr;
	}
}
</style>
