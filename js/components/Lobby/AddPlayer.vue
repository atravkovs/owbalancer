<template>
  <div class="row g-3">
    <div class="col-auto">
      <label for="playerTag" class="sr-only">BattleTag</label>
      <input
        v-model.trim="playerTag"
        @keyup.enter="addPlayer"
        type="text"
        class="form-control"
        id="playerTag"
        placeholder="BattleTag#1337"
      />
    </div>
    <div class="col-auto">
      <button @click="addPlayer" class="btn btn-danger">Add Player</button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';

import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';
import PlayerEditor from '@/objects/player';

export default defineComponent({
  name: 'AddPlayer',
  setup() {
    const store = useStore();
    const playerTag = ref('');

    function addPlayer() {
      if (!/^([A-z0-9])+#\d{3,6}$/.test(playerTag.value)) {
        console.log('Error: Please enter actual battle tag');
        return;
      }

      const player = PlayerEditor.createDefaultPlayer(playerTag.value);

      store.commit(MutationTypes.ADD_PLAYER, player);
      store.commit(MutationTypes.EDIT_PLAYER, player.identity.battleTag);
      playerTag.value = '';
    }

    return {
      addPlayer,
      playerTag,
    };
  },
});
</script>
