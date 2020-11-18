<template>
  <button @click="generateRandom" class="btn btn-sm btn-secondary ml-1">+ Randoms</button>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import { useStore } from '@/store';
import PlayerEditor, { Players } from '@/objects/player';
import { MIN_SR, MAX_SR } from '@/constants';
import MutationTypes from '@/store/mutation-types';

function getRandomInt(min: number, max: number): number {
  const minCeil = Math.ceil(min);
  const maxFloor = Math.floor(max);
  return Math.floor(Math.random() * (maxFloor - minCeil) + minCeil);
}

function flipCoin(): boolean {
  return getRandomInt(0, 2) === 0;
}

export default defineComponent({
  name: 'GenerateRandoms',
  setup() {
    const store = useStore();

    const generateRandom = () => {
      // eslint-disable-next-line
      const playerCount = +(prompt('Enter players amount', '20') || 0);
      const players: Players = {};

      for (let i = 0; i < playerCount; i += 1) {
        const roleSelect: ('dps' | 'support' | 'tank')[] = ['dps', 'support', 'tank'];
        const player = PlayerEditor.createDefaultPlayer(`Player ${i + 1}`);
        let j = 1;

        const firstRole = getRandomInt(0, 3);
        player.stats.classes[roleSelect[firstRole]].isActive = true;
        player.stats.classes[roleSelect[firstRole]].priority = 0;
        player.stats.classes[roleSelect[firstRole]].rank = getRandomInt(MIN_SR, MAX_SR);
        roleSelect.splice(firstRole, 1);

        // Second role if needed
        if (flipCoin()) {
          const secondRole = getRandomInt(0, 2);
          player.stats.classes[roleSelect[secondRole]].isActive = true;
          player.stats.classes[roleSelect[secondRole]].priority = 1;
          player.stats.classes[roleSelect[secondRole]].rank = getRandomInt(MIN_SR, MAX_SR);

          j += 1;
          roleSelect.splice(secondRole, 1);
          if (flipCoin()) {
            player.stats.classes[roleSelect[0]].isActive = true;
            player.stats.classes[roleSelect[0]].priority = 2;
            player.stats.classes[roleSelect[0]].rank = getRandomInt(MIN_SR, MAX_SR);

            roleSelect.splice(0, 1);
          }
        }

        roleSelect.forEach(v => {
          player.stats.classes[v].priority = j;
          j += 1;
        });

        players[player.identity.uuid] = player;
      }

      store.commit(MutationTypes.ADD_PLAYERS, players);
    };

    return { generateRandom };
  },
});
</script>
