<template>
	<div @click="visitProfile" :class="['friend-container', { 'deactive': !isLinkActive }]">
		<div class="avatar-container">
			<img :src="avatar" alt="Avatar">
		</div>
		<div class="info-container">
			<h2>{{ name }}</h2>
			<p>{{ description }}</p>
		</div>
	</div>
</template>

<script>
export default {
	name: 'FriendsComponent',
	props: {
		avatar: String,
		name: String,
		description: String,
		link: String
	},
	data() {
		return {
			isLinkActive: true
		}
	},
	mounted() {
		this.checkLink();
	},
	methods: {
		visitProfile() {
			if (this.isLinkActive && this.link) {
				window.location.href = this.link;
			}
		},
		async checkLink() {
		}
	}
}
</script>

<style scoped>
@import "@/css/main.css";

p {
	background: none;
	color: #a9a7a7;
}
.friend-container {
	display: flex;
	align-items: center;
	cursor: pointer;
	background-color: #475465;
	padding: 4vh;
	margin: 1vh 1vh 8vh;
	width: 65vw;
	transition: background-color 0.1s;
	border-radius: 15px;
}

.friend-container:hover {
	background-color: #5f748f;
}
.friend-container.deactive {
	background-color: #3d3e41;
}
.avatar-container {
	margin-right: 40px;
}

img {
	max-width: 110px;
	min-width: 110px;
	border-radius: 20%;
}

.info-container {
	flex-grow: 1;
}

h2 {
	border-bottom: 1px solid rgb(172, 191, 239);
}
</style>
