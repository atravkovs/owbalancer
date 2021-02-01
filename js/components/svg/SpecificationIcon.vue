<template>
  <tank-icon v-if="rtype === 'tank' && specification === 'any'" />
  <main-tank-icon v-if="rtype === 'tank' && specification === 'primary'" />
  <off-tank-icon v-if="rtype === 'tank' && specification === 'secondary'" />

  <damage-icon v-if="rtype === 'dps' && specification === 'any'" />
  <hitscan-icon v-if="rtype === 'dps' && specification === 'primary'" />
  <projectile-icon v-if="rtype === 'dps' && specification === 'secondary'" />

  <support-icon v-if="rtype === 'support' && specification === 'any'" />
  <heal-icon v-if="rtype === 'support' && specification === 'primary'" />
  <light-heal-icon v-if="rtype === 'support' && specification === 'secondary'" />
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';

import TankIcon from '@/components/svg/TankIcon.vue';
import DamageIcon from '@/components/svg/DamageIcon.vue';
import SupportIcon from '@/components/svg/SupportIcon.vue';
import MainTankIcon from './MainTankIcon.vue';
import OffTankIcon from './OffTankIcon.vue';
import HitscanIcon from './HitscanIcon.vue';
import ProjectileIcon from './ProjectileIcon.vue';
import LightHealIcon from './LightHealIcon.vue';
import HealIcon from './HealIcon.vue';

export default defineComponent({
  name: 'SpecificationIcon',
  props: {
    rtype: String as PropType<'tank' | 'dps' | 'support'>,
    specification: String as PropType<'any' | 'primary' | 'secondary'>,
  },
  components: {
    TankIcon,
    DamageIcon,
    SupportIcon,
    OffTankIcon,
    HitscanIcon,
    ProjectileIcon,
    MainTankIcon,
    LightHealIcon,
    HealIcon,
  },
});
</script>
