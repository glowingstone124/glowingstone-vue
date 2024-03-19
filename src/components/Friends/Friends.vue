<template>
  <div class="contents">
    <h1 class="cagetory">Friends</h1>
    <div class="friends">
      <FriendsComponent
        :avatar="'https://minecreeper.top/images/avatar.png'"
        :name="'矿井小怕的茶馆'"
        :description="''"
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
        :link="'serinanya.cn'"
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
  .contents{
    margin-top: 6vh;
  }
  .cagetory{
    font-size: 5em;
    font-weight: 500;
  }
  .friends{
    margin-top: 3vh;
  }
  </style>
  