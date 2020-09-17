use crate::players::Classes;
use crate::teams::Team;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Tank(i32),
    Dps(i32),
    Support(i32),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SimpleRole {
    Tank,
    Dps,
    Support,
}

pub struct RolePriority {
    pub role: Role,
    pub priority: i16,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Roles(pub Vec<Role>);

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RolesFilter(pub Vec<SimpleRole>);

impl RolePriority {
    pub fn new(role: Role, priority: i16) -> RolePriority {
        RolePriority { role, priority }
    }
}

impl Role {
    pub fn is_same(&self, role: &SimpleRole) -> bool {
        match self {
            Role::Dps(_) => *role == SimpleRole::Dps,
            Role::Tank(_) => *role == SimpleRole::Tank,
            Role::Support(_) => *role == SimpleRole::Support,
        }
    }

    pub fn decompose(&self) -> (SimpleRole, i32) {
        match self {
            Role::Dps(rank) => (SimpleRole::Dps, *rank),
            Role::Support(rank) => (SimpleRole::Support, *rank),
            Role::Tank(rank) => (SimpleRole::Tank, *rank),
        }
    }

    pub fn fits_team(&self, team: &Team) -> bool {
        match self {
            Role::Dps(_) => (team.dps_count() + 1) <= 2,
            Role::Support(_) => (team.support_count() + 1) <= 2,
            Role::Tank(_) => (team.tank_count() + 1) <= 2,
        }
    }

    pub fn is_in_range(&self, range: (i32, i32)) -> bool {
        let rank = self.decompose().1;
        rank >= range.0 && rank <= range.1
    }
}

impl Roles {
    pub fn get_primary(&self) -> &Role {
        if let Some(role) = self.0.first() {
            role
        } else {
            &Role::Dps(0)
        }
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn get_primary_rank(&self) -> i32 {
        self.get_primary().decompose().1
    }

    pub fn get(&self, index: usize) -> &Role {
        self.0.get(index).unwrap()
    }

    pub fn get_by_simple(&self, simple: &SimpleRole) -> Option<&Role> {
        self.0
            .iter()
            .find(|&role| role.decompose().0 == simple.clone())
    }
}

impl From<&Classes> for Roles {
    fn from(classes: &Classes) -> Self {
        let mut temp_roles: Vec<RolePriority> = Vec::new();

        if classes.dps.is_active {
            temp_roles.push(RolePriority::new(
                Role::Dps(classes.dps.rank),
                classes.dps.priority,
            ));
        }

        if classes.support.is_active {
            temp_roles.push(RolePriority::new(
                Role::Support(classes.support.rank),
                classes.support.priority,
            ));
        }

        if classes.tank.is_active {
            temp_roles.push(RolePriority::new(
                Role::Tank(classes.tank.rank),
                classes.tank.priority,
            ));
        }

        temp_roles.sort_by(|a, b| a.priority.cmp(&b.priority));

        let mut roles = Vec::default();

        for role in temp_roles {
            roles.push(role.role);
        }

        Roles(roles)
    }
}

impl RolesFilter {
    pub fn has_same(&self, role: &Role) -> bool {
        self.0
            .iter()
            .find(|&simple_role| role.is_same(simple_role))
            .is_some()
    }
}
