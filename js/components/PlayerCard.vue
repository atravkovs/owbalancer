<template>
  <div draggable="true" @contextmenu="editPlayer" @dragstart="drag">
    {{ player.identity.displayName }}
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';

import { useStore } from '@/store';
import { Player } from '@/objects/player';
import MutationTypes from '@/store/mutation-types';

export default defineComponent({
  name: 'PlayerCard',
  props: {
    player: Object as PropType<Player>,
  },
  setup(props) {
    const store = useStore();

    const drag = (ev: DragEvent) => {
      const _ = ev?.dataTransfer?.setData('playerTag', props.player?.identity.battleTag || '');

      return _;
    };

    const editPlayer = (e: MouseEvent) => {
      e.preventDefault();

      if (props.player === undefined) {
        return;
      }

      store.commit(MutationTypes.EDIT_PLAYER, props.player.identity.battleTag);
    };

    return {
      drag,
      editPlayer,
    };
  },
});
</script>
