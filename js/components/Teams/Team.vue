<template>
  <div class="card mw-nb">
    <div class="card-header">Team {{ mTeam.name }}</div>
    <div class="card-body">
      <div>
        <span>Average SR: {{ mTeam.avgSr }}</span>
      </div>
      <ul class="list-group list-group-flush">
        <team-roles :members="tanks" rtype="tank" :teamUuid="teamUuid" />
        <team-roles :members="dps" rtype="dps" :teamUuid="teamUuid" />
        <team-roles :members="supports" rtype="support" :teamUuid="teamUuid" />
      </ul>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, PropType } from 'vue';
import { Team } from '@/objects/team';
import { useStore } from '@/store';

import TeamRoles from '@/components/Teams/TeamRoles.vue';

export default defineComponent({
  name: 'Team',
  props: {
    team: Object as PropType<Team>,
  },
  components: { TeamRoles },
  setup(props) {
    const store = useStore();
    const players = computed(() => store.state.players);
    const teamUuid = computed(() => props.team?.uuid);
    const mTeam = computed(() =>
      store.state.teams.find((team) => team.uuid === teamUuid.value)
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

    return { tanks, dps, supports, players, teamUuid, mTeam };
  },
});
</script>
