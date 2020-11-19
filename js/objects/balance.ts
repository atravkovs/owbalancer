import { Teams } from '@/objects/team';

export type BalanceRole = {
  [roleName in 'Tank' | 'Dps' | 'Support']: [number, boolean, boolean];
};

export type Leftover = {
  uuid: string;
  name: string;
  roles: BalanceRole[];
};

export type Balance = {
  anchors: number;
  dispersion: number;
  leftovers: Leftover[];
  teams: Teams;
};

export type BalancerOptions = {
  range: number;
  triesCount: number;
  adjustSr: {
    isEnabled: boolean;
    tank: number;
    support: number;
    dps: number;
  };
  lowRankLimiter: boolean;
  dispersionMinimizer: boolean;
  disallowSecondaryRoles: boolean;
};

export type Results = Balance[];
