<template>
    <div class="navigator">
        <ul id="navigation" :class="{ 'display': navStat }">
            <li><a href="/"><img src="@/assets/home.svg"><span>主页</span></a></li>
            <li><a href="/friends"><img src="@/assets/friends.svg"><span>友链</span></a></li>
            <li><a href="/about"><img src="@/assets/about.svg"><span>关于</span></a></li>
        </ul>
    </div>
</template>
<script>
  export default {
    name: 'Nav'
  }
  </script>
  
  <style scoped>
  @import "@/css/main.css";
  li a{
    display: flex;
    flex-direction: row;
    column-gap: 0.5em;
  }
  </style>