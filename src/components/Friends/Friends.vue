<template>
  <div class="contents">
    <h1 class="cagetory">Friends</h1>
    <div class="friends">
      <FriendsComponent
        :avatar="'https://avatars.githubusercontent.com/u/80297629?v=4'"
        :name="'glowingstone124'"
        :description="'Java & web Dev java和web开发者'"
        :link="'https://github.com'"
      />
      <FriendsComponent
        :avatar="'https://gravatar.loli.net/avatar/0bf16c751d119f1fb5e76d2bdef47fd4'"
        :name="'社会易姐QwQ'"
        :description="''"
        :link="'https://shakaianee.top/'"
      />
      <FriendsComponent
        :avatar="'https://avatars.githubusercontent.com/u/80297629?v=4'"
        :name="'glowingstone124'"
        :description="'Java & web Dev java和web开发者'"
        :link="'https://github.com'"
      />
      <FriendsComponent
        :avatar="'https://avatars.githubusercontent.com/u/80297629?v=4'"
        :name="'glowingstone124'"
        :description="'Java & web Dev java和web开发者'"
        :link="'https://github.com'"
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
  