<template>
  <div @dragover="allowDrop" @drop="drop" class="border border-danger w-100">Delete Player</div>
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
      const playerTag = ev?.dataTransfer?.getData('playerTag');
      store.commit(MutationTypes.DELETE_PLAYER, playerTag);
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
</style>
