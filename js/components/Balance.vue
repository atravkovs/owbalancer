<template>
  <modal
    title="Balance"
    variant="large"
    :isActive="isActive"
    customAction="Balance"
    @close-modal="closeModal"
    @save-changes="balance"
  >
    <div class="position-relative w-100">
      <sync />
    </div>
    <balance-type v-model="balanceType" />
    <balance-disable v-model="disableType" v-if="balanceType === 'full'" />
    <balance-options />
    <adjust-rating />
    <dispersion />
    <tries-count />
    <balancer-progress :progress="progress" />
  </modal>
</template>

<script lang="ts">
/* eslint-disable */
import { computed, defineComponent, reactive, ref } from 'vue';
import MutationTypes from '@/store/mutation-types';
import { useStore } from '@/store';
import wasm from '@/mworker';

import player from '@/objects/player';
import { Teams } from '@/objects/team';
import { Results } from '@/objects/balance';

import Sync from '@/components/Balance/Sync.vue';
import Modal from '@/components/Helpers/Modal.vue';
import TriesCount from '@/components/Balance/TriesCount.vue';
import Dispersion from '@/components/Balance/Dispersion.vue';
import BalanceType from '@/components/Balance/BalanceType.vue';
import AdjustRating from '@/components/Balance/AdjustRating.vue';
import BalanceOptions from '@/components/Balance/BalanceOptions.vue';
import BalanceDisable from '@/components/Balance/BalanceDisable.vue';
import BalancerProgress from '@/components/Balance/BalancerProgress.vue';

type DataType = {
  teamsCopy: Teams;
  reserveCopy: string[];
};

export default defineComponent({
  name: 'Balance',
  components: {
    Sync,
    Modal,
    TriesCount,
    Dispersion,
    BalanceType,
    AdjustRating,
    BalanceOptions,
    BalanceDisable,
    BalancerProgress,
  },
  setup() {
    const store = useStore();

    const sbOptions = computed(() => store.state.balancerOptions);

    const balanceType = ref('full');
    const disableType = ref('none');

    const isActive = computed(() => store.state.isBalance);
    const reservedPlayers = computed(() => store.state.reservedPlayers);
    const stateTeams = computed(() => store.state.teams);
    const progress = reactive({ total: 10, current: 0 });

    document.addEventListener('wasm-update', e => {
      // eslint-disable-next-line
      console.log('Step: ', (e as any).detail.message());
      progress.current += 1;
    });

    const closeModal = () => {
      store.commit(MutationTypes.TOGGLE_BALANCE, undefined);
    };

    const fullBalance: (lib: any) => Results = lib => {
      const data = JSON.stringify({
        players: store.state.players,
        range: +sbOptions.value.range,
        lowRankLimiter: sbOptions.value.lowRankLimiter,
        disallowSecondaryRoles: sbOptions.value.disallowSecondaryRoles,
        adjustSr: sbOptions.value.adjustSr,
        disableType: disableType.value,
        dispersionMinimizer: sbOptions.value.dispersionMinimizer,
        triesCount: sbOptions.value.triesCount,
      });

      return lib.fullBalance(data);
    };

    const halfBalance: (lib: any) => Results = lib => {
      return lib.halfBalance(
        JSON.stringify({
          players: store.state.players,
          range: +sbOptions.value.range,
          lowRankLimiter: sbOptions.value.lowRankLimiter,
          disallowSecondaryRoles: sbOptions.value.disallowSecondaryRoles,
          adjustSr: sbOptions.value.adjustSr,
        })
      );
    };

    const finalBalance: (lib: any, data: DataType) => Results = (
      lib,
      { teamsCopy, reserveCopy }
    ) => {
      return lib.finalBalance(
        JSON.stringify({
          players: store.state.players,
          range: +sbOptions.value.range,
          lowRankLimiter: sbOptions.value.lowRankLimiter,
          disallowSecondaryRoles: sbOptions.value.disallowSecondaryRoles,
          reserveCopy: reserveCopy,
          teamsCopy: teamsCopy,
          adjustSr: sbOptions.value.adjustSr,
        })
      );
    };

    const conditionalBalance: (lib: any, data: DataType) => Results = (lib, data) => {
      if (balanceType.value === 'half') {
        return halfBalance(lib);
      }

      if (balanceType.value === 'final') {
        return finalBalance(lib, data);
      }

      return fullBalance(lib);
    };

    const checkCaps: () => boolean = () => {
      return Object.values(store.state.players).some(player => player.identity.isCaptain);
    };

    const checkSquires: () => boolean = () => {
      const caps = Object.values(store.state.players).filter(player => player.identity.isCaptain);
      const squires = Object.values(store.state.players).filter(player => player.identity.isSquire);

      return squires.length >= caps.length;
    };

    const balance = async () => {
      if (!checkCaps()) {
        alert('Please select at least one captain');
        return;
      }

      if (!checkSquires()) {
        alert('Please make sure that every captain has at least one squire');
        return;
      }

      const teamsCopy = [...stateTeams.value];
      const reserveCopy = [...reservedPlayers.value];

      store.commit(MutationTypes.CLEAR_TEAMS, undefined);
      store.commit(MutationTypes.SET_RESULTS, []);
      progress.current = 0;

      const lib = await wasm;

      try {
        let results: Results | null = await conditionalBalance(lib, {
          teamsCopy,
          reserveCopy,
        });

        if (results.length != 1) {
          store.commit(MutationTypes.SET_RESULTS, results);
          store.commit(MutationTypes.TOGGLE_SELECTION, undefined);
          results = null;
          return;
        }

        const ignoredUuids = results[0].leftovers.reduce((acc: string[], leftover) => {
          acc.push(leftover.uuid);
          return acc;
        }, []);

        store.commit(MutationTypes.RESERVE_PLAYERS, ignoredUuids);
        store.commit(MutationTypes.ADD_TEAMS, results[0].teams);

        results = null;
      } catch (e) {
        console.error(e.message);
      }
    };

    return {
      balance,
      isActive,
      progress,
      closeModal,
      balanceType,
      disableType,
    };
  },
});
</script>
