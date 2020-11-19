import { Teams } from '@/objects/team';

export type BalanceRole = {
  [roleName in 'Tank' | 'Dps' | 'Support']: [Number, Boolean, Boolean];
};

export type Leftover = {
  uuid: string;
  name: string;
  roles: BalanceRole[];
};

export type Balance = {
  anchors: Number;
  dispersion: Number;
  leftovers: Leftover[];
  teams: Teams;
};

export type Results = Balance[];
