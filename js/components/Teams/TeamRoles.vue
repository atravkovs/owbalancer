<template>
  <li
    v-for="i in 2"
    :key="i"
    @dragover="allowDrop"
    @drop="e => drop(e, i)"
    class="list-group-item d-flex p-0 ps-3 pe-1"
  >
    <div class="fs-b pe-2">
      <role-icon :rtype="rtype" />
    </div>
    <div class="w-100 lh-26">
      <player-card
        v-if="members[i - 1]"
        :player="players[members[i - 1].uuid]"
        :prefferedRank="members[i - 1].rank"
        :rankRole="showBalancerSR ? undefined : members[i - 1].role"
        :teamUuid="teamUuid"
      />
    </div>
  </li>
</template>

<script lang="ts">
import { computed, defineComponent, PropType } from 'vue';

import PObj from '@/objects/player';
import { useStore } from '@/store';

import RoleIcon from '@/components/svg/RoleIcon.vue';
import PlayerCard from '@/components/PlayerCard.vue';
import MutationTypes from '@/store/mutation-types';

export default defineComponent({
  name: 'TeamRoles',
  props: {
    rtype: String as PropType<'dps' | 'tank' | 'support'>,
    teamUuid: String,
    members: Array as PropType<{ uuid: string; name: string; primary: boolean; secondary: boolean }[]>,
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
      const teamUuid = ev?.dataTransfer?.getData('team');

      if (!playerId || !props.rtype || !props.teamUuid) return;

      const player = players.value[playerId];
      const role = PObj.getRole(player.stats.classes, props.rtype);

      if (!role.isActive) {
        return;
      }

      if (props.members) {
        const member = props.members[i];

        if (member && member.uuid === playerId) {
          return;
        }

        if (!teamUuid) {
          if (member) {
            store.commit(MutationTypes.REMOVE_FROM_TEAM, {
              teamUuid: props.teamUuid,
              playerId: member.uuid,
            });
            store.commit(MutationTypes.ADD_RESERVE, member.uuid);
          }
          store.commit(MutationTypes.REMOVE_FROM_RESERVE, playerId);
          store.commit(MutationTypes.ADD_TEAMPLAYER, {
            teamUuid: props.teamUuid,
            playerId,
            role,
            roleName: props.rtype,
            playerName: player.identity.name,
            primary: role.primary,
            secondary: role.secondary,
          });
        } else {
          const teamF = store.state.teams.findIndex(team => team.uuid === teamUuid);
          const swapCandidate = store.state.players[member.uuid];

          if (teamF >= 0 && swapCandidate) {
            const memF = store.state.teams[teamF].members.find(
              lmember => lmember.uuid === playerId
            );

            if (!memF) return;

            const sdRole = PObj.getRole(swapCandidate.stats.classes, memF.role);

            if (sdRole.isActive) {
              store.commit(MutationTypes.REMOVE_FROM_TEAM, {
                teamUuid: props.teamUuid,
                playerId: member.uuid,
              });
              store.commit(MutationTypes.REMOVE_FROM_TEAM, {
                teamUuid,
                playerId,
              });
              store.commit(MutationTypes.ADD_TEAMPLAYER, {
                teamUuid: props.teamUuid,
                playerId,
                role,
                roleName: props.rtype,
                playerName: player.identity.name,
                primary: role.primary,
                secondary: role.secondary,
              });
              store.commit(MutationTypes.ADD_TEAMPLAYER, {
                teamUuid,
                playerId: member.uuid,
                role: sdRole,
                roleName: memF.role,
                playerName: member.name,
                primary: member.primary,
                secondary: member.secondary,
              });
            }
          }
        }
      } else if (teamUuid) {
        store.commit(MutationTypes.REMOVE_FROM_TEAM, {
          teamUuid,
          playerId,
        });
        store.commit(MutationTypes.REMOVE_FROM_RESERVE, playerId);
        store.commit(MutationTypes.ADD_TEAMPLAYER, {
          teamUuid: props.teamUuid,
          playerId,
          role,
          roleName: props.rtype,
          playerName: player.identity.name,
          primary: role.primary,
          secondary: role.secondary,
        });
      }
    };

    const showBalancerSR = computed(() => store.state.showBalancerSR);

    return { players, allowDrop, drop, showBalancerSR };
  },
});
</script>

<style lang="scss" scoped>
.fs-b {
  font-size: 1.2em;
  line-height: 2.4em;
}
.lh-26 {
  line-height: 2.6rem;
}
</style>
