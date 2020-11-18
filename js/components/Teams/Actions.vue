<template>
  <div class="d-flex">
    <button class="btn btn-sm btn-secondary" @click="addNew">New</button>
    <button class="btn btn-sm btn-primary ml-2" @click="balance">
      Balance
    </button>
    <button class="btn btn-sm btn-danger mx-2" @click="clear">Clear</button>
    <button class="btn btn-sm btn-warning mx-2" v-if="canChange" @click="select">
      Choose balance
    </button>
    <button class="btn btn-sm btn-secondary mx-2" @click="empty">
      Empty
    </button>
    <export-teams />
  </div>
</template>

<script lang="ts">
import { computed, defineComponent } from 'vue';

import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';

import TObj from '@/objects/team';

import ExportTeams from '@/components/Teams/ExportTeams.vue';

export default defineComponent({
  name: 'Actions',
  components: { ExportTeams },
  setup() {
    const store = useStore();

    const canChange = computed(() => store.state.balancerResults.length > 1);

    const balance = () => {
      store.commit(MutationTypes.TOGGLE_BALANCE, undefined);
    };

    const clear = () => {
      store.commit(MutationTypes.CLEAR_TEAMS, undefined);
    };

    const addNew = () => {
      const newTeam = TObj.createEmptyTeam();

      if (store.state.teams.length <= 0) {
        const playerIds = Object.keys(store.state.players);
        store.commit(MutationTypes.RESERVE_PLAYERS, playerIds);
      }

      store.commit(MutationTypes.ADD_TEAM, newTeam);
    };

    const empty = () => {
      store.commit(MutationTypes.EMPTY_TEAMS, undefined);
    };

    const select = () => {
      store.commit(MutationTypes.TOGGLE_SELECTION, undefined);
    };

    return {
      empty,
      clear,
      select,
      addNew,
      balance,
      canChange,
    };
  },
});
</script>
