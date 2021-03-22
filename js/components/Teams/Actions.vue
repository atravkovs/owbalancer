<template>
  <div class="row g-1 flex-auto">
    <div class="col-auto">
      <button class="btn btn-sm btn-secondary" @click="addNew">New</button>
      <button class="btn btn-sm btn-primary mx-1" @click="balance">
        Balance
      </button>
      <button class="btn btn-sm btn-danger mr-1" @click="clear">Clear</button>
    </div>
    <div class="col-auto">
      <button
        class="btn btn-sm btn-warning rounded-right-0 text-nowrap"
        v-if="canChange"
        @click="select"
      >
        Choose balance
      </button>
      <button
        class="btn btn-sm btn-outline-warning rounded-left-0 mr-1"
        v-if="canChange"
        @click="clearSelect"
      >
        <bin-icon />
      </button>
      <button class="btn btn-sm btn-secondary mr-1" @click="empty">
        Empty
      </button>
    </div>
    <div class="col-auto d-flex">
      <export-teams />
      <button class="btn btn-sm btn-primary mx-1" @click="archive"><archive-icon /></button>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent } from 'vue';

import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';

import TObj from '@/objects/team';

import BinIcon from '@/components/svg/BinIcon.vue';
import ArchiveIcon from '@/components/svg/ArchiveIcon.vue';
import ExportTeams from '@/components/Teams/ExportTeams.vue';

export default defineComponent({
  name: 'Actions',
  components: { BinIcon, ExportTeams, ArchiveIcon },
  setup() {
    const store = useStore();

    const canChange = computed(() => store.state.balancerResults.length > 1);

    const balance = () => {
      store.commit(MutationTypes.TOGGLE_BALANCE, undefined);
    };

    const archive = () => {
      store.commit(MutationTypes.TOGGLE_ARCHIVE, undefined);
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

    const clearSelect = () => {
      store.commit(MutationTypes.CLEAR_TEAMS, undefined);
      store.commit(MutationTypes.SET_RESULTS, []);
    };

    return {
      empty,
      clear,
      select,
      addNew,
      archive,
      balance,
      canChange,
      clearSelect,
    };
  },
});
</script>

<style scoped>
.rounded-left-0 {
  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
}

.rounded-right-0 {
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
}
.flex-auto {
  flex: auto;
}
</style>
