<template>
  <Modal
    title="Balance"
    variant="large"
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
    <div class="mb-3" v-if="balanceType === 'full'">
      <label class="form-label w-100">Disable</label>
      <div class="btn-group">
        <input type="radio" value="none" id="disable1" class="btn-check" v-model="disableType" />
        <label class="btn btn-primary" for="disable1">None</label>
        <input type="radio" value="ex_caps" id="disable2" class="btn-check" v-model="disableType" />
        <label class="btn btn-primary" for="disable2">Except captains</label>
        <input
          type="radio"
          value="leut_ens"
          id="disable3"
          class="btn-check"
          v-model="disableType"
        />
        <label class="btn btn-primary" for="disable3">Leutenants and Ensigns</label>
        <input type="radio" value="ens" id="disable4" class="btn-check" v-model="disableType" />
        <label class="btn btn-primary" for="disable4">Ensigns</label>
      </div>
    </div>
    <div class="mb-3">
      <label class="form-label">Options</label>
      <div class="form-check">
        <input
          type="checkbox"
          id="lowRankLimiter"
          class="form-check-input"
          v-model="bOptions.lowRankLimiter"
        />
        <label for="lowRankLimiter" class="form-check-label">Low rank limiter</label>
      </div>
      <div class="form-check">
        <input
          type="checkbox"
          id="disallowSecondaryRoles"
          class="form-check-input"
          v-model="bOptions.disallowSecondaryRoles"
        />
        <label for="disallowSecondaryRoles" class="form-check-label"
          >Disallow duplicate secondary roles</label
        >
      </div>
      <div class="form-check">
        <input
          type="checkbox"
          id="dispersionMiminizer"
          class="form-check-input"
          v-model="bOptions.dispersionMiminizer"
        />
        <label for="dispersionMiminizer" class="form-check-label"
          >Enable dispersion minimizer</label
        >
      </div>
    </div>
    <div class="mb-3">
      <div class="form-check">
        <input
          type="checkbox"
          id="srScaling"
          class="form-check-input"
          v-model="bOptions.adjustSr.isEnabled"
        />
        <label for="srScaling" class="form-check-label">Adjust player SR by main class</label>
      </div>
      <div class="d-flex justify-content-between">
        <div class="input-group mr-2">
          <input
            type="number"
            min="0"
            class="form-control"
            v-model.number="bOptions.adjustSr.tank"
          />
          <span class="input-group-text">
            <role-icon rtype="tank" />
          </span>
          <span class="input-group-text">%</span>
        </div>
        <div class="input-group mr-2">
          <input
            type="number"
            min="0"
            class="form-control"
            v-model.number="bOptions.adjustSr.dps"
          />
          <span class="input-group-text">
            <role-icon rtype="dps" />
          </span>
          <span class="input-group-text">%</span>
        </div>
        <div class="input-group">
          <input
            type="number"
            min="0"
            class="form-control"
            v-model.number="bOptions.adjustSr.support"
          />
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
        <b>±{{ bOptions.range }}</b>
      </label>
      <input
        id="dispersionRange"
        type="range"
        class="form-range"
        min="0"
        max="120"
        step="1"
        v-model.number="bOptions.range"
      />
    </div>
    <div class="mb-3">
      <label for="triesRange" class="form-label">
        Tries count:
        <b>{{ bOptions.triesCount }}</b>
      </label>
      <input
        id="triesRange"
        type="range"
        class="form-range"
        min="0"
        max="2500"
        step="1"
        v-model.number="bOptions.triesCount"
      />
    </div>
    <div class="mb-3">
      <div class="progress">
        <div
          class="progress-bar progress-bar-striped"
          :class="{
            'progress-bar-animated':
              progress.current / (progress.total * bOptions.triesCount) !== 1,
          }"
          role="progressbar"
          :aria-valuenow="
            Math.floor((progress.current / (progress.total * bOptions.triesCount)) * 100)
          "
          aria-valuemin="0"
          aria-valuemax="100"
          :style="
            `width: ${Math.floor(
              (progress.current / (progress.total * bOptions.triesCount)) * 100
            )}%`
          "
        ></div>
      </div>
    </div>
  </Modal>
</template>

<script lang="ts">
/* eslint-disable */
import { computed, defineComponent, reactive, ref } from 'vue';
import MutationTypes from '@/store/mutation-types';
import { useStore } from '@/store';
import wasm from '@/mworker';

