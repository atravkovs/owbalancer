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
        v-for="[battleTag, player] in state.players"
        :player="player"
        :key="battleTag"
      />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, reactive } from 'vue';
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

    const players = Object.entries(store.state.players);
    const state = reactive({
      players,
    });

    const sort = (rule: string, order: 'asc' | 'desc') => {
      const sortedPlayers = orderBy(
        state.players,
        [
          ([, p]) => {
            if (rule === 'name') return p.identity.displayName;
            if (rule === 'sr') {
              return Object.values(p.stats.classes).reduce((acc, value) => {
                return !value.isActive ? acc : acc + value.rating;
              }, 0);
            }

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
