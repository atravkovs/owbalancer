<template>
  <Dropdown id="sort">
    <template #title>⇅ <span class="d-xs-none d-md-none d-lg-none d-xl-inline">Sort</span></template>
    <drop-item @drop-click="$emit('sort', 'date', 'asc')">▲ Order of addition</drop-item>
    <drop-item @drop-click="$emit('sort', 'date', 'desc')">▼ Order of addition</drop-item>
    <drop-item @drop-click="$emit('sort', 'name', 'asc')">▲ Name</drop-item>
    <drop-item @drop-click="$emit('sort', 'name', 'desc')">▼ Name</drop-item>
    <drop-item @drop-click="$emit('sort', 'sr', 'asc')">▲ Rating</drop-item>
    <drop-item @drop-click="$emit('sort', 'sr', 'desc')">▼ Rating</drop-item>
  </Dropdown>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import Dropdown from '@/components/Helpers/Dropdown.vue';
import DropItem from '@/components/Helpers/DropItem.vue';

export default defineComponent({
  name: 'Sort',
  components: { Dropdown, DropItem },
});
</script>
