<template>
  <Dropdown id="sort" title="⇅ Sort">
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