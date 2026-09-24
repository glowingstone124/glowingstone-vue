<template>
	<footer class="metro-footer font-mono">
		<div class="footer-container">
			<span>{{ Footermsg }}</span>
			<span class="footer-sub">&copy; 2026 glowingstone. All rights reserved.</span>
		</div>
	</footer>
</template>

<script>
export default {
	name: 'Footer',
	props: {
		Footermsg: {
			type: String,
			default: 'design & code by glowingstone124',
		},
	},
};
</script>

<style scoped>
.metro-footer {
	width: 100%;
	border-top: 1px solid #262626;
	background: #0a0a0a;
	padding: 28px 0;
	color: #737373;
	font-size: 0.82rem;
}

.footer-container {
	max-width: var(--max-content-width);
	margin: 0 auto;
	padding: 0 32px;
	display: flex;
	justify-content: space-between;
	align-items: center;
	flex-wrap: wrap;
	gap: 12px;
}

.footer-sub {
	color: #525252;
}

@media screen and (max-width: 640px) {
	.footer-container {
		flex-direction: column;
		text-align: center;
	}
}
</style>