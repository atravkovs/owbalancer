<template>
  <li
    v-for="i in 2"
    :key="i"
    @dragover="allowDrop"
    @drop="(e) => drop(e, i)"
    class="list-group-item d-flex pl-0"
  >
    <div class="fs-b pr-2">
      <role-icon :rtype="rtype" />
    </div>
    <div class="w-100">
      <player-card
        v-if="members[i - 1]"
        :player="players[members[i - 1].uuid]"
        :prefferedRank="members[i - 1].rank"
        :teamName="teamName"
      />
    </div>
  </li>
</template>

<script lang="ts">
import { computed, defineComponent } from 'vue';

import PObj from '@/objects/player';
import { useStore } from '@/store';

import RoleIcon from '@/components/svg/RoleIcon.vue';
import PlayerCard from '@/components/PlayerCard.vue';
import MutationTypes from '@/store/mutation-types';

export default defineComponent({
  name: 'TeamRoles',
  props: {
    rtype: String,
    teamName: String,
    members: Array,
  },
  components: { RoleIcon, PlayerCard },
  setup(props) {
    const store = useStore();
    const players = computed(() => store.state.players);

    const allowDrop = (ev: DragEvent) => {
      ev.preventDefault();
    };
    const drop = (ev: DragEvent, index: number) => {
      ev.preventDefault();
      const i = index - 1;
      const playerId = ev?.dataTransfer?.getData('playerTag');
      const teamName = ev?.dataTransfer?.getData('team');

      if (!playerId || !props.rtype || !props.teamName) return;

      const player = players.value[playerId];
      const role = PObj.getRole(player.stats.classes, props.rtype);

      if (!role.isActive) {
        return;
      }

      if (props.members && props.members[i]) {
        const member = props.members[i];
        if (member.uuid === playerId) {
          return;
        }

        if (!teamName) {
          store.commit(MutationTypes.REMOVE_FROM_TEAM, {
            teamName: props.teamName,
            playerId: member.uuid,
          });
          store.commit(MutationTypes.REMOVE_FROM_RESERVE, playerId);
          store.commit(MutationTypes.ADD_RESERVE, member.uuid);
          store.commit(MutationTypes.ADD_TEAMPLAYER, {
            teamName: props.teamName,
            playerId,
            role,
            roleName: props.rtype,
          });
        } else {
          const teamF = store.state.teams.findIndex(
            (team) => team.name === teamName
          );
          const swapCandidate = store.state.players[member.uuid];

          if (teamF >= 0 && swapCandidate) {
            const memF = store.state.teams[teamF].members.find(
              (lmember) => lmember.uuid === playerId
            );

            if (!memF) return;

            const sdRole = PObj.getRole(swapCandidate.stats.classes, memF.role);

            if (sdRole.isActive) {
              store.commit(MutationTypes.REMOVE_FROM_TEAM, {
                teamName: props.teamName,
                playerId: member.uuid,
              });
              store.commit(MutationTypes.REMOVE_FROM_TEAM, {
                teamName,
                playerId,
              });
              store.commit(MutationTypes.ADD_TEAMPLAYER, {
                teamName: props.teamName,
                playerId,
                role,
                roleName: props.rtype,
              });
              store.commit(MutationTypes.ADD_TEAMPLAYER, {
                teamName,
                playerId: member.uuid,
                role: sdRole,
                roleName: memF.role,
              });
            }
          }
        }
      } else {
        store.commit(MutationTypes.REMOVE_FROM_TEAM, {
          teamName,
          playerId,
        });
        store.commit(MutationTypes.REMOVE_FROM_RESERVE, playerId);
        store.commit(MutationTypes.ADD_TEAMPLAYER, {
          teamName: props.teamName,
          playerId,
          role,
          roleName: props.rtype,
        });
      }
    };

    return { players, allowDrop, drop };
  },
});
</script>

<style lang="scss" scoped>
.fs-b {
  font-size: 1.2em;
  line-height: 2.4em;
}
</style>
