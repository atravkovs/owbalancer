use crate::players::{Candidate, PlayerPool};
use crate::roles::{Role, RolesFilter, SimpleRole};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Member {
    pub rank: i32,
    pub uuid: String,
    pub name: String,
    pub role: SimpleRole,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub avg_sr: f32,
    pub name: String,
    pub total_sr: i32,
    pub members: Vec<Member>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Teams(Vec<Team>);

impl Member {
    pub fn new(uuid: String, name: String, role: Role) -> Member {
        let (role, rank) = role.decompose();

        Member {
            uuid,
            name,
            role,
            rank,
        }
    }

    pub fn from_primary_player(candidate: &Candidate) -> Member {
        let uuid = candidate.uuid.clone();
        let name = candidate.name.clone();
        let role = candidate.roles.get_primary().clone();

        return Member::new(uuid, name, role);
    }

    pub fn has_same_role(&self, player: &Candidate) -> bool {
        let primary_role = player.roles.get_primary();

        primary_role.is_same(&self.role)
    }

    fn is_dps(&self) -> bool {
        self.role == SimpleRole::Dps
    }
}

impl Team {
    pub fn new(name: String, player: Member) -> Team {
        let mut members = Vec::default();
        members.push(player);

        Team {
            name,
            members,
            avg_sr: 0.0,
            total_sr: 0,
        }
    }

    pub fn get_captain(&self) -> &Member {
        self.members.first().unwrap()
    }

    pub fn get_squire(&self) -> &Member {
        self.members.get(1).unwrap()
    }

    pub fn get_leutenant(&self) -> &Member {
        self.members.get(2).unwrap()
    }

    pub fn add_primary_player(&mut self, candidate: &Candidate) {
        self.members.push(Member::from_primary_player(candidate));
    }

    pub fn add_player(&mut self, candidate: &Candidate, target_role: &Role) {
        self.members.push(Member::new(
            candidate.uuid.clone(),
            candidate.name.clone(),
            target_role.clone(),
        ));
    }

    pub fn dps_count(&self) -> usize {
        self.count_role(&SimpleRole::Dps)
    }

    pub fn support_count(&self) -> usize {
        self.count_role(&SimpleRole::Support)
    }

    pub fn tank_count(&self) -> usize {
        self.count_role(&SimpleRole::Tank)
    }

    pub fn update(&mut self) -> (f32, i32) {
        let total_sr = self.total_sr();
        let avg_sr = total_sr as f32 / self.members_count() as f32;

        self.avg_sr = avg_sr;
        self.total_sr = total_sr;

        (avg_sr, total_sr)
    }

    pub fn members_count(&self) -> usize {
        self.members.len()
    }

    fn total_sr(&self) -> i32 {
        self.members.iter().map(|member| member.rank).sum()
    }

    fn count_role(&self, role: &SimpleRole) -> usize {
        self.members
            .iter()
            .filter(|&member| member.role == *role)
            .count()
    }
}

impl Teams {
    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    pub fn update(&mut self) {
        for team in &mut self.0 {
            team.update();
        }
    }

    pub fn get_stats(&self) -> (i32, usize) {
        let mut total_sr = 0;
        let mut total_count = 0;

        for team in &self.0 {
            total_sr += team.total_sr;
            total_count += team.members_count();
        }

        (total_sr, total_count)
    }

    pub fn sort(&mut self) {
        self.0
            .sort_by(|a, b| a.avg_sr.partial_cmp(&b.avg_sr).unwrap())
    }

    pub fn filter_dps_captains(&mut self) -> Vec<&mut Team> {
        self.0
            .iter_mut()
            .filter(|team| team.get_captain().is_dps())
            .collect()
    }

    pub fn distribute_squires_to_dps(&mut self, squires: &mut PlayerPool) {
        let roles_filter = RolesFilter(vec![SimpleRole::Tank, SimpleRole::Dps]);

        let dps_teams = self.filter_dps_captains();
        let clonned = squires.clone();
        let mut worthy_squires = clonned.filter_by_roles(roles_filter);
        let mut cache_squires = Vec::default();

        for team in dps_teams {
            if worthy_squires.is_empty() {
                return;
            }

            let candidate = worthy_squires.pop().unwrap();
            cache_squires.push(candidate);
            team.add_primary_player(candidate);
        }

        squires.dispose_of(cache_squires);
    }

    pub fn distribute_squires(&mut self, squires: &mut PlayerPool) {
        for team in &mut self.0 {
            if team.members_count() < 2 {
                team.add_primary_player(&squires.0.pop().unwrap());
            }
        }
    }

    pub fn distribute_leutenants(&mut self, pool: &mut PlayerPool) {
        let mut offset = 0;
        for _ in 0..self.teams_count() {
            offset = pool.distribute_leutenant(self, offset);
        }
    }

    pub fn distribute_ensigns(&mut self, pool: &mut PlayerPool) {
        let mut offset = 0;
        for _ in 0..self.teams_count() {
            offset = pool.distribute_ensign(self, offset);
        }
    }

    pub fn find_mate(
        &mut self,
        candidate: &Candidate,
        max_member_count: usize,
    ) -> Option<&mut Team> {
        self.0.iter_mut().find(|team| {
            team.members_count() <= max_member_count
                && candidate.get_primary_role().fits_team(team)
                && !team.get_captain().has_same_role(candidate)
        })
    }

    pub fn find_perfect_ensign(&mut self, candidate: &Candidate) -> Option<&mut Team> {
        self.0.iter_mut().find(|team| {
            team.members_count() <= 3
                && !team.get_captain().has_same_role(candidate)
                && !team.get_leutenant().has_same_role(candidate)
        })
    }

    pub fn find_team(&mut self, max_size: usize, target_role: &Role) -> Option<&mut Team> {
        self.0
            .iter_mut()
            .find(|team| team.members_count() <= max_size && target_role.fits_team(team))
    }

    pub fn fit_player(
        &mut self,
        player_sr: i32,
        new_average: f32,
        tolerance: u32,
        target_role: &Role,
    ) -> Option<&mut Team> {
        self.0.iter_mut().find(|team| {
            let team_size = team.members_count();
            let new_sr = (team.total_sr + player_sr) as f32 / (team_size + 1) as f32;

            (team_size + 1) <= 6
                && target_role.fits_team(team)
                && ((new_sr - new_average).abs().floor() as u32) <= tolerance
        })
    }

    fn teams_count(&self) -> usize {
        self.0.len()
    }
}

impl From<PlayerPool> for Teams {
    fn from(player_pool: PlayerPool) -> Self {
        let teams = player_pool
            .0
            .into_iter()
            .map(|player| Team::new(player.name.clone(), Member::from_primary_player(&player)))
            .collect();

        Teams(teams)
    }
}
