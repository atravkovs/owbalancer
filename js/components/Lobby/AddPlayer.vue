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
      <button @click="generateRandom" class="btn btn-sm btn-secondary ml-1">+ Randoms</button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';

import { useStore } from '@/store';
import { MIN_SR, MAX_SR } from '@/constants';
import MutationTypes from '@/store/mutation-types';
import PlayerEditor from '@/objects/player';

function getRandomInt(min: number, max: number) {
  const minCeil = Math.ceil(min);
  const maxFloor = Math.floor(max);
  return Math.floor(Math.random() * (maxFloor - minCeil) + minCeil);
}

function flipCoin(): boolean {
  return getRandomInt(0, 2) === 0;
}

export default defineComponent({
  name: 'AddPlayer',
  setup() {
    const store = useStore();
    const playerTag = ref('');

    function addPlayer() {
      const player = PlayerEditor.createDefaultPlayer(playerTag.value);

      store.commit(MutationTypes.ADD_PLAYER, player);
      store.commit(MutationTypes.EDIT_PLAYER, player.identity.uuid);
      playerTag.value = '';
    }

    const generateRandom = () => {
      // eslint-disable-next-line
      const playerCount = +(prompt('Enter players amoun', '20') || 0);
      const players = {};

      for (let i = 0; i < playerCount; i += 1) {
        const roleSelect = ['dps', 'support', 'tank'];
        const player = PlayerEditor.createDefaultPlayer(`Player ${i + 1}`);

        const firstRole = getRandomInt(0, 3);
        // eslint-disable-next-line
        player.stats.classes[roleSelect[firstRole]].isActive = true;
        // eslint-disable-next-line
        player.stats.classes[roleSelect[firstRole]].priority = 0;
        // eslint-disable-next-line
        player.stats.classes[roleSelect[firstRole]].rank = getRandomInt(
          MIN_SR,
          MAX_SR
        );
        roleSelect.splice(firstRole, 1);

        // Second role if needed
        if (flipCoin()) {
          const secondRole = getRandomInt(0, 2);
          // eslint-disable-next-line
          player.stats.classes[roleSelect[secondRole]].isActive = true;
          // eslint-disable-next-line
          player.stats.classes[roleSelect[secondRole]].priority = 1;
          // eslint-disable-next-line
          player.stats.classes[roleSelect[secondRole]].rank = getRandomInt(
            MIN_SR,
            MAX_SR
          );

          roleSelect.splice(secondRole, 1);
          if (flipCoin()) {
            // eslint-disable-next-line
            player.stats.classes[roleSelect[0]].isActive = true;
            // eslint-disable-next-line
            player.stats.classes[roleSelect[0]].priority = 2;
            // eslint-disable-next-line
            player.stats.classes[roleSelect[0]].rank = getRandomInt(
              MIN_SR,
              MAX_SR
            );

            roleSelect.splice(0, 1);
          }
        }

        let j = 1;
        roleSelect.forEach((v) => {
          // eslint-disable-next-line
          player.stats.classes[v].priority = j;
          j += 1;
        });

        // eslint-disable-next-line
        players[player.identity.uuid] = player;
      }

      store.commit(MutationTypes.ADD_PLAYERS, players);
    };

    return {
      addPlayer,
      playerTag,
      generateRandom,
    };
  },
});
</script>
