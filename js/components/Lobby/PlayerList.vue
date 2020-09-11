<template>
  <div class="w-100">
    <div class="d-flex justify-content-between">
      <div>
        <Sort v-on:sort="sort" />
      </div>
      <div>
        <ImportFile />
      </div>
      <div>
        <Export />
      </div>
      <div>
        <DeletePlayers />
      </div>
    </div>
    <div class="overflow-auto h40r bg-secondary border p-1">
      <PlayerCard
        class="bg-light mb-1"
        v-for="[uuid, player] in state.players"
        :player="player"
        :key="uuid"
      />
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, reactive, watch } from 'vue';
import orderBy from 'lodash/orderBy';

import { useStore } from '@/store';

import Sort from '@/components/Lobby/Sort.vue';
import PlayerCard from '@/components/PlayerCard.vue';
import Export from '@/components/Lobby/Export.vue';
import ImportFile from '@/components/Lobby/Import.vue';
import DeletePlayers from '@/components/Lobby/DeletePlayers.vue';

export default defineComponent({
  name: 'App',
  components: { Sort, PlayerCard, Export, ImportFile, DeletePlayers },
  setup() {
    const store = useStore();
    const storePlayers = computed(() => Object.entries(store.state.players));
    const state = reactive({
      storePlayers,
      players: storePlayers.value,
    });

    watch(
      () => state.storePlayers,
      () => {
        state.players = storePlayers.value;
      }
    );

    const sort = (rule: string, order: 'asc' | 'desc') => {
      const sortedPlayers = orderBy(
        storePlayers.value,
        [
          ([, p]) => {
            if (rule === 'name') return p.identity.name.toLowerCase();
            if (rule === 'sr') return p.stats.rank;
            return p.createdAt;
          },
        ],
        order
      );

      state.players = sortedPlayers;
    };

    return { state, sort };
  },
});
</script>

<style lang="scss" scoped>
.h40r {
  height: 40rem;
}
</style>
