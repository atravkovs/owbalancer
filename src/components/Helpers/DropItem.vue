<template>
  <li>
    <a href="#" class="dropdown-item" @click="$emit('drop-click')">
      <slot />
    </a>
  </li>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'DropItem',
});
</script>
