<template>
  <div class="card c-pointer" @click="addBalance">
    <div class="card-body">
      <div>Dispersion: {{ balance.dispersion }}</div>
      <div>Unbalanced: {{ balance.leftovers.length }}</div>
      <div>Offroles: {{ offRolesPercentage }}%</div>
      <div>Anchors: {{ balance.anchors }}</div>
      <div>Dupsecs: {{ dupsecCount }}</div>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent } from 'vue';
import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';
import { TeamMembers, Team } from '@/objects/team';
import PObj from '@/objects/player';

export default defineComponent({
  name: 'Result',
  props: {
    balance: Object,
  },
  setup(props) {
    const store = useStore();

    const addBalance = () => {
      if (!props.balance) return;

      const { leftovers, teams } = props.balance;
      const ignoredUuids = leftovers.reduce((acc, leftover) => {
        acc.push(leftover.uuid);
        return acc;
      }, []);

      store.commit(MutationTypes.RESERVE_PLAYERS, ignoredUuids);
      store.commit(MutationTypes.ADD_TEAMS, teams);
    };

    const offRolesPercentage = computed(() => {
      if (!props.balance) return 0;

      const [offCount, totalCount] = props.balance.teams.reduce(
        (acc, team: Team) => {
          let off = acc[0];
          let total = acc[1];

          team.members.forEach((member: TeamMembers) => {
            total += 1;
            if (
              store.state.players[member.uuid] &&
              member.role !== PObj.getTopRoleName(store.state.players[member.uuid])
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

    const dupsecCount = computed(() => {
      if (!props.balance) return 0;

      const c = props.balance.teams.reduce((acc, team: Team) => {
        let accCount = acc;
        const dps = team.members.filter((member: TeamMembers) => member.role === 'dps');
        const tank = team.members.filter((member: TeamMembers) => member.role === 'tank');
        const support = team.members.filter((member: TeamMembers) => member.role === 'support');

        if (
          dps.every((member: TeamMembers) => member.primary) ||
          dps.every((member: TeamMembers) => member.secondary)
        ) {
          accCount += 1;
        }

        if (
          support.every((member: TeamMembers) => member.primary) ||
          support.every((member: TeamMembers) => member.secondary)
        ) {
          accCount += 1;
        }

        if (
          tank.every((member: TeamMembers) => member.primary) ||
          tank.every((member: TeamMembers) => member.secondary)
        ) {
          accCount += 1;
        }

        return accCount;
      }, 0);

      return c;
    });

    return { addBalance, offRolesPercentage, dupsecCount };
  },
});
</script>

<style lang="scss" scoped>
.c-pointer {
  cursor: pointer;
}
</style>
