<template>
  <div class="w-100">
    <div class="d-flex justify-content-between">
      <div>
        <sort @sort="sort" />
      </div>
      <div>
        <assign />
      </div>
      <div class="d-flex">
        <export />
        <import-file />
      </div>
      <div>
        <delete-players />
      </div>
    </div>
    <stats :players="state.storePlayers" :currentCount="state.players.length" />
    <player-filter :players="state.storePlayers" @filter="filter" />
    <div class="overflow-auto h40r bg-secondary border p-1" @dragover="allowDrop" @drop="drop">
      <player-card
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
import MutationTypes from '@/store/mutation-types';

import Sort from '@/components/Lobby/Sort.vue';
import Stats from '@/components/Lobby/Stats.vue';
import PlayerFilter from '@/components/Lobby/Filter.vue';
import Assign from '@/components/Lobby/Assign.vue';
import PlayerCard from '@/components/PlayerCard.vue';
import Export from '@/components/Lobby/Export.vue';
import ImportFile from '@/components/Lobby/Import.vue';
import DeletePlayers from '@/components/Lobby/DeletePlayers.vue';
import PObj, { Player, PlayerEntries } from '@/objects/player';

export default defineComponent({
  name: 'App',
  components: { Assign, Sort, Stats, PlayerFilter, PlayerCard, Export, ImportFile, DeletePlayers },
  setup() {
    const store = useStore();
    const teamsLen = computed(() => store.state.teams.length);
    const storePlayers = computed(() =>
      Object.entries(store.state.players).filter(([, p]) =>
        store.state.reservedPlayers.length > 0
          ? store.state.reservedPlayers.includes(p.identity.uuid)
          : teamsLen.value <= 0
      )
    );
    const activeSort: { rule: string; order: 'asc' | 'desc' } = {
      rule: 'name',
      order: 'asc',
    };

    const state = reactive({
      activeSort,
      storePlayers,
      players: storePlayers.value,
    });

    const sort = (rule: string, order: 'asc' | 'desc', pl?: [string, Player][]) => {
      const mPlayers = pl || state.players;

      const sortedPlayers = orderBy(
        mPlayers,
        [
          ([, p]) => {
            if (rule === 'name') return p.identity.name.toLowerCase();
            if (rule === 'sr') return PObj.getTopRank(p);

            return new Date(p.createdAt).getTime();
          },
        ],
        order
      );

      state.activeSort.rule = rule;
      state.activeSort.order = order;
      state.players = sortedPlayers;
    };

    watch(
      () => state.storePlayers,
      () => {
        sort(state.activeSort.rule, state.activeSort.order, storePlayers.value);
      }
    );

    const filter = (filtered: PlayerEntries) => {
      sort(state.activeSort.rule, state.activeSort.order, filtered);
    };

    onMounted(() => {
      sort(state.activeSort.rule, state.activeSort.order);
    });

    const allowDrop = (ev: DragEvent) => {
      ev.preventDefault();
    };
    const drop = (ev: DragEvent) => {
      ev.preventDefault();
      const playerId = ev?.dataTransfer?.getData('playerTag');
      const teamUuid = ev?.dataTransfer?.getData('team');

      if (playerId === undefined || teamUuid === undefined) return;

      if (store.state.teams.length > 0) {
        store.commit(MutationTypes.ADD_RESERVE, playerId);
        store.commit(MutationTypes.REMOVE_FROM_TEAM, {
          teamUuid,
          playerId,
        });
      }
    };

    return { state, sort, filter, allowDrop, drop };
  },
});
</script>

<style lang="scss" scoped>
.h40r {
  height: 40rem;
}
</style>
