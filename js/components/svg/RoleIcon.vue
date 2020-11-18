<template>
  <tank-icon v-if="rtype === 'tank'" />
  <damage-icon v-else-if="rtype === 'dps'" />
  <support-icon v-else-if="rtype === 'support'" />
  <span v-else></span>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import TankIcon from '@/components/svg/TankIcon.vue';
import DamageIcon from '@/components/svg/DamageIcon.vue';
import SupportIcon from '@/components/svg/SupportIcon.vue';

export default defineComponent({
  name: 'RoleIcon',
  props: {
    rtype: String,
  },
  components: { TankIcon, DamageIcon, SupportIcon },
});
</script>
