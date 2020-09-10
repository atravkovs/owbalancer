<template>
  <div>
    <div class="row mw-25r">
      <div class="col-sm-8">
        <ImportFile />
      </div>
      <div class="col-4 col-sm-2">
        <Export />
      </div>
      <div class="col-4 col-sm-2">
        <DeletePlayers />
      </div>
    </div>
    <div class="overflow-auto mw-25r h40r bg-secondary border p-1">
      <PlayerCard class="bg-light mb-1" v-for="player in players" :player="player" :key="player" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed } from 'vue';

import { useStore } from '@/store';

import PlayerCard from '@/components/PlayerCard.vue';
import Export from '@/components/Export.vue';
import ImportFile from '@/components/Import.vue';
import DeletePlayers from '@/components/DeletePlayers.vue';

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

.mw-25r {
  max-width: 25rem;
}
</style>
