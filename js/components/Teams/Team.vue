<template>
  <div class="card mw-nb mh-300">
    <div class="card-header d-flex justify-content-between p-0">
      <div class="py-1 d-flex">
        <u class="ml-2 c-pointer" @click="removeTeam">x</u>
        <span class="ml-2">Team</span>
        <input
          type="text"
          class="form-control-plaintext p-0 pl-1"
          :value="mTeam.name"
          @input="updateTeam"
        />
      </div>
      <span class="bg-secondary btr px-2 py-1 text-light">{{ teamAverage }}</span>
    </div>
    <div class="card-body p-0 pb-1 fs-small">
      <ul class="list-group list-group-flush">
        <team-roles :members="tanks" rtype="tank" :teamUuid="teamUuid" />
        <team-roles :members="dps" rtype="dps" :teamUuid="teamUuid" />
        <team-roles :members="supports" rtype="support" :teamUuid="teamUuid" />
      </ul>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, PropType, ref } from 'vue';
import { Team } from '@/objects/team';
import { useStore } from '@/store';
import debounce from 'lodash/debounce';

import TeamRoles from '@/components/Teams/TeamRoles.vue';
import MutationTypes from '@/store/mutation-types';

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
    const teamAverage = computed(() => {
      if (!props.team) return 0;

      if (!store.state.showBalancerSR) {
        return Math.round(
          props.team?.members.reduce(
            (acc, member) => acc + store.state.players[member.uuid].stats.classes[member.role].rank,
            0
          ) / props.team?.members.length
        );
      }

      return Math.round(props.team?.avgSr);
    });
    const cTeam = computed(() => store.state.teams.find(team => team.uuid === teamUuid.value));
    const mTeam = ref(cTeam);

    const tanks = computed(() => mTeam.value?.members.filter(member => member.role === 'tank'));
    const dps = computed(() => mTeam.value?.members.filter(member => member.role === 'dps'));
    const supports = computed(() =>
      mTeam.value?.members.filter(member => member.role === 'support')
    );

    const updateTeam = debounce((e: Event) => {
      const teamName = (e.target as HTMLInputElement).value;

      if (cTeam.value && teamName) {
        store.commit(MutationTypes.UPDATE_TEAM_NAME, {
          teamUuid: cTeam.value.uuid,
          teamName,
        });
      }
    }, 1000);

    const removeTeam = () => {
      if (!cTeam.value) return;
      store.commit(MutationTypes.REMOVE_TEAM, cTeam.value.uuid);
    };

    return {
      tanks,
      dps,
      supports,
      players,
      teamUuid,
      mTeam,
      teamAverage,
      updateTeam,
      removeTeam,
    };
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
.mh-300 {
  max-height: 300px;
}
.c-pointer {
  cursor: pointer;
}
.form-control-plaintext:focus {
  outline: 0;
}
u {
  opacity: 0.4;
  text-decoration: none;

  &:hover {
    opacity: 1;
    text-decoration: underline;
  }
}
</style>
