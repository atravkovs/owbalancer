import { Teams } from '@/objects/team';
import { Points } from '@/objects/bezier';

export type BalanceRole = {
  [roleName in 'Tank' | 'Dps' | 'Support']: [number, boolean, boolean];
};

export type Leftover = {
  uuid: string;
  name: string;
  roles: BalanceRole[];
};

export type RolePoints = {
  any: Points;
  primary: Points;
  secondary: Points;
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
    tank: RolePoints;
    support: RolePoints;
    dps: RolePoints;
  };
  lowRankLimiter: boolean;
  dispersionMinimizer: boolean;
  disallowSecondaryRoles: boolean;
};

export type Results = Balance[];
