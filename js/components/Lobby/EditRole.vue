<template>
  <div class="input-group">
    <span class="input-group-text cursor-pointer sortable-handler">
      <RoleIcon :rtype="rtype" />
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
    <span class="input-group-text w-7rem" v-if="rtype === 'tank'">
      <input
        type="checkbox"
        class="btn-check"
        name="options"
        id="option_main_tank"
        autocomplete="off"
        v-model="mRole.primary"
      />
      <label class="btn btn-sm btn-secondary" for="option_main_tank">Main Tank</label>
    </span>
    <span class="input-group-text w-7rem" v-if="rtype === 'tank'">
      <input
        type="checkbox"
        class="btn-check"
        name="options"
        id="option_off_tank"
        autocomplete="off"
        v-model="mRole.secondary"
      />
      <label class="btn btn-sm btn-secondary" for="option_off_tank">Off Tank</label>
    </span>
    <span class="input-group-text w-7rem" v-if="rtype === 'support'">
      <input
        type="checkbox"
        class="btn-check"
        name="options"
        id="option_heal"
        autocomplete="off"
        v-model="mRole.primary"
      />
      <label class="btn btn-sm btn-secondary" for="option_heal">Heal</label>
    </span>
    <span class="input-group-text w-7rem" v-if="rtype === 'support'">
      <input
        type="checkbox"
        class="btn-check"
        name="options"
        id="option_light_heal"
        autocomplete="off"
        v-model="mRole.secondary"
      />
      <label class="btn btn-sm btn-secondary" for="option_light_heal">Light Heal</label>
    </span>
    <span class="input-group-text w-7rem" v-if="rtype === 'dps'">
      <input
        type="checkbox"
        class="btn-check"
        name="options"
        id="option_hitscan"
        autocomplete="off"
        v-model="mRole.primary"
      />
      <label class="btn btn-sm btn-secondary" for="option_hitscan">Hitscan</label>
    </span>
    <span class="input-group-text w-7rem" v-if="rtype === 'dps'">
      <input
        type="checkbox"
        class="btn-check"
        name="options"
        id="option_projectile"
        autocomplete="off"
        v-model="mRole.secondary"
      />
      <label class="btn btn-sm btn-secondary" for="option_projectile">Projectile</label>
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

    const inpChange = (e: Event) => {
      const value = (e.target as HTMLInputElement).valueAsNumber;
      emit('update-rank', props.rtype, value);
    };

    return { mRole, inpChange };
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