import Modal from '@/components/Helpers/Modal.vue';
import RoleIcon from '@/components/svg/RoleIcon.vue';
import player from '@/objects/player';

export default defineComponent({
  name: 'Balance',
  components: { Modal, RoleIcon },
  setup() {
    const store = useStore();

    const sbOptions = computed(() => store.state.balancerOptions);
    const bOptions = ref(sbOptions);

    const balanceType = ref('full');
    const disableType = ref('none');

    const isActive = computed(() => store.state.isBalance);
    const reservedPlayers = computed(() => store.state.reservedPlayers);
    const stateTeams = computed(() => store.state.teams);
    const progress = reactive({ total: 10, current: 0 });

    document.addEventListener('wasm-update', e => {
      // eslint-disable-next-line
      console.log('Step: ', e.detail.message());
      progress.current += 1;
    });

    const closeModal = () => {
      store.commit(MutationTypes.TOGGLE_BALANCE);
    };

    const fullBalance: (lib: any) => any = lib => {
      const data = JSON.stringify({
        players: store.state.players,
        range: +bOptions.value.range,
        lowRankLimiter: bOptions.value.lowRankLimiter,
        disallowSecondaryRoles: bOptions.value.disallowSecondaryRoles,
        adjustSr: bOptions.value.adjustSr,
        disableType: disableType.value,
        dispersionMiminizer: bOptions.value.dispersionMiminizer,
        triesCount: bOptions.value.triesCount,
      });

      return lib.fullBalance(data);
    };

    const halfBalance: (lib: any) => any = lib => {
      return lib.halfBalance(
        JSON.stringify({
          players: store.state.players,
          range: +bOptions.value.range,
          lowRankLimiter: bOptions.value.lowRankLimiter,
          disallowSecondaryRoles: bOptions.value.disallowSecondaryRoles,
          adjustSr: bOptions.value.adjustSr,
        })
      );
    };

    const finalBalance: (lib: any, data: any) => any = (lib, { teamsCopy, reserveCopy }) => {
      return lib.finalBalance(
        JSON.stringify({
          players: store.state.players,
          range: +bOptions.value.range,
          lowRankLimiter: bOptions.value.lowRankLimiter,
          disallowSecondaryRoles: bOptions.value.disallowSecondaryRoles,
          reserveCopy: reserveCopy,
          teamsCopy: teamsCopy,
          adjustSr: bOptions.value.adjustSr,
        })
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

    const checkCaps: () => boolean = () => {
      return Object.values(store.state.players).some(player => player.identity.isCaptain);
    };

    const checkSquires: () => boolean = () => {
      const caps = Object.values(store.state.players).filter(player => player.identity.isCaptain);
      const squires = Object.values(store.state.players).filter(player => player.identity.isSquire);

      return squires.length >= caps.length;
    };

    const balance = async () => {
      if (!checkCaps()) {
        alert('Please select at least one captain');
        return;
      }

      if (!checkSquires()) {
        alert('Please make sure that every captain has at least one squire');
        return;
      }

      const teamsCopy = [...stateTeams.value];
      const reserveCopy = [...reservedPlayers.value];

      store.commit(MutationTypes.CLEAR_TEAMS);
      progress.current = 0;

      const lib = await wasm;

      try {
        const results = await conditionalBalance(lib, {
          teamsCopy,
          reserveCopy,
        });

        console.log('Results', results);

        if (results.length != 1) {
          store.commit(MutationTypes.SET_RESULTS, results);
          store.commit(MutationTypes.TOGGLE_SELECTION);
          return;
        }

        const [{ leftovers, teams }] = results;

        const ignoredUuids = leftovers.reduce((acc, leftover) => {
          acc.push(leftover.uuid);
          return acc;
        }, []);

        store.commit(MutationTypes.RESERVE_PLAYERS, ignoredUuids);
        store.commit(MutationTypes.ADD_TEAMS, teams);
      } catch (e) {
        console.error(e.message);
      }
    };

    return {
      balance,
      isActive,
      progress,
      bOptions,
      closeModal,
      balanceType,
      disableType,
    };
  },
});
</script>

<style lang="scss" scoped>
#dispersionRange {
  &::-webkit-slider-runnable-track {
    background: linear-gradient(to right, #33ccff 0%, #ff0000 100%);
  }
}
</style>
