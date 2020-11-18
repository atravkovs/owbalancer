<template>
  <div class="input-group">
    <span class="input-group-text cursor-pointer sortable-handler">
      <role-icon :rtype="rtype" />
    </span>
    <span class="input-group-text">
      <rank-icon :rank="mRole.rank" />
    </span>
    <span class="input-group-text">
      <input
        v-model="mRole.isActive"
        type="checkbox"
        :id="`${rtype}_enabled`"
        class="form-check-input"
        :aria-label="rtype"
      />
    </span>
    <span class="input-group-text w-7rem">
      <input
        type="checkbox"
        class="btn-check"
        name="options"
        :id="`option_main_${rtype}`"
        autocomplete="off"
        v-model="mRole.primary"
      />
      <label class="btn btn-sm btn-outline-dark" :for="`option_main_${rtype}`">{{
        roles[rtype].primary
      }}</label>
    </span>
    <span class="input-group-text w-7rem">
      <input
        type="checkbox"
        class="btn-check"
        name="options"
        :id="`option_off_${rtype}`"
        autocomplete="off"
        v-model="mRole.secondary"
      />
      <label class="btn btn-sm btn-outline-dark" :for="`option_off_${rtype}`">{{
        roles[rtype].secondary
      }}</label>
    </span>
    <input
      type="number"
      id="level"
      class="form-control"
      min="0"
      :value="mRole.rank"
      @input="inpChange"
    />
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, PropType, ref } from 'vue';

import RoleIcon from '@/components/svg/RoleIcon.vue';
import RankIcon from '@/components/svg/RankIcon.vue';
import { ClassType } from '@/objects/player';

type RoleDecription = {
  primary: string;
  secondary: string;
};

type RoleDescriptions = { [role: string]: RoleDecription };

export default defineComponent({
  name: 'EditRole',
  props: {
    rtype: String,
    role: Object as PropType<ClassType>,
  },
  components: { RoleIcon, RankIcon },
  setup(props, { emit }) {
    const role = computed(() => props.role);
    const mRole = ref(role);

    const roles: RoleDescriptions = {
      tank: { primary: 'Main Tank', secondary: 'Off Tank' },
      support: { primary: 'Heal', secondary: 'Light Heal' },
      dps: { primary: 'Hitscan', secondary: 'Projectile' },
    };

    const inpChange = (e: Event) => {
      const value = (e.target as HTMLInputElement).valueAsNumber || 0;
      emit('update-rank', props.rtype, value);
    };

    return { roles, mRole, inpChange };
  },
});
</script>

<style lang="scss">
.cursor-pointer {
  cursor: pointer;
}

.s-2em {
  width: 2em;
  height: 2em;
}

.w-7rem {
  width: 7rem;
}
</style>
