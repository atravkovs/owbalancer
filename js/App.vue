<template>
  <div class="container-fluid px-0">
    <div class="d-flex justify-content-between w-100 py-2 mb-1 px-2 mx-auto">
      <div class="d-flex">
        <OWIcon />
        <h3 class="ml-2">Tournament Balancer</h3>
      </div>
      <span>
        Created by
        <a target="blank" href="https://github.com/atravkovs">s0ck3t</a>
      </span>
    </div>
    <div class="row px-2 mx-auto">
      <div class="w-25r">
        <Lobby />
      </div>
      <div class="col">
        <Teams />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import '@/styles/main.scss';
import { defineComponent } from 'vue';

import Lobby from '@/components/Lobby.vue';
import Teams from '@/components/Teams.vue';
import OWIcon from '@/components/svg/OWIcon.vue';

export default defineComponent({
  name: 'App',
  components: { Lobby, Teams, OWIcon },
});
</script>

<style lang="scss" scoped>
.w-25r {
  width: 25rem !important;
}
img {
  width: 24rem;
}
</style>
