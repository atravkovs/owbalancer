<template>
  <div class="mb-3">
    <label class="form-label w-100">Disable</label>
    <div class="btn-group">
      <input
        type="radio"
        value="none"
        id="disable1"
        class="btn-check"
        :checked="modelValue === 'none'"
        @input="$emit('update:modelValue', $event.target.value)"
      />
      <label class="btn btn-primary" for="disable1">None</label>
      <input
        type="radio"
        value="ex_caps"
        id="disable2"
        class="btn-check"
        :checked="modelValue === 'ex_caps'"
        @input="$emit('update:modelValue', $event.target.value)"
      />
      <label class="btn btn-primary" for="disable2">Except captains</label>
      <input
        type="radio"
        value="leut_ens"
        id="disable3"
        class="btn-check"
        :checked="modelValue === 'leut_ens'"
        @input="$emit('update:modelValue', $event.target.value)"
      />
      <label class="btn btn-primary" for="disable3">Leutenants and Ensigns</label>
      <input
        type="radio"
        value="ens"
        id="disable4"
        class="btn-check"
        :checked="modelValue === 'ens'"
        @input="$emit('update:modelValue', $event.target.value)"
      />
      <label class="btn btn-primary" for="disable4">Ensigns</label>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'BalanceDisable',
  props: {
    modelValue: String,
  },
});
</script>
