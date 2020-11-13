<template>
  <Modal
    :isActive="player.identity.name.length > 0"
    :title="player.identity.name"
    @close-modal="closeModal"
    @save-changes="saveChanges"
  >
    <fieldset class="EditPlayer-Block">
      <h3>Identity</h3>
      <EditIdentity :identity="player.identity" />
    </fieldset>
    <fieldset class="EditPlayer-Block">
      <h3>Stats</h3>
      <EditStats :stats="player.stats" :uuid="player.identity.uuid" v-on:update-rank="updateRank" />
    </fieldset>
  </Modal>
</template>

<script lang="ts">
import { defineComponent, computed, ref } from 'vue';

import { useStore } from '@/store';
import MutationTypes from '@/store/mutation-types';
import PlayerEditor from '@/objects/player';

import Modal from '@/components/Helpers/Modal.vue';
import EditStats from '@/components/Lobby/EditStats.vue';
import EditIdentity from '@/components/Lobby/EditIdentity.vue';

export default defineComponent({
  name: 'EditPlayer',
  components: { Modal, EditStats, EditIdentity },
  setup() {
    const store = useStore();
    const emptyPlayer = PlayerEditor.createDefaultPlayer('');

    const playerData = computed(() => store.state.players[store.state.editPlayer] || emptyPlayer);

    const player = ref(playerData);

    const closeModal = () => {
      store.commit(MutationTypes.EMPTY_NO_RANK);
      store.commit(MutationTypes.CLEAR_EDIT_PLAYER);
    };

    const saveChanges = () => {
      store.commit(MutationTypes.EMPTY_NO_RANK);
      // By executing mutation changes are being synchronized from object to local storage
      store.commit(MutationTypes.EDIT_PLAYER, player.value.identity.uuid);
    };

    // eslint-ignore-next-line
    const updateRank = (role: 'dps' | 'support' | 'tank', rank: number) => {
      store.commit(MutationTypes.EDIT_RANK, {
        role,
        rank,
        uuid: store.state.editPlayer,
      });
    };

    return { player, closeModal, saveChanges, updateRank };
  },
});
</script>

<style lang="scss" scoped>
.EditPlayer {
  &-Block:not(:last-child) {
    border-bottom: 1px solid black;
    padding-bottom: 1em;
    margin: 1em 0;
  }
}
</style>
