<template>
  <div class="row g-3">
    <div class="col-auto">
      <label for="playerTag" class="sr-only">Name</label>
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
      <generate-randoms />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';

import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';
import PlayerEditor from '@/objects/player';

import GenerateRandoms from '@/components/Lobby/GenerateRandoms.vue';

export default defineComponent({
  name: 'AddPlayer',
  components: { GenerateRandoms },
  setup() {
    const store = useStore();
    const playerTag = ref('');

    function addPlayer() {
      const player = PlayerEditor.createDefaultPlayer(playerTag.value);

      store.commit(MutationTypes.ADD_PLAYER, player);
      store.commit(MutationTypes.EDIT_PLAYER, player.identity.uuid);
      playerTag.value = '';
    }

    return {
      addPlayer,
      playerTag,
    };
  },
});
</script>
