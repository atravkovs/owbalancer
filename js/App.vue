<template>
  <div class="container-fluid">
    <h2>Overwatch Balancer :)</h2>
    <div class="row">
      <div class="col">
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

export default defineComponent({
  name: 'App',
  components: { Lobby, Teams },
});
</script>
