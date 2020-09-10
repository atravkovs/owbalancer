<template>
  <div class="w-100">
    <div class="d-flex justify-content-end">
      <div>
        <ImportFile />
      </div>
      <div class="mr-1">
        <Export />
      </div>
      <div>
        <DeletePlayers />
      </div>
    </div>
    <div class="overflow-auto h40r bg-secondary border p-1">
      <PlayerCard class="bg-light mb-1" v-for="player in players" :player="player" :key="player" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed } from 'vue';

import { useStore } from '@/store';

import PlayerCard from '@/components/PlayerCard.vue';
import Export from '@/components/Lobby/Export.vue';
import ImportFile from '@/components/Lobby/Import.vue';
import DeletePlayers from '@/components/Lobby/DeletePlayers.vue';

export default defineComponent({
  name: 'App',
  components: { PlayerCard, Export, ImportFile, DeletePlayers },
  setup() {
    const store = useStore();

    const players = computed(() => store.state.players);

    return { players };
  },
});
</script>

<style lang="scss" scoped>
.h40r {
  height: 40rem;
}
</style>
