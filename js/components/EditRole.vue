<template>
  <div class="input-group">
    <span class="input-group-text cursor-pointer">
      <TankIcon v-if="rtype === 'tank'" />
      <DamageIcon v-if="rtype === 'dps'" />
      <SupportIcon v-if="rtype === 'support'" />
    </span>
    <span class="input-group-text">
      <img v-if="mRole.rating < 1500" class="s-2em" src="@/assets/rank_icons/Bronze.png" />
      <img v-else-if="mRole.rating < 2000" class="s-2em" src="@/assets/rank_icons/Silver.png" />
      <img v-else-if="mRole.rating < 2500" class="s-2em" src="@/assets/rank_icons/Gold.png" />
      <img v-else-if="mRole.rating < 3000" class="s-2em" src="@/assets/rank_icons/Platinum.png" />
      <img v-else-if="mRole.rating < 3500" class="s-2em" src="@/assets/rank_icons/Diamond.png" />
      <img v-else-if="mRole.rating < 4000" class="s-2em" src="@/assets/rank_icons/Master.png" />
      <img
        v-else-if="mRole.rating < 5000"
        class="s-2em"
        src="@/assets/rank_icons/Grandmaster.png"
      />
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
      v-model="mRole.rating"
      type="number"
      :id="rtype"
      class="form-control"
      placeholder="Rating"
    />
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, PropType, ref } from 'vue';

import TankIcon from '@/components/svg/TankIcon.vue';
import SupportIcon from '@/components/svg/SupportIcon.vue';
import DamageIcon from '@/components/svg/DamageIcon.vue';
import { ClassType } from '@/objects/player';

export default defineComponent({
  name: 'EditRole',
  props: {
    rtype: String,
    role: Object as PropType<ClassType>,
  },
  components: { TankIcon, SupportIcon, DamageIcon },
  setup(props) {
    const role = computed(() => props.role);
    const mRole = ref(role);

    return { mRole };
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
