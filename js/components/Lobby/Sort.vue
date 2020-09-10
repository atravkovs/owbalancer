<template>
  <Dropdown id="sort" title="⇅ Sort">
    <li>
      <a href="#" class="dropdown-item" @click="$emit('sort', 'date', 'asc')">▲ Order of addition</a>
    </li>
    <li>
      <a href="#" class="dropdown-item" @click="$emit('sort', 'date', 'desc')">▼ Order of addition</a>
    </li>
    <li>
      <a href="#" class="dropdown-item" @click="$emit('sort', 'name', 'asc')">▲ Name</a>
    </li>
    <li>
      <a href="#" class="dropdown-item" @click="$emit('sort', 'name', 'desc')">▼ Name</a>
    </li>
    <li>
      <a href="#" class="dropdown-item" @click="$emit('sort', 'sr', 'asc')">▲ Rating</a>
    </li>
    <li>
      <a href="#" class="dropdown-item" @click="$emit('sort', 'sr', 'desc')">▼ Rating</a>
    </li>
  </Dropdown>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import Dropdown from '@/components/Helpers/Dropdown.vue';

export default defineComponent({
  name: 'Sort',
  components: { Dropdown },
});
</script>