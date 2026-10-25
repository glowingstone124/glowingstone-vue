<template>
	<div class="peer-apple-card" @click="visitProfile">
		<div class="peer-content">
			<div class="avatar-wrap">
				<img :src="avatar" :alt="name" class="peer-avatar" loading="lazy" />
			</div>
			<div class="peer-info">
				<div class="peer-header">
					<h3 class="peer-name">{{ name }}</h3>
					<span class="uplink-arrow">&rarr;</span>
				</div>
				<p class="peer-desc">{{ description || '暂无描述信息。' }}</p>
				<span class="peer-url">{{ displayLink }}</span>
			</div>
		</div>
	</div>
</template>

<script>
export default {
	name: 'FriendsComponent',
	props: {
		avatar: { type: String, default: '' },
		name: { type: String, default: '' },
		description: { type: String, default: '' },
		link: { type: String, default: '' },
	},
	computed: {
		displayLink() {
			if (!this.link) return '';
			try {
				const url = new URL(this.link);
				return url.hostname;
			} catch (e) {
				return this.link;
			}
		},
	},
	methods: {
		visitProfile() {
			if (this.link) {
				window.open(this.link, '_blank');
			}
		},
	},
};
</script>

<style scoped>
.peer-apple-card {
	background: #ffffff;
	border: 1px solid rgba(0, 0, 0, 0.08);
	border-radius: var(--apple-radius-card, 24px);
	padding: 24px 28px;
	cursor: pointer;
	overflow: hidden;
	box-shadow: 0 4px 18px rgba(0, 0, 0, 0.04);
	transition: transform 0.25s ease, box-shadow 0.25s ease, border-color 0.25s ease;
}

.peer-apple-card:hover {
	transform: translateY(-4px);
	box-shadow: 0 14px 32px rgba(0, 0, 0, 0.08);
	border-color: rgba(0, 0, 0, 0.14);
}

.peer-content {
	display: flex;
	align-items: center;
	gap: 20px;
}

.avatar-wrap {
	width: 60px;
	height: 60px;
	border-radius: 50%;
	overflow: hidden;
	flex-shrink: 0;
	background: #f5f5f7;
	border: 2px solid #e5e5ea;
}

.peer-avatar {
	width: 100%;
	height: 100%;
	object-fit: cover;
	display: block;
}

.peer-info {
	display: flex;
	flex-direction: column;
	flex: 1;
	min-width: 0;
}

.peer-header {
	display: flex;
	align-items: center;
	justify-content: space-between;
	margin-bottom: 6px;
}

.peer-name {
	font-size: 17px;
	font-weight: 700;
	color: #1d1d1f;
	margin: 0;
	overflow: hidden;
	text-overflow: ellipsis;
	white-space: nowrap;
}

.uplink-arrow {
	color: #86868b;
	transition: transform 0.2s ease, color 0.2s ease;
}

.peer-apple-card:hover .uplink-arrow {
	color: #0071e3;
	transform: translateX(4px);
}

.peer-desc {
	font-size: 13px;
	color: #6e6e73;
	line-height: 1.5;
	margin: 0 0 8px;
	display: -webkit-box;
	-webkit-line-clamp: 2;
	-webkit-box-orient: vertical;
	overflow: hidden;
}

.peer-url {
	font-size: 12px;
	color: #0071e3;
	font-family: var(--apple-font-mono, monospace);
}
</style>
