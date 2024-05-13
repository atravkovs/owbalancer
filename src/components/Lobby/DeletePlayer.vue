<template>
  <div @dragover="allowDrop" @drop="drop" class="border border-danger w-100 delete-player">
    ==== Delete Player ====
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';

import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';
import { LobbyType } from '@/objects/player';

export default defineComponent({
  name: 'DeletePlayer',
  props: {
    lobby: {
      type: String as PropType<LobbyType>,
      default: 'players'
    }
  },
  setup(props) {
    const store = useStore();

    const allowDrop = (ev: DragEvent) => {
      ev.preventDefault();
    };

    const drop = (ev: DragEvent) => {
      ev.preventDefault();
      const playerId = ev?.dataTransfer?.getData('playerTag');
      const teamUuid = ev?.dataTransfer?.getData('team');

      if (playerId === undefined) return;

      store.commit(MutationTypes.DELETE_PLAYER, { playerId, lobby: props.lobby });

      if (teamUuid) {
        store.commit(MutationTypes.REMOVE_FROM_TEAM, {
          teamUuid,
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
}

.delete-player {
  font-weight: bold;
  line-height: 5rem;
  text-align: center;
  text-transform: uppercase;
  border-style: dashed !important;
}
</style>
