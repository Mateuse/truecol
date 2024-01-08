<template>
    <nav>
      <ul>
        <li>
          <router-link to="/" custom v-slot="{ navigate, isActive }">
            <a :class="{ active: isActive }" @click="navigate">Home</a>
          </router-link>
        </li>
        <li>
          <router-link to="/about" custom v-slot="{ navigate, isActive }">
            <a :class="{ active: isActive }" @click="navigate">About</a>
          </router-link>
        </li>
        <!-- Other links -->
      </ul>
    </nav>
  </template>
  
  <script>
  export default {
    name: 'NavBar'
  }
  </script>
  
  <style scoped lang="scss">
 .active {
  /* Styles for active link */
}
/* Additional styles */
  </style>
  