<template>
  <button @click="deletePlayers" type="button" class="btn btn-sm btn-danger">
    <bin-icon />
  </button>
</template>

<script lang="ts">
import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';
import { defineComponent, PropType } from 'vue';

import BinIcon from '@/components/svg/BinIcon.vue';
import { LobbyType } from '@/objects/player';

export default defineComponent({
  name: 'DeletePlayers',
  components: { BinIcon },
  props: {
    lobby: {
      type: String as PropType<LobbyType>,
      default: 'players',
    },
  },
  setup(props) {
    const store = useStore();

    const deletePlayers = () => {
      // eslint-disable-next-line
      if (window.confirm('Do you really want to Delete ALL players?'))
        store.commit(MutationTypes.DELETE_PLAYERS, props.lobby);
    };

    return { deletePlayers };
  },
});
</script>
