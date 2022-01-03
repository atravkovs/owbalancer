<template>
  <div class="container-fluid px-0">
    <div class="d-flex justify-content-between w-100 py-2 mb-1 px-2 mx-auto">
      <div class="d-flex">
        <OWIcon />
        <h3 class="ms-2">Tournament Balancer</h3>
      </div>
      <span>
        Created by
        <a target="blank" href="https://github.com/atravkovs">s0ck3t</a>
      </span>
    </div>
    <div class="row px-2 pb-2 mx-auto">
      <div class="backup col" :class="{ open: displayBackup }">
        <span @click="toggleBackup">Backup</span>
      </div>
      <div class="lobby-wrapper col" v-if="displayBackup">
        <BackupLobby />
      </div>
      <div class="swap col" v-if="displayBackup">
        <Swap />
      </div>
      <div class="lobby-wrapper col">
        <Lobby />
      </div>
      <div class="col">
        <Teams />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import '@/styles/main.scss';
import { computed, defineComponent } from 'vue';

import Swap from '@/components/Swap.vue';
import Lobby from '@/components/Lobby.vue';
import Teams from '@/components/Teams.vue';
import OWIcon from '@/components/svg/OWIcon.vue';
import BackupLobby from '@/components/BackupLobby.vue';
import { useStore } from './store';
import MutationTypes from './store/mutation-types';

export default defineComponent({
  name: 'App',
  components: { Lobby, Teams, OWIcon, BackupLobby, Swap },
  setup() {
    const store = useStore();

    const toggleBackup = () => {
      store.commit(MutationTypes.TOGGLE_BACKUP, undefined);
    };

    const displayBackup = computed(() => store.state.showBackup);

    return { displayBackup, toggleBackup };
  }
});
</script>

<style lang="scss" scoped>
@import '~bootstrap/scss/functions';
@import '~bootstrap/scss/variables';
@import '~bootstrap/scss/mixins';

.w-25r {
  width: 25rem !important;
}

.swap {
  flex: none;
  width: 5rem !important;
  padding-top: 200px;
}

.backup {
  flex: none;
  width: 1rem;

  span {
    transform: rotateZ(90deg);
    display: block;
    border-top: 2px solid $blue-400;
    width: min-content;
    position: absolute;
    left: -7px;
    margin-top: 21px;
    cursor: pointer;
  }

  &.open span {
    border-top: 2px solid $red-400;
  }
}

@include media-breakpoint-up(md) {
  .lobby-wrapper {
    flex: none;
    width: 25rem !important;
  }
}

img {
  width: 24rem;
}
</style>
