<template>
  <Modal
    title="Balance"
    :isActive="isActive"
    customAction="Balance"
    @close-modal="closeModal"
    @save-changes="balance"
  >
    <div class="mb-3 mt-1">
      <label class="form-label w-100">Balance Type</label>
      <div class="btn-group">
        <input type="radio" value="full" id="balance1" class="btn-check" v-model="balanceType" />
        <label class="btn btn-primary" for="balance1">Full</label>
        <input type="radio" value="half" id="balance2" class="btn-check" v-model="balanceType" />
        <label class="btn btn-primary" for="balance2">Half</label>
        <input type="radio" value="final" id="balance3" class="btn-check" v-model="balanceType" />
        <label class="btn btn-primary" for="balance3">Final</label>
      </div>
    </div>
    <div class="mb-3">
      <label class="form-label">Options</label>
      <div class="form-check">
        <input
          type="checkbox"
          id="lowRankLimiter"
          class="form-check-input"
          v-model="lowRankLimiter"
        />
        <label for="lowRankLimiter" class="form-check-label">Low rank limiter</label>
      </div>
      <div class="form-check">
        <input
          type="checkbox"
          id="disallowSecondaryRoles"
          class="form-check-input"
          v-model="disallowSecondaryRoles"
        />
        <label
          for="disallowSecondaryRoles"
          class="form-check-label"
        >Disallow duplicate secondary roles</label>
      </div>
    </div>
    <div class="mb-3">
      <div class="form-check">
        <input type="checkbox" id="srScaling" class="form-check-input" v-model="adjustSr.isEnabled" />
        <label for="srScaling" class="form-check-label">Adjust player SR by main class</label>
      </div>
      <div class="d-flex justify-content-between">
        <div class="input-group mr-2">
          <input type="number" min="0" class="form-control" v-model="adjustSr.tank" />
          <span class="input-group-text">
            <role-icon rtype="tank" />
          </span>
          <span class="input-group-text">%</span>
        </div>
        <div class="input-group mr-2">
          <input type="number" min="0" class="form-control" v-model="adjustSr.dps" />
          <span class="input-group-text">
            <role-icon rtype="dps" />
          </span>
          <span class="input-group-text">%</span>
        </div>
        <div class="input-group">
          <input type="number" min="0" class="form-control" v-model="adjustSr.support" />
          <span class="input-group-text">
            <role-icon rtype="support" />
          </span>
          <span class="input-group-text">%</span>
        </div>
      </div>
    </div>
    <div class="mb-3">
      <label for="dispersionRange" class="form-label">
        Dispersion:
        <b>±{{ range }}</b>
      </label>
      <input
        id="dispersionRange"
        type="range"
        class="form-range"
        min="0"
        max="120"
        step="1"
        v-model="range"
      />
    </div>
  </Modal>
</template>

<script lang="ts">
/* eslint-disable */
import { computed, defineComponent, ref } from 'vue';
import MutationTypes from '@/store/mutation-types';
import { useStore } from '@/store';

import Modal from '@/components/Helpers/Modal.vue';
import RoleIcon from '@/components/svg/RoleIcon.vue';

const wasm = import('@/../pkg/index.js');

export default defineComponent({
  name: 'Balance',
  components: { Modal, RoleIcon },
  setup() {
    const range = ref(30);
    const adjustSr = ref({
      isEnabled: false,
      tank: 100,
      support: 90,
      dps: 110,
    });
    const balanceType = ref('full');
    const lowRankLimiter = ref(false);
    const disallowSecondaryRoles = ref(false);

    const store = useStore();
    const isActive = computed(() => store.state.isBalance);
    const reservedPlayers = computed(() => store.state.reservedPlayers);
    const stateTeams = computed(() => store.state.teams);

    const closeModal = () => {
      store.commit(MutationTypes.TOGGLE_BALANCE);
    };

    const fullBalance: (lib: any) => any = (lib) => {
      return lib.balance(
        store.state.players,
        +range.value,
        lowRankLimiter.value,
        disallowSecondaryRoles.value,
        adjustSr.value
      );
    };

    const halfBalance: (lib: any) => any = (lib) => {
      return lib.balance_half(
        store.state.players,
        +range.value,
        lowRankLimiter.value,
        disallowSecondaryRoles.value,
        adjustSr.value
      );
    };

    const finalBalance: (lib: any, data: any) => any = (
      lib,
      { teamsCopy, reserveCopy }
    ) => {
      return lib.balance_final(
        store.state.players,
        +range.value,
        lowRankLimiter.value,
        disallowSecondaryRoles.value,
        reserveCopy,
        teamsCopy,
        adjustSr.value
      );
    };

    const conditionalBalance: (lib: any, data: any) => any = (lib, data) => {
      if (balanceType.value === 'half') {
        return halfBalance(lib);
      }

      if (balanceType.value === 'final') {
        return finalBalance(lib, data);
      }

      return fullBalance(lib);
    };

    const balance = () => {
      const teamsCopy = [...stateTeams.value];
      const reserveCopy = [...reservedPlayers.value];

      store.commit(MutationTypes.CLEAR_TEAMS);

      wasm
        .then((lib) => {
          try {
            const { leftovers, teams } = conditionalBalance(lib, {
              teamsCopy,
              reserveCopy,
            });

            const ignoredUuids = leftovers.reduce((acc, leftover) => {
              acc.push(leftover.uuid);
              return acc;
            }, []);

            store.commit(MutationTypes.RESERVE_PLAYERS, ignoredUuids);
            store.commit(MutationTypes.ADD_TEAMS, teams);
          } catch (e) {
            console.error(e.message);
          }
        })
        .catch((e) => {
          console.error(e.message);
        });
    };

    return {
      range,
      balance,
      adjustSr,
      isActive,
      closeModal,
      balanceType,
      lowRankLimiter,
      disallowSecondaryRoles,
    };
  },
});
</script>

<style lang="scss" scoped>
.form-range {
  &::-webkit-slider-runnable-track {
    background: linear-gradient(to right, #33ccff 0%, #ff0000 100%);
  }
}
</style>
