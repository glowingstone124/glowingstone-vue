<template>
	<div class="contents">
		<span style="width: 70%">
		<h1 class="cagetory">Friends</h1>
		<h2>关于添加友链：<br>如果在群里抓到我，可以直接@我并且告诉我你的以下信息:<br>
			博客网址<br>
			博客名字<br>
			博客logo<br>
			博客介绍<br>
			也可以直接通过“关于”中的邮箱联系我。</h2>
		<h2>我的信息：<br>
			博客网址: glowingstone.cn<br>
			博客名字: Glowstone 萤石<br>
			博客logo: https://avatars.githubusercontent.com/u/163746181 (若无法访问可以自己缓存，仅9k大小)<br>
			博客介绍: 每月更新的可能性为lim -> 0，jvm痴子，全栈小白，性能优化魔怔人。<br>
			也可以直接通过“关于”中的邮箱联系我。</h2>
		</span>
		<div class="friends">
			<FriendsComponent
				:avatar="'https://minecreeper.top/images/avatar.png'"
				:name="'矿井小怕的茶馆'"
				:description="'也许是自己的后院，但更是大家的茶馆。'"
				:link="'https://minecreeper.top/'"
			/>
			<FriendsComponent
				:avatar="'https://gravatar.loli.net/avatar/0bf16c751d119f1fb5e76d2bdef47fd4'"
				:name="'社会易姐QwQ'"
				:description="'我的秒开'"
				:link="'https://shakaianee.top/'"
			/>
			<FriendsComponent
				:avatar="'https://serinanya.cn/assets-zone-1/avatar-round.png'"
				:name="'SerinaNya'"
				:description="'Serina 的日常 / daily of SerinaNya'"
				:link="'https://serinanya.cn'"
			/>
			<FriendsComponent
				:avatar="'https://q2.qlogo.cn/headimg_dl?dst_uin=2504726488&spec=100'"
				:name="'凌空的博客小站'"
				:description="''"
				:link="'https://www.skyblogs.xyz/'"
			/>
			<FriendsComponent
				:avatar="'https://blog.ibctech.top/usr/themes/handsome/assets/img/tx.jpg'"
				:name="'NLのBlog'"
				:description="''"
				:link="'https://blog.ibctech.top'"
			/>
			<FriendsComponent
				:avatar="'https://picture.zhanganzhi.com/common/avatar.jpg'"
				:name="'Andy\'s Blog'"
				:description="''"
				:link="'https://blog.zhanganzhi.com/'"
			/>
			<FriendsComponent
				:avatar="'https://static.xhustudio.eu.org/images/jpeg/102411014.jpg'"
				:name="'SЕSSのB10GТЕ5Т'"
				:description="'Session的个人博客, 这里有各种类型的有趣的文章内容, 网站基于纯前端构'"
				:link="'https://sess.xhustudio.eu.org/'"
			></FriendsComponent>
		</div>
	</div>
</template>
<script>
export default {
	name: 'Friends',
	components: {
		FriendsComponent
	}
}
import FriendsComponent from './FriendsComponent.vue';

</script>

<style scoped>
@import "@/css/main.css";

.contents {
	margin-top: 12vh;
	display: flex;
	flex-direction: column;
	align-items: center;
}
h2 {
	font-weight: 200;
	font-size: 1rem;
	margin: 1vh 1vh 2vh;
	max-width: 65vw;
	border-radius: 10px;
	background: #05cb86;
	padding: 4vh 6vh;
}
.cagetory {
	font-size: 6em;
	font-weight: 300;}

.friends {
	margin:auto
}
</style>
