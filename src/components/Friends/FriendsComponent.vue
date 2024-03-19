<template>
    <div @click="visitProfile" class="friend-container">
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
  methods: {
    visitProfile() {
      if (this.link) {
        window.location.href = this.link;
      }
    }
  }
}
</script>

<style scoped>
@import "@/css/main.css";
.friend-container {
  display: flex;
  align-items: center;
  cursor: pointer;
  margin: 1vh;
  background-color: #3d4b5c;
  padding: 4vh;
  max-width:65vw;
  transition: background-color 0.1s;
  border-radius: 25px;
  box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;
}
.friend-container:hover {
    background-color: #5f748f;
}

.avatar-container {
  margin-right: 40px;
}

img {
  max-width: 110px;
  border-radius: 50%;
}

.info-container {
  flex-grow: 1;
}
</style>

