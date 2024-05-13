<template>
  <div class="row g-3">
    <div class="col">
      <label for="playerTag" class="visually-hidden">Name</label>
      <input
        v-model.trim="playerTag"
        @keyup.enter="addPlayer"
        type="text"
        class="form-control form-control-sm"
        id="playerTag"
        placeholder="Name#1337"
      />
    </div>
    <div class="col-auto">
      <button @click="addPlayer" class="btn btn-sm btn-danger">Add Player</button>
      <generate-randoms v-if="enableRandoms" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType, ref } from 'vue';

import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';
import PlayerEditor, { LobbyType } from '@/objects/player';

import GenerateRandoms from '@/components/Lobby/GenerateRandoms.vue';

export default defineComponent({
  name: 'AddPlayer',
  components: { GenerateRandoms },
  props: {
    enableRandoms: {
      type: Boolean,
      default: true,
    },
    lobby: {
      type: String as PropType<LobbyType>,
      default: 'players'
    },
  },
  setup(props) {
    const store = useStore();
    const playerTag = ref('');

    function addPlayer() {
      const player = PlayerEditor.createDefaultPlayer(playerTag.value);

      store.commit(MutationTypes.ADD_PLAYER, { player, lobby: props.lobby });
      store.commit(MutationTypes.EDIT_PLAYER, { playerId: player.identity.uuid, lobby: props.lobby });
      playerTag.value = '';
    }

    return {
      addPlayer,
      playerTag,
    };
  },
});
</script>
