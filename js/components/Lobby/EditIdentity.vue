<template>
  <div>
    <div class="row">
      <div class="col-sm-3">
        <label for="name" class="col-form-label">Name</label>
      </div>
      <div class="col-auto">
        <input type="text" id="name" class="form-control" v-model="mIdentity.name" />
      </div>
    </div>
    <div class="row">
      <div class="col-sm-3">
        <label for="captain" class="col-form-label">Captain</label>
      </div>
      <div class="col-auto">
        <input
          type="checkbox"
          id="captain"
          class="form-check-input mt-2"
          v-model="mIdentity.isCaptain"
        />
      </div>
    </div>
    <div class="row">
      <div class="col-sm-3">
        <label for="squire" class="col-form-label">Squire</label>
      </div>
      <div class="col-auto">
        <input
          type="checkbox"
          id="squire"
          class="form-check-input mt-2"
          v-model="mIdentity.isSquire"
        />
      </div>
    </div>
    <div class="row">
      <div class="col-sm-3">
        <label for="fullFlex" class="col-form-label">Full flex</label>
      </div>
      <div class="col-auto">
        <input
          type="checkbox"
          id="fullFlex"
          class="form-check-input mt-2"
          v-model="mIdentity.isFullFlex"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { Identity } from '@/objects/player';
import { computed, defineComponent, PropType, ref } from 'vue';

export default defineComponent({
  name: 'EditIdentity',
  props: {
    identity: Object as PropType<Identity>,
  },
  setup(props) {
    const identity = computed(() => props.identity);
    const mIdentity = ref(identity);

    return { mIdentity };
  },
});
</script>
