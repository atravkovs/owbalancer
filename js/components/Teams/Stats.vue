<template>
  <div class="mt-2">
    <span>Min SR: {{ minSr }}</span>
    <span class="ms-2">Max SR: {{ maxSr }}</span>
    <span class="ms-2">Difference: {{ maxSr - minSr }}</span>
    <span class="ms-2">Average: {{ avgSr }}</span>
    <span class="ms-2">Off roles: {{ offRolesPercentage }}%</span>
  </div>
</template>

<script lang="ts">
import { useStore } from '@/store';
import { computed, defineComponent } from 'vue';

import { Team as TeamType } from '@/objects/team';
import PObj, { Players } from '@/objects/player';

export default defineComponent({
  name: 'Stats',
  setup() {
    const store = useStore();

    const calcTeamAvg = (team: TeamType, players: Players): number => {
      return !store.state.showBalancerSR
        ? team.members.reduce(
            (acc, member) => acc + players[member.uuid].stats.classes[member.role].rank,
            0
          ) / team.members.length
        : team.avgSr;
    };

    const maxSr = computed(() =>
      Math.floor(
        store.state.teams.reduce(
          (acc, team) =>
            acc < calcTeamAvg(team, store.state.players)
              ? calcTeamAvg(team, store.state.players)
              : acc,
          0
        )
      )
    );

    const minSr = computed(() =>
      Math.floor(
        store.state.teams.reduce(
          (acc, team) =>
            acc > calcTeamAvg(team, store.state.players)
              ? calcTeamAvg(team, store.state.players)
              : acc,
          10000
        )
      )
    );

    const avgSr = computed(() =>
      Math.floor(
        store.state.teams.reduce((acc, team) => acc + calcTeamAvg(team, store.state.players), 0) /
          store.state.teams.length
      )
    );

    const offRolesPercentage = computed(() => {
      const [offCount, totalCount] = store.state.teams.reduce(
        (acc, team) => {
          let off = acc[0];
          let total = acc[1];

          team.members.forEach(member => {
            total += 1;
            if (
              store.state.players[member.uuid] &&
              member.role !== PObj.getTopRoleName(store.state.players[member.uuid]) &&
              !store.state.players[member.uuid].identity.isFullFlex
            ) {
              off += 1;
            }
          });

          return [off, total];
        },
        [0, 0]
      );

      return Math.floor((offCount / totalCount) * 1000) / 10;
    });

    return {
      maxSr,
      minSr,
      avgSr,
      offRolesPercentage,
    };
  },
});
</script>
