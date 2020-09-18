<template>
  <div class="card mw-nb">
    <div class="card-header">Team {{ mTeam.name }}</div>
    <div class="card-body">
      <div>
        <span>Average SR: {{ mTeam.avgSr }}</span>
      </div>
      <ul class="list-group list-group-flush">
        <li v-for="member in tanks" :key="member.uuid" class="list-group-item d-flex pl-0">
          <div class="fs-b pr-2">
            <role-icon rtype="tank" />
          </div>
          <div class="w-100">
            <player-card
              :player="players[member.uuid]"
              :prefferedRank="member.rank"
              :teamName="teamName"
            />
          </div>
        </li>
        <li v-for="member in dps" :key="member.uuid" class="list-group-item d-flex pl-0">
          <div class="fs-b pr-2">
            <role-icon rtype="dps" />
          </div>
          <div class="w-100">
            <player-card
              :player="players[member.uuid]"
              :prefferedRank="member.rank"
              :teamName="teamName"
            />
          </div>
        </li>
        <li v-for="member in supports" :key="member.uuid" class="list-group-item d-flex pl-0">
          <div class="fs-b pr-2">
            <role-icon rtype="support" />
          </div>
          <div class="w-100">
            <player-card
              :player="players[member.uuid]"
              :teamName="teamName"
              :prefferedRank="member.rank"
            />
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, PropType } from 'vue';
import { Team } from '@/objects/team';
import { useStore } from '@/store';

import RoleIcon from '@/components/svg/RoleIcon.vue';
import PlayerCard from '@/components/PlayerCard.vue';

export default defineComponent({
  name: 'Team',
  props: {
    team: Object as PropType<Team>,
  },
  components: { RoleIcon, PlayerCard },
  setup(props) {
    const store = useStore();
    const players = computed(() => store.state.players);
    const teamName = computed(() => props.team?.name);
    const mTeam = computed(() =>
      store.state.teams.find((team) => team.name === teamName.value)
    );

    const tanks = computed(() =>
      mTeam.value?.members.filter((member) => member.role === 'tank')
    );
    const dps = computed(() =>
      mTeam.value?.members.filter((member) => member.role === 'dps')
    );
    const supports = computed(() =>
      mTeam.value?.members.filter((member) => member.role === 'support')
    );

    return { tanks, dps, supports, players, teamName, mTeam };
  },
});
</script>

<style lang="scss" scoped>
.fs-b {
  font-size: 1.2em;
  line-height: 3em;
}
</style>
