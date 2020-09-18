<template>
  <div
    @dragover="allowDrop"
    @drop="drop"
    class="border border-danger w-100 delete-player"
  >==== Delete Player ====</div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';

export default defineComponent({
  name: 'DeletePlayer',
  setup() {
    const store = useStore();

    const allowDrop = (ev: DragEvent) => {
      ev.preventDefault();
    };

    const drop = (ev: DragEvent) => {
      ev.preventDefault();
      const playerId = ev?.dataTransfer?.getData('playerTag');
      const teamName = ev?.dataTransfer?.getData('team');
      store.commit(MutationTypes.DELETE_PLAYER, playerId);

      if (teamName) {
        store.commit(MutationTypes.REMOVE_FROM_TEAM, {
          teamName,
          playerId,
        });
      }
    };

    return {
      drop,
      allowDrop,
    };
  },
});
</script>

<style lang="scss" scoped>
div {
  height: 5rem;
  max-width: 25rem;
}

.delete-player {
  font-weight: bold;
  line-height: 5rem;
  text-align: center;
  text-transform: uppercase;
  border-style: dashed !important;
}
</style>
