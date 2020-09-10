<template>
  <div>
    <div class="row">
      <div class="col-sm-3">
        <label for="battleTag" class="col-form-label">BattleTag</label>
      </div>
      <div class="col-auto text-center">
        <input
          type="text"
          readonly
          id="battleTag"
          class="form-control-plaintext"
          v-model="mIdentity.battleTag"
        />
      </div>
    </div>
    <div class="row">
      <div class="col-sm-3">
        <label for="displayName" class="col-form-label">Display Name</label>
      </div>
      <div class="col-auto">
        <input type="text" id="displayName" class="form-control" v-model="mIdentity.displayName" />
      </div>
    </div>
    <div class="row">
      <div class="col-sm-3">
        <label for="twitchName" class="col-form-label">Twitch Name</label>
      </div>
      <div class="col-auto">
        <input type="text" id="twitchName" class="form-control" v-model="mIdentity.twitchName" />
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
