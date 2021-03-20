<template>
  <dropdown id="assign" >
    <template #title>♛ <span class="d-xs-none d-md-none d-lg-none d-xl-inline">Extra</span> </template>
    <drop-item @drop-click="assignAuto">Distribute Auto</drop-item>
    <drop-item @drop-click="assignCaptains">Assign Captains</drop-item>
    <drop-item @drop-click="assignSquires">Assign Squires</drop-item>
    <drop-item @drop-click="clearCaptains">Clear Captains</drop-item>
    <drop-item @drop-click="clearSquires">Clear Squires</drop-item>
    <drop-item @drop-click="clearAll">Clear All</drop-item>
  </dropdown>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useStore } from '@/store';

import Dropdown from '@/components/Helpers/Dropdown.vue';
import DropItem from '@/components/Helpers/DropItem.vue';
import MutationTypes from '@/store/mutation-types';

export default defineComponent({
  name: 'Assign',
  components: { Dropdown, DropItem },
  setup() {
    const store = useStore();

    const assignAuto = () => {
      store.commit(MutationTypes.CLEAR_ALL_EXTRA, undefined);
      store.commit(MutationTypes.ASSIGN_CAPTAINS, 0);
      store.commit(MutationTypes.ASSIGN_SQUIRES, 5500);
    };

    const assignCaptains = () => {
      // eslint-disable-next-line
      const captainSR = +(prompt("Enter captain's minimum SR", '3600') || 0);
      store.commit(MutationTypes.ASSIGN_CAPTAINS, captainSR);
    };

    const assignSquires = () => {
      // eslint-disable-next-line
      const squireSR = +(prompt("Enter suire's maximum SR", '1700') || 0);
      store.commit(MutationTypes.ASSIGN_SQUIRES, squireSR);
    };

    const clearCaptains = () => {
      store.commit(MutationTypes.CLEAR_CAPTAINS, undefined);
    };

    const clearSquires = () => {
      store.commit(MutationTypes.CLEAR_SQUIRES, undefined);
    };

    const clearAll = () => {
      store.commit(MutationTypes.CLEAR_ALL_EXTRA, undefined);
    };

    return {
      assignAuto,
      assignCaptains,
      assignSquires,
      clearCaptains,
      clearSquires,
      clearAll,
    };
  },
});
</script>
