<template>
  <div class="card mw-nb">
    <div class="card-header d-flex justify-content-between pr-0 py-0">
      <span class="py-1">Team {{ mTeam.name }}</span>
      <span class="bg-primary btr px-2 py-1 text-light">{{ teamAverage }}</span>
    </div>
    <div class="card-body p-0 fs-small">
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
    const teamAverage = computed(() => Math.round(props.team?.avgSr || 0));
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

    return { tanks, dps, supports, players, teamUuid, mTeam, teamAverage };
  },
});
</script>

<style lang="scss" scoped>
.btr {
  border-top-right-radius: calc(0.25rem - 1px);
}
.fs-small {
  font-size: 0.9rem;
}
</style>
