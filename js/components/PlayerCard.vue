<template>
  <div
    class="d-flex justify-content-between"
    draggable="true"
    @contextmenu="editPlayer"
    @dragstart="drag"
  >
    <div class="d-flex">
      <div class="w-40p text-center font-smaller">
        <div>
          <rank-icon :rank="sr" />
        </div>
        <div>{{ sr }}</div>
      </div>
      <div class="lh-100">
        {{ player.identity.name }}
        &nbsp;
        <span class="extra-icon">
          <CrownIcon v-if="player.identity.isCaptain" />
          <SwordIcon v-if="player.identity.isSquire" />
        </span>
      </div>
    </div>
    <div class="role-icons">
      <RoleIcon v-for="role in state.icons" :rtype="role" :key="role" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType, reactive, computed } from 'vue';

import { useStore } from '@/store';
import { Player } from '@/objects/player';
import MutationTypes from '@/store/mutation-types';

import RoleIcon from '@/components/svg/RoleIcon.vue';
import RankIcon from '@/components/svg/RankIcon.vue';
import CrownIcon from '@/components/svg/CrownIcon.vue';
import SwordIcon from '@/components/svg/SwordIcon.vue';

export default defineComponent({
  name: 'PlayerCard',
  props: {
    player: Object as PropType<Player>,
  },
  components: { RoleIcon, RankIcon, CrownIcon, SwordIcon },
  setup(props) {
    const store = useStore();

    const drag = (ev: DragEvent) => {
      const _ = ev?.dataTransfer?.setData(
        'playerTag',
        props.player?.identity.uuid || ''
      );

      return _;
    };

    const editPlayer = (e: MouseEvent) => {
      e.preventDefault();

      if (props.player === undefined) {
        return;
      }

      store.commit(MutationTypes.EDIT_PLAYER, props.player.identity.uuid);
    };

    const icons = computed(() =>
      Object.entries(props.player?.stats.classes || {})
        .filter(([, role]) => role.isActive)
        .sort(([, role], [, role2]) => role2.priority - role.priority)
        .reduce((acc: string[], [name]) => [...acc, name], [])
    );

    const state = reactive({
      icons,
    });

    const sr = computed(
      () =>
        Object.values(props.player?.stats.classes || {})
          .filter((role) => role.isActive)
          .sort((a, b) => a.priority - b.priority)[0].rank
    );

    return {
      sr,
      drag,
      state,
      editPlayer,
    };
  },
});
</script>

<style lang="scss" scoped>
.lh-100 {
  line-height: 50px;
}
.w-40p {
  width: 40px;
}
.font-smaller {
  font-size: 0.9rem;
}
.role-icons {
  line-height: 1;
}
.role-icons > * {
  font-size: 0.8rem;
  color: var(--bs-gray);
}
.role-icons > *:last-child {
  font-size: 1rem;
  color: var(--bs-gray-dark);
}
.extra-icon {
  color: var(--bs-info);
}
</style>
