<template>
  <div class="w-100">
    <div class="d-flex justify-content-between">
      <div>
        <Sort v-on:sort="sort" />
      </div>
      <div>
        <Assign />
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
    <div class="row">
      <div class="col-3">Total: {{ state.storePlayers.length }}</div>
      <div class="col-auto">Showing: {{ state.players.length }}</div>
    </div>
    <div class="row">
      <label for="playerFilter" class="col-2 col-form-label">Filter:</label>
      <div class="col-10">
        <input id="playerFilter" type="text" class="form-control form-control-sm" @input="filter" />
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
import { computed, defineComponent, onMounted, reactive, watch } from 'vue';
import orderBy from 'lodash/orderBy';

import { useStore } from '@/store';

import Sort from '@/components/Lobby/Sort.vue';
import Assign from '@/components/Lobby/Assign.vue';
import PlayerCard from '@/components/PlayerCard.vue';
import Export from '@/components/Lobby/Export.vue';
import ImportFile from '@/components/Lobby/Import.vue';
import DeletePlayers from '@/components/Lobby/DeletePlayers.vue';
import { Player } from '@/objects/player';

export default defineComponent({
  name: 'App',
  components: { Assign, Sort, PlayerCard, Export, ImportFile, DeletePlayers },
  setup() {
    const store = useStore();
    const storePlayers = computed(() => Object.entries(store.state.players));
    const activeSort: { rule: string; order: 'asc' | 'desc' } = {
      rule: 'name',
      order: 'asc',
    };

    const state = reactive({
      activeSort,
      storePlayers,
      players: storePlayers.value,
    });

    watch(
      () => state.storePlayers,
      () => {
        state.players = storePlayers.value;
      }
    );

    const sort = (
      rule: string,
      order: 'asc' | 'desc',
      pl?: [string, Player][]
    ) => {
      const mPlayers = pl || state.players;

      const sortedPlayers = orderBy(
        mPlayers,
        [
          ([, p]) => {
            if (rule === 'name') return p.identity.name.toLowerCase();
            if (rule === 'sr')
              return Object.values(p.stats.classes)
                .filter((role) => role.isActive)
                .sort((a, b) => a.priority - b.priority)[0].rank;
            return p.createdAt;
          },
        ],
        order
      );

      state.activeSort.rule = rule;
      state.activeSort.order = order;
      state.players = sortedPlayers;
    };

    const filter = (e: Event) => {
      const filterValue = (e.target as HTMLInputElement).value.toLowerCase();
      const filtered = state.storePlayers.filter(([, p]) =>
        p.identity.name.toLowerCase().startsWith(filterValue)
      );
      sort(state.activeSort.rule, state.activeSort.order, filtered);
    };

    onMounted(() => {
      sort(state.activeSort.rule, state.activeSort.order);
    });

    return { state, sort, filter };
  },
});
</script>

<style lang="scss" scoped>
.h40r {
  height: 40rem;
}
</style>
